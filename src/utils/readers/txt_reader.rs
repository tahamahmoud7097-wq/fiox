use std::path::PathBuf;

use crate::{
    utilities::{UniversalData, Vals},
    utils::BetterExpect,
};
use serde_json::Value as JsonVal;
use toml::Value as TomlVal;

pub fn read_from_txt(path: &PathBuf, output_ext: &str) -> UniversalData {
    // matches output file extension to determine how to serialize
    let content = std::fs::read_to_string(path).better_expect("ERROR: Failed to read input file.");
    match output_ext {
        "csv" => {
            let rows: Vec<Vec<String>> = content
                .lines()
                .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
                .collect();
            let headers: &Vec<String> = &rows[1];
            UniversalData::Table {
                headers: headers.to_vec(),
                rows: rows[1..rows.len()].to_vec(),
            }
        }
        "json" => {
            let objs: JsonVal = serde_json::from_str(&content)
                .better_expect("ERROR: Failed to deserialize file into JSON format.");
            UniversalData::Structured(Vals::Json(objs))
        }
        "toml" => {
            let tomls: TomlVal = toml::from_str(&content)
                .better_expect("ERROR: Failed to deserialize file into TOML format.");
            UniversalData::Structured(Vals::Toml(tomls))
        }
        _ => std::process::exit(1),
    }
}
