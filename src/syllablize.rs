use std::{collections::HashMap, env, fs::{self, File}, io::Write, sync::{Arc, Mutex}};
use color_print::cprintln;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::syllable::Syllable;


#[derive(Serialize, Deserialize)]
pub struct SyllablizedPhonemes {
    // sorted by frequency
    pub words: Vec<(String, Vec<Syllable>)>
}


impl SyllablizedPhonemes {
    const CACHE_FILE: &'static str = "assets/internal/syllablized-phonemes.ron";
    const WORD_FREQUENCY_FILE: &'static str = "assets/resources/word_frequency.txt";
    const CMU_FILE: &'static str = "assets/resources/cmudict.0.6-syl.txt";

    pub fn new() -> Self {
        if let Ok(contents) = fs::read(Self::cache_file()) {
            cprintln!("  <bold>[1/3]</bold> <green, bold>Reading</green, bold> Syllablized Phonemes Cache File...");
            Self::load(contents)
        } else {
            cprintln!("  <bold>[1/3]</bold> <yellow, bold>Cache File Not Found</yellow, bold>, Regenerating Syllablized Phonemes...");
            Self::generate()
        }
    }

    fn cache_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CACHE_FILE
    }
    fn word_freq_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::WORD_FREQUENCY_FILE
    }
    fn cmu_file() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + Self::CMU_FILE
    }
    
    fn load(contents: Vec<u8>) -> Self {
        let de: SyllablizedPhonemes = ron::de::from_bytes(contents.as_slice())
            .expect("Failed to parse file");
        cprintln!("  <bold>[1/3]</bold> <green, bold>Finished</green, bold> Reading Syllablized Phonemes Cache File");
        de
    }

    fn load_word_frequencies() -> Vec<String> {
        let contents = fs::read_to_string(Self::word_freq_file())
            .expect("Failed to load word frequency file");
        let lines = contents.split("\n");
        let words: Vec<String> = lines
            .map(|line| line.split("\t").nth(0).unwrap().to_owned())
            .collect();

        // only consider first 60,000 most frequent words
        words.split_at(60000).0.to_vec()
    }

    fn generate() -> Self {
        cprintln!("    <bold>[1/4]</bold> <green, bold>Reading</green, bold> Word Frequencies...");
        let word_freqs = Self::load_word_frequencies();
        let word_syllables_mutex: Arc<Mutex<HashMap<String, Vec<Syllable>>>> = Arc::new(Mutex::new(HashMap::new()));

        cprintln!("    <bold>[2/4]</bold> <green, bold>Reading</green, bold> CMU Dictionary...");
        let cmu_file = fs::read_to_string(Self::cmu_file())
            .expect("Failed to load cmu file");
        let lines = cmu_file.lines();

        cprintln!("    <bold>[3/4]</bold> <green, bold>Parsing</green, bold> CMU Dictionary...");
        let bar = ProgressBar::new(cmu_file.lines().count() as u64)
            .with_style(ProgressStyle::with_template("      [{human_pos}/{human_len} ({percent}%)] | Elapsed: {elapsed} | ETA: {eta} {bar:50.green/gray}").unwrap());

        lines.par_bridge().progress_with(bar).for_each(|line| {
            if line.starts_with("#") { return };
            if line.trim() == "" { return };

            let elements: Vec<&str> = line.splitn(2, "  ").collect();
            let word = elements[0];
            let sounds = elements[1];

            if Regex::new(r".*\(\d+\)$").unwrap().is_match(word) { return };
            let syllables: Vec<Syllable> = sounds
                .split(".")
                .map(|arpabet| Syllable::from_arpabet(arpabet))
                .collect();
            
            let word_lower = word.to_lowercase();
            word_syllables_mutex.lock().unwrap().insert(word_lower, syllables);
        });

        let mut word_syllables = word_syllables_mutex.lock().unwrap();

        let mut ordered_words = vec![];
        for word in word_freqs {
            if let Some(syls) = word_syllables.get(&word) {
                ordered_words.push((word.clone(), syls.clone()));
                word_syllables.remove(&word);
            }
        }

        ordered_words.append(&mut word_syllables
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(String, Vec<Syllable>)>>());

        cprintln!("    <bold>[4/4]</bold> <green, bold>Writing</green, bold> Syllablized Phonemes to File...");
        let syl_phones = SyllablizedPhonemes { words: ordered_words };
        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&syl_phones, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");

        syl_phones
    }
}