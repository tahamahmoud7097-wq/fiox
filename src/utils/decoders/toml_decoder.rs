use std::iter::once;

use crate::utils::{BetterExpect, ByteTypes, WriterStreams};

pub fn toml_decoder(
    content: toml::Value,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = ByteTypes>> {
    WriterStreams::LineByLine {
        iter: once(ByteTypes::Raw(
            toml::to_string_pretty(&content)
                .better_expect("ERROR: Failed to deserialize (decode) TOML.", verbose)
                .as_bytes()
                .to_vec(),
        )),
    }
}
