use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::BetterExpect;

pub fn json_reader(
    path: &PathBuf,
    verbose: bool,
) -> serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>> {
    let file = File::open(path).better_expect(
        format!("ERROR: Couldn't open input file [{}].", path.to_str().unwrap_or("[input.json]"))
            .as_str(),
        verbose,
    );

    let buffered = BufReader::with_capacity(16384, file);

    serde_json::Deserializer::from_reader(buffered)
}
