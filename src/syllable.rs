use std::ops::{Index, IndexMut};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::phoneme::{Phoneme, SyllablePart};


const RESPELL_KEY: [[&'static str; 2]; 68]  = [
    ["ire", "aɪər"],
    ["oir", "ɔɪər"],
    ["our", "aʊər"],
    ["eer", "ɪər"],
    ["air", "ɛər"],
    ["ure", "jʊər"],
    ["ur", "ɜːr"],
    ["ew", "juː"],
    ["eye", "aɪ"],
    ["err", "ɛr"],
    ["irr", "ɪr"],
    ["urr", "ʌr"],
    ["uurr", "ʊr"],
    ["uhr", "ər"],
    ["oor", "ʊər"],
    ["or", "ɔːr"],
    ["orr", "ɒr"],
    ["oh", "oʊ"],
    ["oo", "uː"],
    ["ar", "ɑːr"],
    ["arr", "ær"],
    ["y", "aɪ"],
    ["ay", "eɪ"],
    ["ee", "iː"],
    ["aw", "ɔː"],
    ["ow", "aʊ"],
    ["oy", "ɔɪ"],
    ["ah", "ɑː"],
    ["ah", "ɑ"],
    ["ee", "i"],
    ["oo", "u"],
    ["aw", "ɔ"],
    //   ["ə", "ə"],
    //   ["ər", "ər"],
    ["uh", "ə"], // use `uh` instead of ə
    //
    ["a", "æ"],
    ["o", "ɒ"],
    ["uu", "ʊ"],
    //
    ["i", "ɪ"],
    ["u", "ʌ"],
    ["e", "ɛ"],
    //   ["ih", "ɪ$"], // ɪ at the end of a syllable is 'ih' not 'i'
    //   ["uh", "ʌ$"], // ʌ at the end of a syllable is 'uh' not 'u'
    //   ["eh", "ɛ$"], // ɛ at the end of a syllable is 'eh' not 'e'
    //
    ["j", "dʒ"],
    ["nk", "ŋk"],
    ["wh", "hw"],
    ["b", "b"],
    ["ch", "tʃ"],
    ["d", "d"],
    ["dh", "ð"],
    ["f", "f"],
    ["g", "ɡ"],
    //   ["gh", "ɡ"], //  IGNORED: /ɡ/ may be respelled gh instead of g when otherwise it may be misinterpreted as /dʒ/.
    //   ["tch", "tʃ"], // IGNORED: /tʃ/ after a vowel in the same syllable is respelled tch instead of ch to better distinguish it from /k, x/.
    ["h", "h"],
    ["k", "k"],
    ["kh", "x"],
    ["l", "l"],
    ["l", "ɫ"],
    ["m", "m"],
    ["n", "n"],
    ["ng", "ŋ"],
    ["p", "p"],
    ["r", "ɹ"],
    ["r", "r"],
    ["s", "s"],
    //   ["ss", "s"], // /s/ may be respelled ss instead of s when otherwise it may be misinterpreted as /z/: "ice" EYESS, "tense" TENSS (compare eyes, tens).
    ["sh", "ʃ"],
    ["t", "t"],
    ["th", "θ"],
    ["v", "v"],
    ["w", "w"],
    ["y", "j"],
    ["z", "z"],
    ["zh", "ʒ"],
];


const SPECIAL_ENDERS: [[&'static str; 2]; 3] = [
  ["ih", "ɪ"], // ɪ at the end of a syllable is 'ih' not 'i'
  ["uh", "ʌ"], // ʌ at the end of a syllable is 'uh' not 'u'
  ["eh", "ɛ"], // ɛ at the end of a syllable is 'eh' not 'e'
];


#[derive(Serialize, Deserialize, Clone)]
pub struct Syllable {
    phonemes: Vec<Phoneme>,
}

impl Syllable {
    pub fn new(phonemes: Vec<Phoneme>) -> Self {
        Self { phonemes }
    }
    pub fn empty() -> Self {
        Self { phonemes: vec![] }
    }

    pub fn from_phonemes(phonemes: Vec<Phoneme>) -> Self {
        Self { phonemes }
    }

    pub fn from_arpabet(arpabet: &str) -> Self {
        let mut phonemes = Vec::new();
        for phoneme in arpabet.split(' ') {
            if let Some(caps) = Regex::new(r"^([A-Z]+)\d*$").unwrap().captures(phoneme) {
                let phone = Phoneme::from_arpabet(caps.get(1).unwrap().as_str());
                phonemes.push(phone);
            }
        }
        Syllable::from_phonemes(phonemes)
    }

    pub fn to_ipa(&self) -> String {
        let mut result = String::new();
        for phoneme in self.phonemes.iter() {
            result += phoneme.to_ipa();
        };
        result
    }
    
    pub fn to_english(&self) -> String {
        let mut result = String::new();
        let mut ipa = self.to_ipa();

        while ipa.chars().count() > 0 {
            let mut found_any = false;
            for [replace, check] in RESPELL_KEY {
                let ender = SPECIAL_ENDERS.iter().position(|[_, end]| *end == ipa);
                
                if let Some(end) = ender {
                    let eng_end = SPECIAL_ENDERS[end][0];
                    result += eng_end;
                    ipa = String::from("");
                    found_any = true;
                    break;
                }

                if ipa.starts_with(check) {
                    result += replace;
                    ipa = ipa[check.len()..].to_string();
                    found_any = true;
                    break;
                }
            };

            if !found_any {
                result += &ipa.chars().nth(0).unwrap().to_string();
                ipa.remove(0);
            }
        };

        result
    }

    pub fn add_phoneme(&mut self, phoneme: Phoneme) {
        self.phonemes.push(phoneme);
    }

    /// Returns a tuple where the first element is the onset, 
    /// the second is the nucleus, and the third is the coda. 
    /// 
    /// Returns none if the syllable is invalid (the coda has a vowel in it or there is no vowel at all). 
    pub fn split(&self) -> Option<(Vec<Phoneme>, Vec<Phoneme>, Vec<Phoneme>)> {
        let mut onset = Vec::new();
        let mut vowel = Vec::new();
        let mut coda = Vec::new();
        let mut state = SyllablePart::Onset;

        for phone in self.phonemes.iter() {
            if state == SyllablePart::Onset {
                if phone.is_consonant() {
                    onset.push(phone.clone());
                } else if phone.is_vowel() {
                    vowel.push(phone.clone());
                    state = SyllablePart::Nucleus;
                    continue;
                } else {
                    return None;
                }
            } else if state == SyllablePart::Nucleus {
                if phone.is_vowel() {
                    vowel.push(phone.clone());
                } else {
                    coda.push(phone.clone());
                    state = SyllablePart::Coda { layer: 1 };
                }
            } else { // state == SyllablePart::Coda
                if phone.is_consonant() {
                    coda.push(phone.clone());
                    state = match state {
                        SyllablePart::Coda { layer } => SyllablePart::Coda { layer: layer + 1 },
                        _ => { unreachable!() }
                    };
                } else {
                    return None;
                }
            }
        }

        return Some((onset, vowel, coda));
    }
}

impl Index<usize> for Syllable {
    type Output = Phoneme;

    fn index(&self, index: usize) -> &Self::Output {
        &self.phonemes[index]
    }
}

impl IndexMut<usize> for Syllable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.phonemes[index]
    }
}