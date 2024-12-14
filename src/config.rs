use std::{fs, io::Error};

use serde::{Deserialize, Serialize};
use toml::value::Array;
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigTomlMonorepo {
    pub name: String,
    pub git: String,
    pub run: String,
    pub build: String,
    pub opt_build: Option<String>,
    pub opt_run: Option<String>,
    pub libs: Option<Vec<ConfigTomlLib>>,
    pub bins: Option<Vec<ConfigTomlBin>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigTomlLib {
    pub name: String,
    pub build: String,
    pub opt_build: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigTomlBin {
    pub name: String,
    pub run: String,
    pub opt_run: Option<String>,
    pub build: String,
    pub opt_build: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigToml {
    pub monorepo: ConfigTomlMonorepo,
}

#[derive(Debug)]
pub struct Config {
    pub config_toml: ConfigToml,
}
impl Config {
    pub fn new() -> Self {
        let mut content: String = "".to_owned();
        let result: Result<String, Error> = fs::read_to_string(".superepo.toml");
        if result.is_ok() {
            content = result.unwrap();
        }
        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|err| {
            println!("Failed to create ConfigToml Object out of config file.");
            println!("{:#}", err);
            ConfigToml {
                monorepo: ConfigTomlMonorepo {
                    name: "default".to_owned(),
                    git: "".to_owned(),
                    run: "".to_owned(),
                    bins: None,
                    libs: None,
                    build: "".to_owned(),
                    opt_build: None,
                    opt_run: None,
                },
            }
        });

        Self { config_toml }
    }
}
