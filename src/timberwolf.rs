use petgraph::{graph::{self, NodeIndex}, Directed};
use std::{cmp::{max, min}, collections::HashMap, fs::File, io::Write, path::Path};
use rand::{Rng, seq::IndexedRandom};

use crate::{mcfunction, read_blif};

// Schedule from page 434 of the Timberwolf paper: https://cs.baylor.edu/~maurer/CSI5346/timberwolf.pdf
const TEMPERATURE_START: f32 = 4000000.0;
const SCHEDULE_TEMPS: [f32; 10] = [40000.0, 20000.0, 10000.0, 5000.0, 200.0, 100.0, 50.0, 5.0, 1.5, 0.0];
const SCHEDULE_ALPHAS: [f32; 10] = [0.8, 0.84, 0.88, 0.91, 0.94, 0.9, 0.85, 0.8, 0.7, 0.1];

const COORD_MIN: i32 = 0;
const COORD_MAX: i32 = 50;

const GATE_PADDING: i32 = 2;

const MOVES_PER_SWAP: i32 = 3;
const PERTUB_ATTEMPTS_PER_GATE: i32 = 50;

const OVERLAP_COST_WEIGHT: f32 = 2.0;
const PADDING_COST_WEIGHT: f32 = 1.0;
const BOUND_COST_WEIGHT: f32 = 2.0;
const TEI_COST_WEIGHT: f32 = 1.5;

const LOG_PATH: &'static str = "res/logs/timberwolf.log";

#[allow(non_snake_case)]
pub mod LoggingRules {
    pub const TO_FILE: u8 = 0x01;
    pub const TO_STDOUT: u8 = 0x02;
    pub const ON_ACCEPT: u8 = 0x04;
    pub const ON_REJECT: u8 = 0x08;
    pub const ALWAYS: u8 = 0x0C;
}

pub fn anneal(
    circuit_graph: &graph::Graph<read_blif::Node, String, Directed>,
    gate_info: &HashMap<String, mcfunction::GateInfo>,
    logging_rules: u8
) -> HashMap<NodeIndex, mcfunction::Gate> {
    let mut current_state = gen_random_state(circuit_graph);
    let mut candidate_state = current_state.clone();
    let idxs: Vec<NodeIndex> = current_state.keys().map(|k| *k).collect();
    let mut temperature = TEMPERATURE_START;

    let path: &Path = Path::new(LOG_PATH);
    let mut log_file = File::create(path).unwrap();
    let file_error: &str = &format!("Failed to write entry to Timberwolf log file at {LOG_PATH}");

    println!("=== TIMBERWOLF ===");

    while temperature >= 0.1 {
        for _ in 0..PERTUB_ATTEMPTS_PER_GATE {
            // Find a random neighboring state
            perturb(&mut candidate_state, &idxs);

            // delta_cost < 0 -> new state is better
            let current_cost = cost(&current_state, circuit_graph, gate_info);
            let candidate_cost = cost(&candidate_state, circuit_graph, gate_info);
            let delta_cost = candidate_cost - current_cost;

            // Decide whether to accept the new state
            let prob = accept_prob(delta_cost, temperature);
            let accepted = rand::random_range(0f32..=1f32) < prob;
            if accepted {
                current_state = candidate_state.clone();
            }

            if (accepted && logging_rules & LoggingRules::ON_ACCEPT > 0) || logging_rules & LoggingRules::ON_REJECT > 0 {
                log_anneal_step(temperature, current_cost, delta_cost, candidate_cost, prob, accepted, logging_rules, &mut log_file, &file_error);
            }
        }

        // Temperature tends to 0 across annealing
        temperature = temperature_multiplier(temperature) * temperature;
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
    match rand::rng().random_range(0..=MOVES_PER_SWAP) {
        0 => {
            // Swap
            let mut idx_iterator = idxs.choose_multiple(&mut rand::rng(), 2);
            let (idx1, idx2) = (idx_iterator.next().unwrap(), idx_iterator.next().unwrap());
            let [gate1, gate2] = state.get_disjoint_mut([idx1, idx2]).map(|x| x.unwrap());

            (gate1.x, gate2.x) = (gate2.x, gate1.x);
            (gate1.z, gate2.z) = (gate2.z, gate1.z);
        },
        _ => {
            // Move
            let random_idx = idxs.choose(&mut rand::rng()).unwrap();
            let gate = state.get_mut(random_idx).unwrap();
            gate.x += rand::rng().random_range(-5..=5);
            gate.z += rand::rng().random_range(-5..=5);
        }
        // TODO: rotate/flip gates?
    }
}

fn temperature_multiplier(current_temperature: f32) -> f32 {
    for i in 0..SCHEDULE_TEMPS.len() {
        if current_temperature >= SCHEDULE_TEMPS[i] {
            return SCHEDULE_ALPHAS[i];
        }
    }

    return SCHEDULE_ALPHAS[SCHEDULE_ALPHAS.len() - 1];
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
                    // Start in closest 3rd of the available area
                    x: rand::rng().random_range(COORD_MIN..=COORD_MAX / 3),
                    z: rand::rng().random_range(COORD_MIN..=COORD_MAX / 3),
                });
            },
            _ => ()  // Placement states only include gates
        }
    }

    return gates;
}

fn log_anneal_step(
    temperature: f32,
    current_cost: f32,
    delta_cost: f32,
    candidate_cost: f32,
    probability: f32,
    accepted: bool,
    logging_rules: u8,
    log_file: &mut File,
    file_error: &str
) {
    let temperature_s = temperature.to_string();
    let current_cost_s = current_cost.to_string();
    let delta_cost_s = delta_cost.to_string();
    let candidate_cost_s = candidate_cost.to_string();
    let probability_s = probability.to_string();

    let entry = format!(
        "Annealed at T={:9} @ Δcost = {:7} = {:7} - {:7} w/accept prob={:3} ({})",
        &temperature_s[..9.min(temperature_s.len())],
        &delta_cost_s[..7.min(delta_cost_s.len())],
        &current_cost_s[..7.min(current_cost_s.len())],
        &candidate_cost_s[..7.min(candidate_cost_s.len())],
        &probability_s[..3.min(probability_s.len())],
        if accepted { "ACCEPTED" } else { "DENIED" }
    );

    if logging_rules & LoggingRules::TO_STDOUT > 0 {
        println!("{entry}");
    }
    if logging_rules & LoggingRules::TO_FILE > 0 {
        writeln!(log_file, "{entry}").expect(file_error);
    }
}
