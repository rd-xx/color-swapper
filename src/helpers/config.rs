use crate::helpers::file;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub input_folder: String,
    pub output_folder: String,
    pub colors: Vec<ColorToSwap>
}

#[derive(Deserialize)]
pub struct ColorToSwap {
    pub from: String,
    pub to: String
}

pub fn read_config() -> Config {
    let content = file::read_file("config.json");
    let json: serde_json::Value = serde_json::from_str(&content).expect("Couldn't parse the config file. Did you break it?");
    let config: Config = serde_json::from_value(json.clone()).expect("Couldn't parse the config file. Did you break it?");

    return config;
}