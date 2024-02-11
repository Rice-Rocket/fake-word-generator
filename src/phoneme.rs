use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Phoneme {
    AA,
    AE,
    AH,
    AO,
    AW,
    AX,
    AXR,
    AY,
    EH,
    ER,
    EY,
    IH,
    IX,
    IY,
    OW,
    OY,
    UH,
    UW,
    UX,

    B,
    CH,
    D,
    DH,
    DX,
    EL,
    EM,
    EN,
    F,
    G,
    H,
    JH,
    K,
    L,
    M,
    N,
    NG,
    NX,
    P,
    Q,
    R,
    S,
    SH,
    T,
    TH,
    V,
    W,
    WH,
    Y,
    Z,
    ZH
}


impl Phoneme {
    pub fn from_arpabet(arpabet: &str) -> Self {
        match arpabet {
            "AA" => Self::AA,
            "AE" => Self::AE,
            "AH" => Self::AH,
            "AO" => Self::AO,
            "AW" => Self::AW,
            "AX" => Self::AX,
            "AXR" => Self::AXR,
            "AY" => Self::AY,
            "EH" => Self::EH,
            "ER" => Self::ER,
            "EY" => Self::EY,
            "IH" => Self::IH,
            "IX" => Self::IX,
            "IY" => Self::IY,
            "OW" => Self::OW,
            "OY" => Self::OY,
            "UH" => Self::UH,
            "UW" => Self::UW,
            "UX" => Self::UX,

            "B" => Self::B,
            "CH" => Self::CH,
            "D" => Self::D,
            "DH" => Self::DH,
            "DX" => Self::DX,
            "EL" => Self::EL,
            "EM" => Self::EM,
            "EN" => Self::EN,
            "F" => Self::F,
            "G" => Self::G,
            "HH" => Self::H,
            "H" => Self::H,
            "JH" => Self::JH,
            "K" => Self::K,
            "L" => Self::L,
            "M" => Self::M,
            "N" => Self::N,
            "NG" => Self::NG,
            "NX" => Self::NX,
            "P" => Self::P,
            "Q" => Self::Q,
            "R" => Self::R,
            "S" => Self::S,
            "SH" => Self::SH,
            "T" => Self::T,
            "TH" => Self::TH,
            "V" => Self::V,
            "W" => Self::W,
            "WH" => Self::WH,
            "Y" => Self::Y,
            "Z" => Self::Z,
            "ZH" => Self::ZH,

            _ => panic!("Could not convert {} to a phoneme", arpabet),
        }
    }

