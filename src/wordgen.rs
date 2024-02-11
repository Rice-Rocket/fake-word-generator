use std::{env, fs};

use color_print::cprintln;

use crate::{connections::SyllableConnections, graph::SonorityGraph, syllablize::SyllablizedPhonemes};


pub struct FakeWordGenerator {
    pub syllablized_phonemes: SyllablizedPhonemes,
    pub sonority_graph: SonorityGraph,
    pub syllable_connections: SyllableConnections,
}

impl FakeWordGenerator {
    pub fn new() -> Self {
        cprintln!("<green, bold>Initializing</green, bold> Fake Word Generator...");

        
    }
}