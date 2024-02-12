use core::fmt;

use crate::syllable::Syllable;

pub struct Word {
    pub syllables: Vec<Syllable>,
}

impl Word {
    pub fn new(syllables: Vec<Syllable>) -> Word {
        Word { syllables }
    }
    pub fn empty() -> Word {
        Word { syllables: vec![] }
    }
    
    pub fn add_syllable(&mut self, syllable: Syllable) {
        self.syllables.push(syllable);
    }

    pub fn to_english(&self) -> String {
        let mut res = String::from(&self.syllables.first().unwrap().to_english());
        for syl in &self.syllables[1..] {
            res += "-";
            res += &syl.to_english();
        };
        res
    }

    pub fn to_ipa(&self) -> String {
        let mut res = String::from(&self.syllables.first().unwrap().to_ipa());
        for syl in &self.syllables[1..] {
            res += " ";
            res += &syl.to_ipa();
        };
        res
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.to_english(), self.to_ipa())
    }
}