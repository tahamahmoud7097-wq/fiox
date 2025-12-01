use std::{fs::File, io::BufReader, path::PathBuf};

use crate::{utilities::BetterExpect, utils::DecoderStreams};

pub fn csv_reader(path: &PathBuf, verbose: bool) -> DecoderStreams {
    let file = File::open(path).better_expect(
        format!("ERROR: Couldn't open input file [{}].", path.to_str().unwrap_or("[input.csv]"))
            .as_str(),
        verbose,
    );

    let reader = BufReader::with_capacity(16384, file);

    DecoderStreams::CsvStream { stream: reader }
}
