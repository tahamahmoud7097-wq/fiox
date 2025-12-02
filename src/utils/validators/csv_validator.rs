use std::path::PathBuf;

use colored::Colorize;

use crate::utils::BetterExpect;

pub fn validate_csv(path: &PathBuf, verbose: bool) {
    let mut reader = csv::Reader::from_path(path).better_expect(
        format!(
            "ERROR: Couldn't read input file [{}] for validation.",
            path.to_str().unwrap_or("[input.csv]")
        )
        .as_str(),
        verbose,
    );

    let headers = reader
        .byte_headers()
        .better_expect(
            format!(
                "ERROR: Couldn't get headers for input file [{}] for validation.",
                path.to_str().unwrap_or("[input.csv]")
            )
            .as_str(),
            verbose,
        )
        .clone();

    let headers_len = headers.len();

    reader.byte_records().enumerate().for_each(|(idx, rec)| {
        let record = rec.better_expect(
            format!(
                "ERROR: Serialization error in input file [{}] at line [{}].",
                path.to_str().unwrap_or("[input.csv]"),
                idx + 2
            )
            .as_str(),
            verbose,
        );

        if record.len() != headers_len {
            eprintln!(
                "{}",
                format!(
                    "ERROR: Number of fields at line [{}] doesn't match the number of headers.",
                    idx + 2
                )
                .as_str()
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    });
}
