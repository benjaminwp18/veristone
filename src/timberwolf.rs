use petgraph::{graph::{self, NodeIndex}, Directed};
use std::{collections::HashMap, vec};
use rand::prelude::*;

#[path = "./bin/mcfunction.rs"]
mod mcfunction;

#[path = "./bin/read_blif.rs"]
mod read_blif;

const K_MAX: i32 = 200;

const TEMP_MAX: f32 = 10000000.0;
const TEMP_MID1: f32 = 100000.0;
const TEMP_MID2: f32 = 100.0;
const TEMP_MIN: f32 = 0.001;

const ALPHA_START: f32 = 0.80;
const ALPHA_MID: f32 = 0.95;
const ALPHA_END: f32 = 0.80;

fn anneal(
    circuit_graph: &graph::Graph<read_blif::Node, String, Directed>,
    gate_info: &HashMap<String, mcfunction::GateInfo>,
) -> HashMap<NodeIndex, mcfunction::Gate> {
    let mut state = gen_random_state(circuit_graph);
    let mut temperature = TEMP_MAX;

    for k in 0..K_MAX {
        // let temperature = get_temperature(1 - (k + 1) / K_MAX);  // Temperature tends to 0 across annealing
        temperature = temperature_multiplier(temperature) * temperature;
        println!("Annealing iteration {k} at temperature {temperature}");
        let new_state = perturb(state);  // Find a random neighboring state
        let delta_cost = cost(new_state) - cost(state);  // delta_cost < 0 -> new state is better
        if rand::random_range(0f32..=1f32) < accept_prob(delta_cost, temperature) {  // Decide whether to accept the new state
            state = new_state;
        }
    }

    return state;
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
    let mut state: HashMap<NodeIndex, mcfunction::Gate> = HashMap::new();

    for node_idx in circuit_graph.node_indices() {
        let node_weight = circuit_graph.node_weight(node_idx).unwrap();
        if node_weight.node_type == read_blif::NodeType::Gate {
            state.insert(node_idx, mcfunction::Gate { name: node_weight.name.clone(), x: rand::random(), z: rand::random() });
        }
    }

    return state;
}

// function accept_prob(delta_cost, T) {
//     // Probability to accept new state tends to 0 as delta_cost increases
//     // Higher temperature T makes it go to 0 slower, so higher cost states are more likely to be accepted at higher temperatures
//     return min(1, exp(-delta_cost / T))
// }