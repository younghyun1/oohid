use clap::Parser;
use uuid::Uuid;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Number of UUIDs to generate
    #[arg(short, long, default_value_t = 1, help = "Specify the number of UUIDs to generate")]
    count: u32,

    // Format of the UUIDs
    #[arg(short, long, default_value = "u", help = "Set the format of the UUIDs ('u' for bare, 'ul' for bare w. comma, 'q' for quoted, 'ql' for quoted w. comma, 'qlb' for [] brackets, 'qlbl' for {} brackets)")]
    format: String,

    // Output file
    #[arg(short, long, help = "Specify an output file. Prints to stdout if not set")]
    output: Option<String>,

    // Measure and display time taken to generate UUIDs
    #[arg(long, help = "Display the time taken to generate UUIDs")]
    time: bool,
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let args: Args = Args::parse();
    let num_ids: u32 = args.count;

    let formatted_uuids: Vec<String> = (0..num_ids)
        .into_par_iter()
        .map(|_| Uuid::new_v4())
        .map(|uuid| {
            let base_format = uuid.to_string();
            match args.format.as_str() {
                "u" => base_format,
                "ul" => format!("{},", base_format),
                "q" => format!("\"{}\"", base_format),
                "ql" | "qlb" | "qlbl" => format!("\"{}\",", base_format),
                _ => base_format,
            }
        })
        .collect();

    let output_content = match args.format.as_str() {
        "qlb" => format!("[\n\t{}\n]", formatted_uuids.join("\n\t")),
        "qlbl" => format!("{{\n\t{}\n}}", formatted_uuids.join("\n\t")),
        _ => formatted_uuids.join("\n").trim_end_matches(',').to_owned(),
    };

    match args.output {
        Some(file_name) => {
            let mut file = File::create(file_name)?;
            writeln!(file, "{}", output_content)?;
        }
        None => {
            println!("{}", output_content);
        }
    }

    if args.time {
        let duration = start.elapsed();
        println!("Time elapsed for UUID generation is: {:?}", duration);
    }

    Ok(())
}
