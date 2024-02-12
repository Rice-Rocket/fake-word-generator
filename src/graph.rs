use std::{collections::{hash_map::Entry, HashMap}, env, fs::{self, File}, io::Write, path::Path};

use indicatif::ProgressIterator;
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{logger::{ProgressBarElements, TerminalLogger, WorkIndex, WorkMessage}, phoneme::{Phoneme, SyllablePart}, syllable::Syllable, syllablize::SyllablizedPhonemes, utils};

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
    pub nodes: HashMap<NodeID, SonorityGraphNode>,
}

impl SonorityGraph {
    const CACHE_FILE: &'static str = "assets/internal/sonority-graph.ron";

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }
    pub fn cache_exists() -> bool {
        Path::new(&Self::cache_file()).exists()
    }
    pub fn try_read_cache() -> Result<Vec<u8>, std::io::Error> {
        fs::read(Self::cache_file())
    }

    pub fn new(syl_phones: &SyllablizedPhonemes, logger: &mut TerminalLogger) -> Self {
        let mut graph = Self { nodes: HashMap::new() };
        
        if let Ok(contents) = Self::try_read_cache() {
            graph.load(contents);
        } else {
            graph.build(syl_phones, logger);
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

    pub fn get_node(&self, id: NodeID) -> Option<&SonorityGraphNode> {
        self.nodes.get(&id)
    }
    pub fn get_node_mut(&mut self, id: NodeID) -> Option<&mut SonorityGraphNode> {
        self.nodes.get_mut(&id)
    }
    pub fn get_node_unchecked(&self, id: NodeID) -> &SonorityGraphNode {
        &self.nodes.get(&id).unwrap()
    }

    pub fn load(&mut self, contents: Vec<u8>) -> Option<()> {
        let de: SonorityGraph = match ron::de::from_bytes(contents.as_slice()) {
            Ok(d) => d,
            Err(_) => return None
        };
        *self = de;
        Some(())
    }

    pub fn build(&mut self, syl_phones: &SyllablizedPhonemes, logger: &mut TerminalLogger) {
        let build_work = logger.begin_work(WorkMessage::new("Building", "Sonority Graph", WorkIndex::new(1, 2)));

        let bar = logger.create_progress(syl_phones.words.len() as u64, ProgressBarElements::PERCENTAGE | ProgressBarElements::ETA);

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
                self.update_graph_part(SyllablePart::Coda { layer: 1 }, coda, NodeData::Stop);
            }
        }

        logger.finish_work(build_work);
        let writing_work = logger.begin_work(WorkMessage::new("Writing", "Syllablized Phonemes to File", WorkIndex::new(2, 2)));

        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");

        logger.finish_work(writing_work);
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

        let (mut cur_id, mut node_ids): (NodeID, Vec<NodeID>) = match (phonemes.get(0), part) {
            (Some(phone), SyllablePart::Onset | SyllablePart::Nucleus) => (
                NodeID { data: NodeData::Phoneme(*phone), part }, 
                phonemes
                    .split_at(1).1
                    .iter()
                    .map(|phone| NodeID { data: NodeData::Phoneme(*phone), part })
                    .collect()
            ),
            (Some(phone), SyllablePart::Coda { layer: _ }) => (
                NodeID { data: NodeData::Phoneme(*phone), part: SyllablePart::Coda { layer: 1 } }, 
                phonemes
                    .split_at(1).1
                    .iter()
                    .enumerate()
                    .map(|(i, phone)| NodeID { data: NodeData::Phoneme(*phone), part: SyllablePart::Coda { layer: i + 2 } })
                    .collect()
            ),
            (None, _) => return,
        };
        
        node_ids.push(match part.next() {
            Some(SyllablePart::Coda { layer: _ }) => NodeID { data: next, part: SyllablePart::Coda { layer: node_ids.len() + 1 } },
            Some(SyllablePart::Nucleus | SyllablePart::Onset) => NodeID { data: next, part: part.next().unwrap() },
            None => NodeID { data: NodeData::Stop, part: SyllablePart::Coda { layer: 0 } },
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

    fn eval(&self, result: &mut SonorityGraphResult, cur_id: NodeID, mut rng: &mut ThreadRng) {
        let cur_node = self.get_node_unchecked(cur_id);

        let edge = utils::weighted_random_choice(&cur_node.outs.iter().map(|edge| (edge.count, edge.clone())).collect(), &mut rng);
        let Some(next_node) = self.get_node(edge.to) else { return };
        
        let should_continue = next_node.evaluate(result, edge.to);
        if !should_continue { return };
        self.eval(result, edge.to, &mut rng);
    }
    pub fn evaluate(&self, rng: &mut ThreadRng) -> SonorityGraphResult {
        let mut result = SonorityGraphResult(Syllable::empty());
        let root_id = NodeID { data: NodeData::Start, part: SyllablePart::Onset };
        self.eval(&mut result, root_id, rng);
        result
    }
    pub fn evaluate_from_start(&self, start: Phoneme, rng: &mut ThreadRng) -> SonorityGraphResult {
        let mut result = SonorityGraphResult(Syllable::empty());
        let root_id = NodeID {
            data: NodeData::Phoneme(start), 
            part: match start.is_vowel() {
                true => SyllablePart::Nucleus,
                false => SyllablePart::Onset,
            }
        };
        let root_node = self.get_node_unchecked(root_id);
        root_node.evaluate(&mut result, root_id);
        self.eval(&mut result, root_id, rng);
        result
    }
}