    pub fn to_arpabet(self) -> &'static str {
        match self {
            Self::AA => "AA",
            Self::AE => "AE",
            Self::AH => "AH",
            Self::AO => "AO",
            Self::AW => "AW",
            Self::AX => "AX",
            Self::AXR => "AXR",
            Self::AY => "AY",
            Self::EH => "EH",
            Self::ER => "ER",
            Self::EY => "EY",
            Self::IH => "IH",
            Self::IX => "IX",
            Self::IY => "IY",
            Self::OW => "OW",
            Self::OY => "OY",
            Self::UH => "UH",
            Self::UW => "UW",
            Self::UX => "UX",

            Self::B => "B",
            Self::CH => "CH",
            Self::D => "D",
            Self::DH => "DH",
            Self::DX => "DX",
            Self::EL => "EL",
            Self::EM => "EM",
            Self::EN => "EN",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::JH => "JH",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::NG => "NG",
            Self::NX => "NX",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::SH => "SH",
            Self::T => "T",
            Self::TH => "TH",
            Self::V => "V",
            Self::W => "W",
            Self::WH => "WH",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::ZH => "ZH",
        }
    }

    pub fn from_ipa(ipa: &str) -> Self {
        match ipa {
            "ɑ" => Self::AA, // ɑ or ɒ
            "æ" => Self::AE,
            "ʌ" => Self::AH,
            "ɔ" => Self::AO,
            "aʊ" => Self::AW,
            "əɹ" => Self::AX, // ɚ
            "ə" => Self::AXR,
            "aɪ" => Self::AY,
            "ɛ" => Self::EH,
            "ɛɹ" => Self::ER, // ɝ
            "eɪ" => Self::EY,
            "ɪ" => Self::IH,
            "ɨ" => Self::IX,
            "i" => Self::IY,
            "oʊ" => Self::OW,
            "ɔɪ" => Self::OY,
            "ʊ" => Self::UH,
            "u" => Self::UW,
            "ʉ" => Self::UX,

            "b" => Self::B,
            "tʃ" => Self::CH,
            "d" => Self::D,
            "ð" => Self::DH,
            "ɾ" => Self::DX,
            "l̩" => Self::EL,
            "m̩" => Self::EM,
            "n̩" => Self::EN,
            "f" => Self::F,
            "ɡ" => Self::G,
            "h" => Self::H,
            "dʒ" => Self::JH,
            "k" => Self::K,
            "l" => Self::L,
            "m" => Self::M,
            "n" => Self::N,
            "ŋ" => Self::NG,
            "ɾ̃" => Self::NX,
            "p" => Self::P,
            "ʔ" => Self::Q,
            "ɹ" => Self::R,
            "s" => Self::S,
            "ʃ" => Self::SH,
            "t" => Self::T,
            "θ" => Self::TH,
            "v" => Self::V,
            "w" => Self::W,
            "ʍ" => Self::WH,
            "j" => Self::Y,
            "z" => Self::Z,
            "ʒ" => Self::ZH,

            _ => panic!("Could not convert {} to a phoneme", ipa),
        }
    }

    pub fn to_ipa(self) -> &'static str {
        match self {
            Self::AA => "ɑ", // ɑ or ɒ
            Self::AE => "æ",
            Self::AH => "ʌ",
            Self::AO => "ɔ",
            Self::AW => "aʊ",
            Self::AX => "əɹ", // ɚ
            Self::AXR => "ə",
            Self::AY => "aɪ",
            Self::EH => "ɛ",
            Self::ER => "ɛɹ", // ɝ
            Self::EY => "eɪ",
            Self::IH => "ɪ",
            Self::IX => "ɨ",
            Self::IY => "i",
            Self::OW => "oʊ",
            Self::OY => "ɔɪ",
            Self::UH => "ʊ",
            Self::UW => "u",
            Self::UX => "ʉ",

            Self::B => "b",
            Self::CH => "tʃ",
            Self::D => "d",
            Self::DH => "ð",
            Self::DX => "ɾ",
            Self::EL => "l̩",
            Self::EM => "m̩",
            Self::EN => "n̩",
            Self::F => "f",
            Self::G => "ɡ",
            Self::H => "h",
            Self::JH => "dʒ",
            Self::K => "k",
            Self::L => "l",
            Self::M => "m",
            Self::N => "n",
            Self::NG => "ŋ",
            Self::NX => "ɾ̃",
            Self::P => "p",
            Self::Q => "ʔ",
            Self::R => "ɹ",
            Self::S => "s",
            Self::SH => "ʃ",
            Self::T => "t",
            Self::TH => "θ",
            Self::V => "v",
            Self::W => "w",
            Self::WH => "ʍ",
            Self::Y => "j",
            Self::Z => "z",
            Self::ZH => "ʒ",
        }
    }

    pub fn is_vowel(&self) -> bool {
        match self {
            Self::AA => true,
            Self::AE => true,
            Self::AH => true,
            Self::AO => true,
            Self::AW => true,
            Self::AX => true,
            Self::AXR => true,
            Self::AY => true,
            Self::EH => true,
            Self::ER => true,
            Self::EY => true,
            Self::IH => true,
            Self::IX => true,
            Self::IY => true,
            Self::OW => true,
            Self::OY => true,
            Self::UH => true,
            Self::UW => true,
            Self::UX => true,
            _ => false
        }
    }
    pub fn is_consonant(&self) -> bool {
        !self.is_vowel()
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub enum SyllablePart {
    Onset,
    Nucleus,
    Coda { layer: usize },
}

impl SyllablePart {
    pub fn next(self) -> Option<Self> {
        match self {
            Self::Onset => Some(Self::Nucleus),
            Self::Nucleus => Some(Self::Coda { layer: 1 }),
            Self::Coda { layer: _ } => None,
        }
    }
    pub fn is_onset(self) -> bool {
        match self {
            Self::Onset => true,
            _ => false,
        }
    }
    pub fn is_nucleus(self) -> bool {
        match self {
            Self::Nucleus => true,
            _ => false,
        }
    }
    pub fn is_coda(self) -> bool {
        match self {
            Self::Coda { layer: _ } => true,
            _ => false,
        }
    }
}