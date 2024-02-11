use std::{collections::{hash_map::Entry, HashMap}, env, fs::{self, File}, io::Write};

use color_print::{cprint, cprintln};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{phoneme::{Phoneme, SyllablePart}, syllable::Syllable, syllablize::SyllablizedPhonemes};

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
            result.0.add_phoneme(phoneme.clone());
            true
        } else if NodeData::Start == id.data {
            true
        } else {
            false
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SonorityGraphEdge {
    pub from: NodeID,
    pub to: NodeID,
    pub count: usize,
}


pub struct SonorityGraphResult(pub Syllable);


#[derive(Serialize, Deserialize)]
pub struct SonorityGraph {
    nodes: HashMap<NodeID, SonorityGraphNode>,
}

impl SonorityGraph {
    const CACHE_FILE: &'static str = "assets/internal/sonority-graph.ron";

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }

    pub fn new() -> Self {
        let mut graph = Self { nodes: HashMap::new() };
        
        println!();
        if let Ok(contents) = fs::read(Self::cache_file()) {
            cprintln!("<green, bold>Reading</green, bold> Sonority Graph Cache File...");
            graph.load(contents);
        } else {
            cprintln!("<yellow, bold>Cache File Not Found</yellow, bold>, Regenerating Sonority Graph...");
            let syllablized_phonemes = SyllablizedPhonemes::new();
            graph.build(syllablized_phonemes);
            cprintln!("<green, bold>Finished</green, bold> Building Sonority Graph");
        }
        println!();

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
        cprint!("<green, bold>Finished</green, bold> Reading Sonority Graph Cache File");
        *self = de;
    }

    fn build(&mut self, syl_phones: SyllablizedPhonemes) {
        cprintln!("  <bold>[2/3]</bold> <green, bold>Building</green, bold> Sonority Graph...");

        let bar = ProgressBar::new(syl_phones.words.len() as u64)
            .with_style(ProgressStyle::with_template("  [{human_pos}/{human_len} ({percent}%)] | Elapsed: {elapsed} | ETA: {eta} {bar:50.green/gray}").unwrap());

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

        cprintln!("  <bold>[3/3]</bold> <green, bold>Writing</green, bold> Syllablized Phonemes to File...");
        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");
    }
    fn update_graph_part(&mut self, part: SyllablePart, phonemes: Vec<Phoneme>, next: NodeData) {
        if part == SyllablePart::Onset {
            let from_node_id = NodeID { data: NodeData::Start, part };
            let to_node_id = match phonemes.get(0) {
                Some(phone) => NodeID { data: NodeData::Phoneme(*phone), part },
                None => NodeID { data: next, part: SyllablePart::Nucleus },
            };
            self.add_node(from_node_id);
            self.add_node(to_node_id);
            self.add_edge(from_node_id, to_node_id);
        }

        let (mut cur_id, mut node_ids): (NodeID, Vec<NodeID>) = match phonemes.get(0) {
            Some(phone) => (NodeID { data: NodeData::Phoneme(*phone), part }, phonemes.split_at(1).1.iter().map(|phone| NodeID { data: NodeData::Phoneme(*phone), part }).collect()),
            None => return,
        };
        node_ids.push(match part.next() {
            Some(next_part) => NodeID { data: next, part: next_part },
            None => NodeID { data: NodeData::Stop, part: SyllablePart::Coda },
        });

        for next_id in node_ids {
            let from_node_id = cur_id;
            let to_node_id = next_id;
            self.add_node(from_node_id);
            self.add_node(to_node_id);
            self.add_edge(from_node_id, to_node_id);
            cur_id = next_id;
        }
    }

    fn weighted_random_choice(a: Vec<(usize, SonorityGraphEdge)>, rng: &mut ThreadRng) -> SonorityGraphEdge {
        let mut weights = Vec::new();

        for i in 0..a.len() {
            if i == 0 {
                weights.push(a[i].0);
            } else {
                weights.push(a[i].0 + weights[i - 1]);
            }
        }

        let rand = rng.gen_range(0..weights[weights.len() - 1]);
        let mut i = 0;
        for _ in 0..weights.len() {
            if weights[i] > rand {
                break;
            }
            i += 1;
        }
        return a[i].1.clone();
    }
    fn random_choice(a: Vec<(usize, SonorityGraphEdge)>, rng: &mut ThreadRng) -> SonorityGraphEdge {
        let rand = rng.gen_range(0..a.len());
        return a[rand].1.clone();
    }
    fn eval(&self, result: &mut SonorityGraphResult, cur_id: NodeID, mut rng: &mut ThreadRng) {
        let cur_node = self.get_node_unchecked(cur_id);
        
        // println!("{:?}, {:?}", cur_id.data, cur_id.part);
        let edge = Self::weighted_random_choice(cur_node.outs.iter().map(|edge| (edge.count, edge.clone())).collect(), &mut rng);
        let Some(next_node) = self.get_node(edge.to) else { return };
        
        let should_continue = next_node.evaluate(result, edge.to);
        if !should_continue { return };
        self.eval(result, edge.to, &mut rng);
    }
    pub fn evaluate(&self) -> SonorityGraphResult {
        let mut result = SonorityGraphResult(Syllable::empty());
        let mut rng = thread_rng();
        let root_id = NodeID { data: NodeData::Start, part: SyllablePart::Onset };
        self.eval(&mut result, root_id, &mut rng);
        result
    }
}