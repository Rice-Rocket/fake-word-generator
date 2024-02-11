use std::{collections::HashMap, env, fs::{self, File}, io::Write, path::Path, sync::{Arc, Mutex}};
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::{logger::{ProgressBarElements, TerminalLogger, WorkIndex, WorkMessage}, syllable::Syllable};


#[derive(Serialize, Deserialize)]
pub struct SyllablizedPhonemes {
    // sorted by frequency
    pub words: Vec<(String, Vec<Syllable>)>
}


impl SyllablizedPhonemes {
    const CACHE_FILE: &'static str = "assets/internal/syllablized-phonemes.ron";
    const WORD_FREQUENCY_FILE: &'static str = "assets/resources/word_frequency.txt";
    const CMU_FILE: &'static str = "assets/resources/cmudict.0.6-syl.txt";

    pub fn new(logger: &mut TerminalLogger) -> Self {
        let mut syl_phones = Self { words: Vec::new() };

        if let Ok(contents) = Self::try_read_cache() {
            syl_phones.load(contents);
        } else {
            syl_phones.generate(logger);
        }

        syl_phones
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
    pub fn cache_exists() -> bool {
        Path::new(&Self::cache_file()).exists()
    }
    pub fn try_read_cache() -> Result<Vec<u8>, std::io::Error> {
        fs::read(Self::cache_file())
    }
    
    pub fn load(&mut self, contents: Vec<u8>) -> Option<()> {
        let de: SyllablizedPhonemes = match ron::de::from_bytes(contents.as_slice()) {
            Ok(d) => d,
            Err(_) => return None
        };
        *self = de;
        Some(())
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

    pub fn generate(&mut self, logger: &mut TerminalLogger) {
        let read_work_freqs_work = logger.begin_work(WorkMessage::new("Reading", "Word Frequencies", WorkIndex::new(1, 5)));

        let word_freqs = Self::load_word_frequencies();
        let word_syllables_mutex: Arc<Mutex<HashMap<String, Vec<Syllable>>>> = Arc::new(Mutex::new(HashMap::new()));

        logger.sleep(0.25);
        logger.finish_work(read_work_freqs_work);
        let read_cmu_work = logger.begin_work(WorkMessage::new("Reading", "CMU Dictionary", WorkIndex::new(2, 5)));

        let cmu_file = fs::read_to_string(Self::cmu_file())
            .expect("Failed to load cmu file");
        let lines = cmu_file.lines();

        logger.sleep(0.25);
        logger.finish_work(read_cmu_work);
        let parse_cmu_work = logger.begin_work(WorkMessage::new("Parsing", "CMU Dictionary", WorkIndex::new(3, 5)));

        let bar = logger.create_progress(cmu_file.lines().count() as u64, ProgressBarElements::PERCENTAGE | ProgressBarElements::ETA);

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

        logger.finish_work(parse_cmu_work);
        let ordering_work = logger.begin_work(WorkMessage::new("Ordering", "Words By Frequency", WorkIndex::new(4, 5)));

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

        logger.sleep(0.25);
        logger.finish_work(ordering_work);
        let writing_work = logger.begin_work(WorkMessage::new("Writing", "Syllablized Phonemes to File", WorkIndex::new(5, 5)));

        self.words = ordered_words;
        let mut file = File::create(Self::cache_file()).unwrap();
        file.write_all(ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap().as_bytes())
            .expect("Failed to write to file");

        logger.sleep(0.25);
        logger.finish_work(writing_work);
    }
}