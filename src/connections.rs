use std::{collections::{hash_map::Entry, HashMap}, env, fs::{self, File}, io::Write};

use color_print::{cprint, cprintln};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{graph::NodeData, syllablize::SyllablizedPhonemes};

#[derive(Serialize, Deserialize)]
pub struct SyllableConnections {
    connections: HashMap<NodeData, (NodeData, usize)>,
}

impl SyllableConnections {
    const CACHE_FILE: &'static str = "assets/internal/syllable-connections.ron";

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }

    pub fn new(syl_phones: &SyllablizedPhonemes) {
        let mut connections = Self { connections: HashMap::new() };
    
        if let Ok(contents) = fs::read(Self::cache_file()) {
            cprintln!("  <green, bold>Reading</green, bold> Syllable Connections Cache File...");
            connections.load(contents);
        } else {
            cprintln!("  <yellow, bold>Cache File Not Found</yellow, bold>, Rebuilding Syllable Connections...");
            connections.build(syl_phones);
            cprint!("  <green, bold>Finished</green, bold> Building Syllable Connections                     ");
        }
    }
    
    fn load(&mut self, contents: Vec<u8>) {
        let de: SyllableConnections = ron::de::from_bytes(contents.as_slice())
            .expect("Failed to parse file");
        cprint!("  <green, bold>Finished</green, bold> Reading Syllable Connections Cache File");
        *self = de;
    }

    fn build(&mut self, syl_phones: &SyllablizedPhonemes) {
        cprintln!("    <bold>[1/2]</bold> <green, bold>Building</green, bold> Syllable Connections...");

        let bar = ProgressBar::new(syl_phones.words.len() as u64)
            .with_style(ProgressStyle::with_template("  [{human_pos}/{human_len} ({percent}%)] | Elapsed: {elapsed} | ETA: {eta} {bar:50.green/gray}").unwrap());

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

        cprintln!("    <bold>[2/2]</bold> <green, bold>Writing</green, bold> Syllable Connections to File...");
        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");
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