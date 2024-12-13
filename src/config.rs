use std::{fs, io::Error};

use serde::{Deserialize, Serialize};
use toml::value::Array;
#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlMonorepo {
    name: String,
    git: String,
    run: String,
    build: String,
    opt_build: Option<String>,
    opt_run: Option<String>,
    libs: Vec<ConfigTomlLib>,
    bins: Vec<ConfigTomlBin>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlLib {
    name: String,
    build: String,
    opt_build: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConfigTomlBin {
    name: String,
    run: String,
    opt_run: Option<String>,
    build: Option<String>,
    opt_build: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConfigToml {
    monorepo: ConfigTomlMonorepo,
}

#[derive(Debug)]
pub struct Config {}
impl Config {
    pub fn new() -> Self {
        let mut content: String = "".to_owned();
        let result: Result<String, Error> = fs::read_to_string(".superepo.toml");
        if result.is_ok() {
            content = result.unwrap();
        }
        let config_toml: ConfigToml = toml::from_str(&content).unwrap();
        println!("{content}");
        Self {}
    }
}
