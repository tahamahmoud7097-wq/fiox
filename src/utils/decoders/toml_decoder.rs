use crate::utils::{ByteTypes, WriterStreams};

pub fn toml_decoder(content: Vec<u8>) -> WriterStreams {
    WriterStreams::LineByLine { iter: Box::new(std::iter::once(ByteTypes::Raw(content))) }
}
