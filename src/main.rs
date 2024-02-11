pub mod phoneme;
pub mod syllable;
pub mod syllablize;
pub mod graph;


use graph::SonorityGraph;
use syllable::Syllable;
use syllablize::SyllablizedPhonemes;


fn main() {
    let phones = SyllablizedPhonemes::new();
    let graph = SonorityGraph::new(phones);
}
