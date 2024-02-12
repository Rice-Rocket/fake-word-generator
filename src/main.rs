pub mod phoneme;
pub mod syllable;
pub mod syllablize;
pub mod graph;
pub mod wordgen;
pub mod connections;
pub mod word;
pub mod utils;

pub mod logger;

use tts_rust::tts::GTTSClient;
use wordgen::FakeWordGenerator;


fn main() {
    let mut generator = FakeWordGenerator::new();
    
    let tts = GTTSClient::default();

    for _ in 0..50 {
        let res = generator.generate_word();
        println!("{}", res);
        tts.speak(&res.to_english().clone()).unwrap();
    }
}
