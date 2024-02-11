pub mod phoneme;
pub mod syllable;
pub mod syllablize;
pub mod graph;
// pub mod wordgen;
pub mod connections;


#[cfg(target_os = "macos")]
use cocoa_foundation::base::id;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSRunLoop;
use connections::SyllableConnections;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

use graph::SonorityGraph;
use syllablize::SyllablizedPhonemes;
// use tts_rust::{tts::GTTSClient, languages::Languages};
use tts::*;


fn main() {
    // let narrator = GTTSClient {
    //     volume: 1.0,
    //     language: Languages::English,
    //     tld: "com",
    // };
    let syl_phones = SyllablizedPhonemes::new();
    let connections = SyllableConnections::new(&syl_phones);
    let graph = SonorityGraph::new(&syl_phones);

    let mut tts = Tts::default().unwrap();
    tts.set_voice(&tts.voices().unwrap()[55]).unwrap();

    for _ in 0..20 {
        let res = graph.evaluate();
        println!("{}, ({})", res.0.to_ipa(), res.0.to_english());
        
        tts.speak(&res.0.to_english(), false).unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
        unsafe {
            let _: () = msg_send![run_loop, run];
        }
    }

}
