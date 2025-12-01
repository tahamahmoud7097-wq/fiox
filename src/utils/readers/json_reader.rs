use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::{BetterExpect, DecoderStreams};

pub fn json_reader(path: &PathBuf, verbose: bool) -> DecoderStreams {
    let file = File::open(path).better_expect(
        format!("ERROR: Couldn't open input file [{}].", path.to_str().unwrap_or("[input.json]"))
            .as_str(),
        verbose,
    );

    let buffered = BufReader::with_capacity(16384, file);

    let reader = serde_json::Deserializer::from_reader(buffered);

    DecoderStreams::Json { stream: reader }
}
