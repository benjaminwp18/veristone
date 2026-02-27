use petgraph::{graph::{self, NodeIndex}, Directed};
use std::{cmp::max, collections::HashMap, cmp::min};
use rand::{Rng, seq::IndexedRandom};

use crate::{mcfunction, read_blif};

const K_MAX: i32 = 200;

const TEMP_MAX: f32 = 10000000.0;
const TEMP_MID1: f32 = 100000.0;
const TEMP_MID2: f32 = 100.0;
const TEMP_MIN: f32 = 0.001;

const ALPHA_START: f32 = 0.80;
const ALPHA_MID: f32 = 0.95;
const ALPHA_END: f32 = 0.80;

const COORD_MIN: i32 = 0;
const COORD_MAX: i32 = 100;

const GATE_PADDING: i32 = 2;

const OVERLAP_COST_WEIGHT: f32 = 2.0;
const PADDING_COST_WEIGHT: f32 = 1.0;
const BOUND_COST_WEIGHT: f32 = 1.0;
const TEI_COST_WEIGHT: f32 = 1.0;

pub fn anneal(
    circuit_graph: &graph::Graph<read_blif::Node, String, Directed>,
    gate_info: &HashMap<String, mcfunction::GateInfo>
) -> HashMap<NodeIndex, mcfunction::Gate> {
    let mut current_state = gen_random_state(circuit_graph);
    let mut candidate_state = current_state.clone();
    let idxs: Vec<NodeIndex> = current_state.keys().map(|k| *k).collect();
    let mut temperature = TEMP_MAX;

    println!("=== TIMBERWOLF ===");

    for k in 0..K_MAX {
        // let temperature = get_temperature(1 - (k + 1) / K_MAX);  // Temperature tends to 0 across annealing
        temperature = temperature_multiplier(temperature) * temperature;
        println!("Annealing iteration {k} at temperature {temperature}");
        perturb(&mut candidate_state, &idxs);  // Find a random neighboring state
        let delta_cost = cost(&candidate_state, circuit_graph, gate_info) - cost(&current_state, circuit_graph, gate_info);  // delta_cost < 0 -> new state is better
        if rand::random_range(0f32..=1f32) < accept_prob(delta_cost, temperature) {  // Decide whether to accept the new state
            current_state = candidate_state.clone();
        }
    }

    return current_state;
}

fn cost(
    state: &HashMap<NodeIndex, mcfunction::Gate>,
    graph: &graph::Graph<read_blif::Node, String, Directed>,
    gate_info: &HashMap<String, mcfunction::GateInfo>
) -> f32 {
    let mut cost = 0f32;

    // Overlapping gates & gates too close together (within 2 * GATE_PADDING)
    let mut overlap_cost = 0i32;
    let mut padding_cost = 0i32;
    let ordered_gates: Vec<&mcfunction::Gate> = state.values().collect();
    for i in 0..ordered_gates.len() {
        let gate1 = ordered_gates[i];
        let gate_info_1 = gate_info.get(&ordered_gates[i].name).unwrap();

        let gate1_x_end = gate1.x + gate_info_1.x_dim;
        let gate1_z_end = gate1.z + gate_info_1.z_dim;
        for j in i..ordered_gates.len() {
            let gate2 = ordered_gates[j];
            let gate_info_2 = gate_info.get(&ordered_gates[j].name).unwrap();

            let gate2_x_end = gate2.x + gate_info_2.x_dim;
            let gate2_z_end = gate2.z + gate_info_2.z_dim;

            let x_start = max(gate1.x, gate2.x);
            let x_end = min(gate1_x_end, gate2_x_end);
            let x_overlap = x_end - x_start;

            let z_start = max(gate1.z, gate2.z);
            let z_end = min(gate1_z_end, gate2_z_end);
            let z_overlap = z_end - z_start;

            overlap_cost += max(x_overlap, 0) * max(z_overlap, 0);
            padding_cost += max(x_overlap + GATE_PADDING * 2, 0) * max(z_overlap + GATE_PADDING * 2, 0);
        }
    }

    cost += OVERLAP_COST_WEIGHT * overlap_cost as f32;
    cost += PADDING_COST_WEIGHT * padding_cost as f32;

    // Arbitrary circuit boundaries
    let mut bound_cost = 0i32;
    for gate in state.values() {
        let current_gate_info = gate_info.get(&gate.name).unwrap();
        let x_end = gate.x + current_gate_info.x_dim;
        let z_end = gate.z + current_gate_info.z_dim;
        if gate.x < COORD_MIN {
            bound_cost += COORD_MIN - gate.x;
        }
        if x_end > COORD_MAX {
            bound_cost += x_end - COORD_MAX;
        }
        if gate.z < COORD_MIN {
            bound_cost += COORD_MIN - gate.z;
        }
        if z_end > COORD_MAX {
            bound_cost += z_end - COORD_MAX;
        }
    }
    cost += BOUND_COST_WEIGHT * bound_cost as f32;

    // TEIC: Total Estimated Interconnect Cost (summative wire length)
    let wires = read_blif::get_wires(graph, state, gate_info);
    let mut tei_cost = 0;
    for wire in wires {
        tei_cost += (wire.end.x - wire.start.x).abs() + (wire.end.y - wire.start.y).abs() + (wire.end.z - wire.start.z).abs();
    }
    cost += TEI_COST_WEIGHT * tei_cost as f32;

    return cost;
}

