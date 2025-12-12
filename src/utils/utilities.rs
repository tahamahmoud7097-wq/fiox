use std::process::exit;

use colored::Colorize;

use csv::ByteRecord;

pub enum WriterStreams {
    LineByLine { iter: Box<dyn Iterator<Item = ByteTypes>> },

    Table { headers: Vec<String>, iter: Box<dyn Iterator<Item = ByteTypes>> },
}

pub enum ByteTypes {
    Raw(Vec<u8>),

    Csv(ByteRecord),
}

pub fn into_raw_bytes(bytes: ByteTypes) -> Vec<u8> {
    match bytes {
        ByteTypes::Raw(raw_bytes) => raw_bytes,

        ByteTypes::Csv(byte_record) => byte_record.as_slice().to_vec(),
    }
}

pub fn into_byte_record(bytes: ByteTypes) -> ByteRecord {
    match bytes {
        ByteTypes::Csv(byte_record) => byte_record,

        ByteTypes::Raw(_) => ByteRecord::new(),
    }
}

// Custom better expect trait for better error messages without duping code

pub trait BetterExpect<T> {
    fn better_expect(self, msg: &str, verbose: bool) -> T;
}

// impl for Result which matches the value to Ok to return the value or print the error msg in red if Err
impl<T, E: std::fmt::Display> BetterExpect<T> for Result<T, E> {
    fn better_expect(self, msg: &str, verbose: bool) -> T {
        match self {
            Ok(v) => v,
            Err(_) if !verbose => {
                eprintln!("{}", msg.red().bold());
                exit(1);
            }
            Err(e) => {
                eprintln!("{}\n{}", msg.red().bold(), e);
                exit(1);
            }
        }
    }
}

// impl for Option to match the value for Some to return the actual value and if None prints error msg in red

impl<T> BetterExpect<T> for Option<T> {
    fn better_expect(self, msg: &str, _verbose: bool) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("{}", msg.red().bold());
                exit(1);
            }
        }
    }
}
