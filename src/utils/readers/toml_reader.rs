use std::path::PathBuf;

use crate::utils::{BetterExpect, DecoderStreams};
use toml::Value as TomlVal;

pub fn toml_reader(path: &PathBuf, verbose: bool) -> DecoderStreams {
    let file_bytes = std::fs::read(path).better_expect(
        format!(
            "ERROR: Couldn't read input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );

    let toml_ser = toml::from_slice::<TomlVal>(&file_bytes).better_expect(
        format!(
            "ERROR: Serialization error in input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );

    DecoderStreams::Toml { data: toml_ser }
}
