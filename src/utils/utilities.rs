use std::fs::File;
use std::io::BufReader;
use std::process::exit;

use colored::Colorize;
use serde_json::de::IoRead;
use toml::Value as TomlVal;

pub enum DecoderStreams {
    Json { stream: serde_json::Deserializer<IoRead<BufReader<File>>> },

    Toml { data: TomlVal },

    CsvStream { stream: BufReader<File> },

    NdjsonStrean { stream: BufReader<File> },
}

pub enum WriterStreams {
    LineByLine { iter: Box<dyn Iterator<Item = Vec<u8>>> },

    Table { headers: Vec<String>, iter: Box<dyn Iterator<Item = Vec<u8>>> },
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
