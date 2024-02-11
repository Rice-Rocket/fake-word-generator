pub mod phoneme;
pub mod syllable;
pub mod syllablize;
pub mod graph;


use graph::SonorityGraph;


fn main() {
    let graph = SonorityGraph::new();
    let res = graph.evaluate();
    println!("{}, ({})", res.0.clone().to_ipa(), res.0.to_english())
}
