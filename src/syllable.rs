use std::ops::{Index, IndexMut};

use crate::phoneme::Phoneme;


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


pub struct Syllable {
    phonemes: Vec<Phoneme>,
}

impl Syllable {
    pub fn from_phonemes(phonemes: Vec<Phoneme>) -> Self {
        Self { phonemes }
    }

    pub fn from_arpabet(arpabet: &'static str) -> Self {
        let mut phonemes = Vec::new();
        for phoneme in arpabet.split(' ') {
            phonemes.push(Phoneme::from_arpabet(phoneme));
        }
        Syllable::from_phonemes(phonemes)
    }

    pub fn to_ipa(self) -> String {
        let mut result = String::new();
        for phoneme in self.phonemes {
            result += phoneme.to_ipa();
        };
        result
    }
    
    pub fn to_english(self) -> String {
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