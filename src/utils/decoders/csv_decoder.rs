use std::{fs::File, io::BufReader};

use crate::utils::{BetterExpect, ByteTypes, WriterStreams};

pub fn csv_decoder(
    mut reader: csv::Reader<BufReader<File>>,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = ByteTypes>> {
    let headers = reader
        .headers()
        .better_expect("ERROR: Couldn't read input file headers.", verbose)
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let iter = reader.into_byte_records().enumerate().map(move |(line_no, rec)| {
        ByteTypes::Csv(
            rec.better_expect(
                format!("ERROR: Serialization error in input file in record [{}]", line_no + 1)
                    .as_str(),
                verbose,
            ),
        )
    });

    WriterStreams::Table { headers, iter }
}