fn perturb(state: &mut HashMap<NodeIndex, mcfunction::Gate>, idxs: &Vec<NodeIndex>) {
    match rand::rng().random_range(0..=1) {
        0 => {
            // Move
            let random_idx = idxs.choose(&mut rand::rng()).unwrap();
            let gate = state.get_mut(random_idx).unwrap();
            gate.x += rand::rng().random_range(-3..=3);
            gate.z += rand::rng().random_range(-3..=3);
        },
        _ => {
            // Swap
            let mut idx_iterator = idxs.choose_multiple(&mut rand::rng(), 2);
            let (idx1, idx2) = (idx_iterator.next().unwrap(), idx_iterator.next().unwrap());
            let [gate1, gate2] = state.get_disjoint_mut([idx1, idx2]).map(|x| x.unwrap());

            (gate1.x, gate2.x) = (gate2.x, gate1.x);
            (gate1.z, gate2.z) = (gate2.z, gate1.z);
        }
        // TODO: rotate/flip gates?
    }
}

fn temperature_multiplier(current_temperature: f32) -> f32 {
    // Super basic piecewise multiplier
    // See temperature graph? https://miro.medium.com/v2/resize:fit:720/format:webp/1*FqxMgk1-JHuwIOGGQ8U8sg.jpeg
    if current_temperature > TEMP_MID1 {
        return ALPHA_START;
    }
    else if TEMP_MID1 >= current_temperature && current_temperature > TEMP_MID2 {
        return ALPHA_MID;
    }
    else {
        return ALPHA_END;
    }
}

fn accept_prob(delta_cost: f32, temperature: f32) -> f32 {
    // Probability to accept new state tends to 0 as delta_cost increases
    // Higher temperature T makes it go to 0 slower, so higher cost states are more likely to be accepted at higher temperatures
    return f32::min(1f32, f32::exp(-delta_cost / temperature));
}

fn gen_random_state(circuit_graph: &graph::Graph<read_blif::Node, String, Directed>) -> HashMap<NodeIndex, mcfunction::Gate> {
    let mut gates: HashMap<NodeIndex, mcfunction::Gate> = HashMap::new();

    for node_idx in circuit_graph.node_indices() {
        let node_weight = circuit_graph.node_weight(node_idx).unwrap();
        match node_weight.node_type {
            read_blif::NodeType::Gate => {
                gates.insert(node_idx, mcfunction::Gate {
                    name: node_weight.name.clone(),
                    x: rand::rng().random_range(COORD_MIN..=COORD_MAX),
                    z: rand::rng().random_range(COORD_MIN..=COORD_MAX),
                });
            },
            _ => ()  // Placement states only include gates
        }
    }

    return gates;
}
