use crate::config::Config;
use crate::infra::Server;
use clap::{App, Arg};
use std::fs::File;

mod config;
mod crypto;
mod infra;
mod infra_linear_scan;
mod infra_lru_scan;
mod util;

fn main() {
    let matches = App::new("Shadomplexer")
        .version("0.1")
        .author("DuckSoft & DuckVador")
        .about("one port for all")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("load config from YAML file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.yaml");
    let file = File::open(config_path)
        .expect(format!("failed to open config file {}", config_path).as_str());
    let config: Config = serde_yaml::from_reader(file)
        .expect(format!("failed to parse config file {}", config_path).as_str());
    let mut server = Server::new(config);
    smol::block_on(async { server.run().await.unwrap() });
}
