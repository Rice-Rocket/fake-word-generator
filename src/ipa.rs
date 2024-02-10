pub enum IPA {
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
    HH,
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


impl IPA {
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
            "HH" => Self::HH,
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

            _ => panic!("Could not convert {} to IPA", arpabet),
        }
    }

    pub fn to_char(self) -> &'static str {
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
            Self::HH => "h",
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
}