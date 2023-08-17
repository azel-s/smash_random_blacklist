use crate::StageKind;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Entry {
    #[serde(default)]
    pub allow: Vec<String>,
}

#[derive(Deserialize)]
pub struct UserConfig(pub HashMap<String, Entry>);

impl UserConfig {
    pub fn load() -> Option<UserConfig> {
        match std::fs::read_to_string("sd:/ultimate/random-allow-config.toml") {
            Ok(data) => match toml::from_str(&data) {
                Ok(res) => Some(res),
                Err(err) => {
                    println!("[random-allow-config::UserConfig::load] Error! {:?}", err);
                    None
                }
            },
            Err(err) => {
                println!("[random-allow-config::UserConfig::load] Error! {:?}", err);
                None
            }
        }
    }
}
