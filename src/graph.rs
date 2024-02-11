use std::{collections::{hash_map::Entry, HashMap}, env, fs};

use color_print::cprintln;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{phoneme::{Phoneme, SyllablePart}, syllablize::SyllablizedPhonemes};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NodeID {
    pub data: NodeData,
    pub part: SyllablePart,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeData {
    Start,
    Stop,
    Phoneme(Phoneme),
}

#[derive(Serialize, Deserialize)]
pub struct SonorityGraphNode {
    pub outs: Vec<SonorityGraphEdge>,
}

impl SonorityGraphNode {
    pub fn new() -> SonorityGraphNode {
        Self { outs: Vec::new() }
    }
    pub fn evaluate(&self, result: &mut SonorityGraphResult, id: NodeID) -> bool {
        if let NodeData::Phoneme(phoneme) = &id.data {
            result.phonemes.push(phoneme.clone());
            true
        } else {
            false
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct SonorityGraphEdge {
    pub from: NodeID,
    pub to: NodeID,
    pub count: usize,
}


pub struct SonorityGraphResult {
    pub phonemes: Vec<Phoneme>,
}

impl SonorityGraphResult {
    pub fn initial() -> Self {
        Self { phonemes: Vec::new() }
    }
}


#[derive(Serialize, Deserialize)]
pub struct SonorityGraph {
    root_node: Option<NodeID>,
    nodes: HashMap<NodeID, SonorityGraphNode>,
}

impl SonorityGraph {
    const CACHE_FILE: &'static str = "assets/internal/sonority-graph.ron";

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }

    pub fn new(syllablized_phonemes: SyllablizedPhonemes) -> Self {
        let mut graph = Self {
            root_node: None,
            nodes: HashMap::new(),
        };

        println!("");
        if let Ok(contents) = fs::read(Self::cache_file()) {
            cprintln!("<green, bold>Reading</green, bold> Sonority Graph Cache File...");
            graph.load(contents);
        } else {
            cprintln!("<yellow, bold>Cache File Not Found</yellow, bold>, Regenerating Sonority Graph...");
            graph.build(syllablized_phonemes);
            cprintln!("<green, bold>Finished</green, bold> Building Sonority Graph");
        }

        graph
    }
    pub fn add_node(&mut self, id: NodeID) {
        match self.nodes.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(SonorityGraphNode::new());
            }
            Entry::Occupied(_) => (),
        }
    }
    pub fn add_edge(&mut self, from: NodeID, to: NodeID) {
        if self.get_node(to).is_none() { return }

        if let Some(from_node) = self.get_node_mut(from) {
            let mut has_edge = false;
            for edge in from_node.outs.iter_mut() {
                if edge.from == from && edge.to == to {
                    has_edge = true;
                    edge.count += 1;
                }
            }
            if !has_edge {
                from_node.outs.push(SonorityGraphEdge { from, to, count: 1 });
            }
        }
    }
    pub fn set_root(&mut self, node: NodeID) {
        self.root_node = Some(node);
    }

    pub fn get_node(&self, id: NodeID) -> Option<&SonorityGraphNode> {
        self.nodes.get(&id)
    }
    pub fn get_node_mut(&mut self, id: NodeID) -> Option<&mut SonorityGraphNode> {
        self.nodes.get_mut(&id)
    }
    pub fn get_node_unchecked(&self, id: NodeID) -> &SonorityGraphNode {
        &self.nodes.get(&id).unwrap()
    }

    fn load(&mut self, contents: Vec<u8>) {
        let de: SonorityGraph = ron::de::from_bytes(contents.as_slice())
            .expect("Failed to parse file");
        cprintln!("<green, bold>Finished</green, bold> Reading Sonority Graph Cache File");
        *self = de;
    }

    fn build(&mut self, syl_phones: SyllablizedPhonemes) {
        cprintln!("  <green, bold>Building</green, bold> Sonority Graph...");

        let bar = ProgressBar::new(syl_phones.words.len() as u64)
            .with_style(ProgressStyle::with_template("    [{human_pos}/{human_len} ({percent}%)] | Elapsed: {elapsed} | ETA: {eta} {bar:50.green/gray}").unwrap());

        for (_word, syllables) in syl_phones.words.iter().progress_with(bar) {
            for syl in syllables.iter() {
                let (onset, nucleus, coda) = syl.split().unwrap();
                self.update_graph_part(SyllablePart::Onset, onset, match nucleus.get(0) {
                    Some(phone) => NodeData::Phoneme(*phone),
                    None => NodeData::Stop,
                });
                self.update_graph_part(SyllablePart::Nucleus, nucleus, match coda.get(0) {
                    Some(phone) => NodeData::Phoneme(*phone),
                    None => NodeData::Stop,
                });
                self.update_graph_part(SyllablePart::Coda, coda, NodeData::Stop);
            }
        }
    }
    fn update_graph_part(&mut self, part: SyllablePart, phonemes: Vec<Phoneme>, next: NodeData) {
        if part == SyllablePart::Onset {
            let node_data = match phonemes.get(0) {
                Some(phone) => NodeData::Phoneme(*phone),
                None => next,
            };
            let from_node_id = NodeID { data: NodeData::Start, part: SyllablePart::Onset };
            let to_node_id = NodeID { data: node_data, part: SyllablePart::Onset };
            self.add_node(from_node_id);
            self.add_node(to_node_id);
            self.add_edge(from_node_id, to_node_id);
        }

        let (mut cur_data, mut node_datas) = match phonemes.get(0) {
            Some(phone) => (NodeData::Phoneme(*phone), phonemes.split_at(1).1.iter().map(|phone| NodeData::Phoneme(*phone)).collect()),
            None => (NodeData::Stop, Vec::new()),
        };
        node_datas.push(next);

        for next_data in node_datas {
            let from_node_id = NodeID { data: cur_data, part };
            let to_node_id = NodeID { data: next_data, part };
            self.add_node(from_node_id);
            self.add_node(to_node_id);
            self.add_edge(from_node_id, to_node_id);
            cur_data = next_data;
        }
    }

    fn eval(&self, result: &mut SonorityGraphResult, cur_id: NodeID) {
        let cur_node = self.get_node_unchecked(cur_id);
        
        for edge in cur_node.outs.iter() {
            let Some(next_node) = self.get_node(edge.to) else { continue };
            
            let should_continue = next_node.evaluate(result, edge.to);
            if !should_continue { continue };
            self.eval(result, edge.to);
        }
    }
    pub fn evaluate(&self) -> SonorityGraphResult {
        let mut result = SonorityGraphResult::initial();
        let Some(root_id) = self.root_node else { panic!("No root node set for sonority graph") };
        self.eval(&mut result, root_id);
        result
    }
}