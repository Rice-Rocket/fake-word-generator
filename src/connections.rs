use std::{collections::{hash_map::Entry, HashMap}, env, fs::{self, File}, io::Write, path::Path};

use indicatif::ProgressIterator;
use serde::{Deserialize, Serialize};

use crate::{graph::NodeData, logger::{ProgressBarElements, TerminalLogger, WorkIndex, WorkMessage}, syllablize::SyllablizedPhonemes};

#[derive(Serialize, Deserialize)]
pub struct SyllableConnections {
    pub connections: HashMap<NodeData, (NodeData, usize)>,
}

impl SyllableConnections {
    const CACHE_FILE: &'static str = "assets/internal/syllable-connections.ron";

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }
    pub fn try_read_cache() -> Result<Vec<u8>, std::io::Error> {
        fs::read(Self::cache_file())
    }
    pub fn cache_exists() -> bool {
        Path::new(&Self::cache_file()).exists()
    }

    pub fn new(syl_phones: &SyllablizedPhonemes, logger: &mut TerminalLogger) {
        let mut connections = Self { connections: HashMap::new() };
    
        if let Ok(contents) = Self::try_read_cache() {
            connections.load(contents);
        } else {
            connections.build(syl_phones, logger);
        }
    }
    
    pub fn load(&mut self, contents: Vec<u8>) -> Option<()> {
        let de: SyllableConnections = match ron::de::from_bytes(contents.as_slice()) {
            Ok(d) => d,
            Err(_) => return None
        };
        *self = de;
        Some(())
    }

    pub fn build(&mut self, syl_phones: &SyllablizedPhonemes, logger: &mut TerminalLogger) {
        let build_work = logger.begin_work(WorkMessage::new("Building", "Syllable Connections", WorkIndex::new(1, 2)));

        let bar = logger.create_progress(syl_phones.words.len() as u64, ProgressBarElements::PERCENTAGE | ProgressBarElements::ETA);

        for (_word, syllables) in syl_phones.words.iter().progress_with(bar) {
            if syllables.len() <= 1 { continue };

            for (i, syllable) in syllables.iter().enumerate() {
                if i == 0 {
                    self.add_edge(NodeData::Start, NodeData::Phoneme(syllable.first_phoneme()));
                }

                match syllables.get(i + 1) {
                    Some(next_syl) => {
                        self.add_edge(NodeData::Phoneme(syllable.last_phoneme()), NodeData::Phoneme(next_syl.first_phoneme()));
                    },
                    None => {
                        self.add_edge(NodeData::Phoneme(syllable.last_phoneme()), NodeData::Stop);
                    }
                }
            }
        }

        logger.finish_work(build_work);
        let writing_work = logger.begin_work(WorkMessage::new("Writing", "Syllable Connections to File", WorkIndex::new(2, 2)));

        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");

        logger.finish_work(writing_work);
    }

    fn add_edge(&mut self, from: NodeData, to: NodeData) {
        match self.connections.entry(from) {
            Entry::Vacant(entry) => {
                entry.insert((to, 1));
            },
            Entry::Occupied(mut entry) => {
                entry.get_mut().1 += 1;
            }
        }
    }
}