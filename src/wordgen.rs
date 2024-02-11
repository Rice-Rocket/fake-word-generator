use std::{collections::HashMap, env, fs};

use crate::{connections::SyllableConnections, graph::SonorityGraph, logger::{TerminalLogger, WorkIndex, WorkMessage}, syllablize::SyllablizedPhonemes};


pub struct FakeWordGenerator {
    pub syllablized_phonemes: SyllablizedPhonemes,
    pub sonority_graph: SonorityGraph,
    pub syllable_connections: SyllableConnections,
}

impl FakeWordGenerator {
    pub fn new() -> Self {
        let mut logger = TerminalLogger::new();

        logger.initialize();
        let init_work = logger.begin_work(WorkMessage::new("Initializing", "Fake Word Generator", WorkIndex::None));

        let mut syllablized_phonemes = SyllablizedPhonemes { words: vec![] };
        let mut sonority_graph = SonorityGraph { nodes: HashMap::new() };
        let mut syllable_connections = SyllableConnections { connections: HashMap::new() };

        logger.begin_section();

        let mut created_syl_phones = false;
        if !SonorityGraph::cache_exists() || !SyllableConnections::cache_exists() {
            created_syl_phones = true;
            let gen_syl_phones_work = logger.begin_work(WorkMessage::new("Generating", "Syllablized Phonemes", WorkIndex::new(1, 3)));
            let mut regenerate = true;
            logger.begin_section();
            let find_cache_work = logger.begin_work(WorkMessage::new("Finding", "Cache File", WorkIndex::new(1, 3)));

            if SyllablizedPhonemes::cache_exists() {

                logger.sleep(0.25);
                logger.finish_work(find_cache_work);
                let read_cache_work = logger.begin_work(WorkMessage::new("Reading", "Cache File", WorkIndex::new(2, 3)));

                if let Ok(contents) = SyllablizedPhonemes::try_read_cache() {

                    logger.sleep(0.25);
                    logger.finish_work(read_cache_work);
                    let parse_cache_work = logger.begin_work(WorkMessage::new("Parsing", "Cached Data", WorkIndex::new(3, 3)));

                    match syllablized_phonemes.load(contents) {
                        Some(_) => {
                            logger.sleep(0.25);
                            logger.finish_work(parse_cache_work);
                            regenerate = false;
                        },
                        None => {
                            logger.sleep(0.25);
                            logger.fail_work(parse_cache_work, "Regenerating");
                        }
                    };
                } else {
                    logger.sleep(0.25);
                    logger.fail_work(read_cache_work, "Regenerating");
                }
            } else {
                logger.sleep(0.25);
                logger.fail_work(find_cache_work, "Regenerating");
            }

            if regenerate {
                logger.begin_section();
                syllablized_phonemes.generate(&mut logger);
                logger.end_section();
            }
            logger.finish_work(gen_syl_phones_work);
            logger.end_section();
        }


        let gen_graph_work = logger.begin_work(WorkMessage::new("Generating", "Sonority Graph", match created_syl_phones {
            true => WorkIndex::new(2, 3),
            false => WorkIndex::new(1, 2),
        }));
        let mut regenerate = true;
        logger.begin_section();
        let find_cache_work = logger.begin_work(WorkMessage::new("Finding", "Cache File", WorkIndex::new(1, 3)));

        if SonorityGraph::cache_exists() {

            logger.sleep(0.25);
            logger.finish_work(find_cache_work);
            let read_cache_work = logger.begin_work(WorkMessage::new("Reading", "Cache File", WorkIndex::new(2, 3)));

            if let Ok(contents) = SonorityGraph::try_read_cache() {

                logger.sleep(0.25);
                logger.finish_work(read_cache_work);
                let parse_cache_work = logger.begin_work(WorkMessage::new("Parsing", "Cached Data", WorkIndex::new(3, 3)));

                match sonority_graph.load(contents) {
                    Some(_) => {
                        logger.sleep(0.25);
                        logger.finish_work(parse_cache_work);
                        regenerate = false;
                    },
                    None => {
                        logger.sleep(0.25);
                        logger.fail_work(parse_cache_work, "Regenerating");
                    }
                };
            } else {
                logger.sleep(0.25);
                logger.fail_work(read_cache_work, "Regenerating");
            }
        } else {
            logger.sleep(0.25);
            logger.fail_work(find_cache_work, "Regenerating");
        }

        if regenerate {
            logger.begin_section();
            sonority_graph.build(&syllablized_phonemes, &mut logger);
            logger.end_section();
        }
        logger.finish_work(gen_graph_work);
        logger.end_section();


        let gen_conn_work = logger.begin_work(WorkMessage::new("Generating", "Syllable Connections", match created_syl_phones {
            true => WorkIndex::new(3, 3),
            false => WorkIndex::new(2, 2),
        }));
        let mut regenerate = true;
        logger.begin_section();
        let find_cache_work = logger.begin_work(WorkMessage::new("Finding", "Cache File", WorkIndex::new(1, 3)));

        if SyllableConnections::cache_exists() {

            logger.sleep(0.25);
            logger.finish_work(find_cache_work);
            let read_cache_work = logger.begin_work(WorkMessage::new("Reading", "Cache File", WorkIndex::new(2, 3)));

            if let Ok(contents) = SyllableConnections::try_read_cache() {

                logger.sleep(0.25);
                logger.finish_work(read_cache_work);
                let parse_cache_work = logger.begin_work(WorkMessage::new("Parsing", "Cached Data", WorkIndex::new(3, 3)));

                match syllable_connections.load(contents) {
                    Some(_) => {
                        logger.sleep(0.25);
                        logger.finish_work(parse_cache_work);
                        regenerate = false;
                    },
                    None => {
                        logger.sleep(0.25);
                        logger.fail_work(parse_cache_work, "Regenerating");
                    }
                };
            } else {
                logger.sleep(0.25);
                logger.fail_work(read_cache_work, "Regenerating");
            }
        } else {
            logger.sleep(0.25);
            logger.fail_work(find_cache_work, "Regenerating");
        }

        if regenerate {
            logger.begin_section();
            syllable_connections.build(&syllablized_phonemes, &mut logger);
            logger.end_section();
        }
        logger.finish_work(gen_conn_work);
        logger.end_section();

        logger.end_section();
        logger.finish_work(init_work);
        logger.finish();

        Self {
            syllablized_phonemes,
            sonority_graph,
            syllable_connections,
        }
    }
}