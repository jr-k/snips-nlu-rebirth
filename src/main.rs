//#[macro_use]
extern crate toml;
extern crate log;
extern crate snips_nlu_lib;
extern crate snips_nlu_ontology;
extern crate serde_json;
extern crate env_logger;

mod mqtt;
mod schema;

use snips_nlu_lib::SnipsNluEngine;
use clap::{App, Arg};
use std::fs;
use schema::config;

fn main() {
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();


    println!("snips-nlu-rebirth v{}", env!("CARGO_PKG_VERSION"));
    println!("========================");

    let cli = App::new("snips-nlu-rebirth")
        .about("Snips NLU interactive CLI for parsing intents")
        .arg(
            Arg::with_name("NLU_ENGINE_DIR")
                .required(true)
                .default_value("")
                .takes_value(true)
                    .index(1)
                    .help("path to the trained nlu engine directory"),
        )
         .arg(
            Arg::with_name("CONF_FILE")
                .short("c")
                .long("--conf")
                .default_value("./snips-nlu.toml")
                .takes_value(true)
                .help("path to the configuration file"),
        )
        .get_matches();

    let conf_file = cli.value_of("CONF_FILE").unwrap();
    let config: config::Config = *parse_configuration(conf_file);

    let mut engine_dir = cli.value_of("NLU_ENGINE_DIR").unwrap();

    if engine_dir.len() == 0 {
        engine_dir = &config.global.engine_dir;
    }

    let engine: SnipsNluEngine = *load_nlu_engine(engine_dir);

    let mut engine_holder = mqtt::EngineContext { engine: engine, engine_dir: engine_dir.into() };

    engine_holder.start(&config);
}

fn parse_configuration(conf_file: &str) -> Box<config::Config> {
    println!("\nLoading the conf file...");
    let contents = fs::read_to_string(conf_file).expect("Something went wrong reading the configuration file");
    let config: config::Config = toml::from_str(&contents).unwrap();
    Box::new(config)
}

fn load_nlu_engine(engine_dir: &str) -> Box<SnipsNluEngine> {
    println!("\nLoading the nlu engine...");
    let engine: SnipsNluEngine = SnipsNluEngine::from_path(engine_dir).expect("Can't find engine");
    Box::new(engine)
}