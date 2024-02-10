pub mod phoneme;
pub mod syllable;


use phoneme::Phoneme;
use syllable::Syllable;


fn main() {
    let syllable = Syllable::from_arpabet("HH AW S");

    let spelling = syllable.to_english();

    println!("{}", spelling);
}
