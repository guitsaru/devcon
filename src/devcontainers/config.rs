use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    name: String,
    build: Build,
    #[serde(default)]
    forward_ports: Vec<u16>,
    post_create_command: Option<String>,
    #[serde(default = "default_remote_user")]
    remote_user: String,
    #[serde(default)]
    run_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Build {
    dockerfile: Option<String>,
    #[serde(default)]
    args: HashMap<String, String>,
}

impl Config {
    pub fn parse(file: &Path) -> Result<Config, std::io::Error> {
        println!("File: {:?}", file);
        let contents = std::fs::read_to_string(file)?;
        let config: Config =
            json5::from_str(&contents).unwrap_or_else(|_| panic!("Could not parse {:?}", file));

        Ok(config)
    }
}

fn default_remote_user() -> String {
    "root".to_string()
}
