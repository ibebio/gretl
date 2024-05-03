use crate::path_similarity::stats::accession2level;
use crate::path_similarity::writer_test::write_ps;
use clap::ArgMatches;
use gfa_reader::{Gfa, Pansn};

/// Main function for path related stats
pub fn ps_main(matches: &ArgMatches) {
    let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file(matches.value_of("gfa").unwrap());
    graph.walk_to_path();
    let wrapper: Pansn<u32, (), ()> = Pansn::from_graph(&graph.paths, " ");
    let data = accession2level(&graph, &wrapper);
    let output = matches.value_of("output").unwrap();

    write_ps(&data, output);
}
