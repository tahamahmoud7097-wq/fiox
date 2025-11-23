// MiMalloc setup because MiMalloc is much faster than the default rust allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod utils;
use clap::Parser;
use colored::Colorize;
use std::path::Path;
use std::process::exit;
use utils::*;

fn main() {
    // Remove backtraces
    unsafe { std::env::remove_var("RUST_BACKTRACE") };

    let args: FioxArgs = cli::FioxArgs::parse();

    match args.cmd {
        Commands::Convert {
            verbose,
            input,
            output,
        } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("{}", "ERROR: Input file doesn't exist.".red().bold());
                exit(1);
            }

            // Check if output exists, if it doesn't then create it
            if !Path::new(&output).exists() {
                println!(
                    "{}",
                    format!(
                        "Output file [{}] doesn't exist, creating a new file for it...",
                        output.to_str().unwrap_or("output file")
                    )
                    .truecolor(179, 245, 216)
                    .italic()
                );
                std::fs::File::create(&output).better_expect("ERROR: Couldn't create output file.");
            }

            let repo_link = "https://github.com/tahamahmoud7097-wq/fiox"
                .truecolor(16, 101, 230)
                .bold();

            let input_ext = input
                .extension()
                .better_expect("ERROR: Input file has no valid extension.")
                .to_str()
                .better_expect("ERROR: Input file has no valid extension.");
            let output_ext = output
                .extension()
                .better_expect("ERROR: Output file has no valid extension.")
                .to_str()
                .better_expect("ERROR: Output file has no valid extension.");

            let now = std::time::Instant::now();

            let data: UniversalData = match input_ext {
                "txt" => txt_reader::read_from_txt(&input, output_ext),
                "json" => json_reader::json_reader(&input),
                "toml" => toml_reader::toml_reader(&input),
                "csv" => csv_reader::csv_reader(&input),
                _ => {
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Intput extension \"{}\" is not supported currently.",
                            input_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            match output_ext {
                "json" => write_json::write_json(&data, &output),
                "toml" => toml_writer::toml_writer(&data, &output),
                "csv" => csv_writer::csv_writer(&data, &output),
                _ => {
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Output extension \"{}\" is not supported currently.",
                            output_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            println!(
                "Finished converting {} -> {} in {:?}",
                input.to_str().unwrap_or("input file").bright_green().bold(),
                output
                    .to_str()
                    .unwrap_or("output file")
                    .bright_green()
                    .bold(),
                now.elapsed()
            );
        }

        Commands::Validate { input, verbose } => {
            let err_str = "STILL NOT SUPPORTED YET.".red().bold();
            eprintln!("{}", err_str);
        }
    }
}
