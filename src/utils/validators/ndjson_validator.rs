use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use serde::de::IgnoredAny;

use crate::utils::BetterExpect;

pub fn validate_ndjson(path: &PathBuf, verbose: bool) {
    let file = File::open(path).better_expect(
        format!(
            "ERROR: Couldn't open input file [{}] for validation.",
            path.to_str().unwrap_or("[input.ndjson]")
        )
        .as_str(),
        verbose,
    );

    let mut reader = BufReader::with_capacity(16384, file);

    // read lines one by one and deserialize them to check for errors
    let mut buf: Vec<u8> = Vec::new();
    let mut idx: usize = 1;

    loop {
        // check for line reading errors
        let n = reader.read_until(b'\n', &mut buf).better_expect(
            format!(
                "ERROR: Couldn't read line [{}] in input file [{}].",
                idx,
                path.to_str().unwrap_or("[input.ndjson]")
            )
            .as_str(),
            verbose,
        );

        // check for EOF
        if n == 0 {
            break;
        };

        // check line validity
        serde_json::from_slice::<IgnoredAny>(&buf).better_expect(
            format!(
                "ERROR: Serialization error in input file [{}] at line [{}].",
                path.to_str().unwrap_or("[input.ndjson]"),
                idx
            )
            .as_str(),
            verbose,
        );
        buf.clear();
        idx += 1;
    }
}
