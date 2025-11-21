use petgraph::{graph, Directed};
use std::vec;
use rand::prelude::*;

#[path = "./bin/read_blif.rs"]
mod read_blif;

const K_MAX: i32 = 100;

struct GateCoords {
    gate_index: graph::NodeIndex,
    coords: (i32, i32, i32)
}

fn anneal(circuit_graph: graph::Graph<read_blif::Node, String, Directed>) -> Vec<GateCoords> {
    let mut state = gen_random_state(circuit_graph);

    for k in 0..K_MAX {
        let temperature = next_temperature(1 - (k + 1) / K_MAX);  // Temperature tends to 0 across annealing
        let new_state = perturb(state);  // Find a random neighboring state
        let delta_cost = cost(new_state) - cost(state);  // delta_cost < 0 -> new state is better
        if rand::random_range(0f32..=1f32) < accept_prob(delta_cost, temperature) {  // Decide whether to accept the new state
            state = new_state;
        }
    }

    return state;
}

fn accept_prob(delta_cost: f32, temperature: f32) -> f32 {
    // Probability to accept new state tends to 0 as delta_cost increases
    // Higher temperature T makes it go to 0 slower, so higher cost states are more likely to be accepted at higher temperatures
    return f32::min(1f32, f32::exp(-delta_cost / temperature));
}

fn gen_random_state(circuit_graph: graph::Graph<read_blif::Node, String, Directed>) -> Vec<GateCoords> {
    let mut state: Vec<GateCoords> = Vec::new();

    for node_idx in circuit_graph.node_indices() {
        let node_weight = circuit_graph.node_weight(node_idx).unwrap();
        if node_weight.node_type == read_blif::NodeType::Gate {
            state.push(GateCoords { gate_index: node_idx, coords: (rand::random(), rand::random(), rand::random()) });
        }
    }

    return state;
}

// function accept_prob(delta_cost, T) {
//     // Probability to accept new state tends to 0 as delta_cost increases
//     // Higher temperature T makes it go to 0 slower, so higher cost states are more likely to be accepted at higher temperatures
//     return min(1, exp(-delta_cost / T))
// }