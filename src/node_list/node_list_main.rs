use crate::node_list::wrapper::wrapper_node;
use clap::ArgMatches;
use gfa_reader::{Gfa, Pansn};

/// Main function for node list
pub fn nodelist_main(matches: &ArgMatches) {
    eprintln!("Running node-list analysis.");

    // Parse GFA file + Wrapper
    let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
    graph.walk_to_path();
    let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, " ");

    let output = matches.value_of("output").unwrap();
    let splits = vec!["Core", "Length", "Depth", "Core", "ND"];
    let mut split_vec = Vec::new();
    if matches.is_present("Features") {
        split_vec = matches.value_of("Features").unwrap().split(',').collect();
    }
    let mut final_features = Vec::new();
    for x in split_vec.iter() {
        if splits.contains(x) {
            final_features.push(*x);
        }
    }

    if final_features.is_empty() {
        final_features = splits.clone();
    }
    // This wrapper also writes data to a file
    wrapper_node(&graph, &wrapper, output, final_features);
}
