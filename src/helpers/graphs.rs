use crate::helpers::helper::{calculate_depth, calculate_similarity, node_degree_total};
use gfa_reader::{Gfa, Pansn};

pub fn get_stats(wrapper: &Pansn<u32, (), ()>, graph: &Gfa<u32, (), ()>, kind: &str) -> Vec<u32> {
    let paths = wrapper.get_path_genome();

    if kind == "depth" {
        calculate_depth(&paths, graph)
    } else if kind == "nd" {
        return node_degree_total(graph);
    } else {
        return calculate_similarity(&paths, graph);
    }
}
