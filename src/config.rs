use std::{fs, path::Path, process::exit, sync::OnceLock};

use log::{error, warn};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub targets: Targets,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
    // pub log_level: String,
    pub accept_invalid_certs: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Targets {
    pub hosts: Vec<String>,
}

pub static CONFIG_CELL: OnceLock<Config> = OnceLock::new();

// TODO: improve configuration parsing
pub fn read_config<P: AsRef<Path>>(path: P) {
    let config_str = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            error!("could not read config file: {}", e);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&config_str) {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    if config.targets.hosts.is_empty() {
        warn!("List of targets is empty!");
    }

    for str in &config.targets.hosts {
        if !str.contains("http") {
            error!("check the configuration file!");
            exit(1);
        }
    }

    for host in &config.targets.hosts {
        let http_url = format!("http://{}:{}", config.server.host, config.server.port);
        let https_url = format!("https://{}:{}", config.server.host, config.server.port);

        if (host.trim() == http_url) || (host.trim() == https_url) {
            error!("dont set a monitor url to the host address silly (it will deadlock)");
            exit(1);
        }
    }

    CONFIG_CELL.get_or_init(|| config);
}
