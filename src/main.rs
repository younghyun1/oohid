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
    #[arg(short, long, default_value = "b", help = "Set the format of the UUIDs ('u' for bare, 'ul' for bare w. comma, 'q' for quoted, 'ql' for quoted w. comma)")]
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
        .map(|uuid| match args.format.as_str() {
            "u" => uuid.to_string(),
            "ul" => format!("{},", uuid),
            "q" => format!("\"{}\"", uuid),
            "ql" => format!("\"{}\",", uuid),
            _ => uuid.to_string(),
        })
        .collect();

    match args.output {
        Some(file_name) => {
            let mut file = File::create(file_name)?;
            for formatted_uuid in formatted_uuids {
                writeln!(file, "{}", formatted_uuid)?;
            }
        }
        None => {
            for formatted_uuid in formatted_uuids {
                println!("{}", formatted_uuid);
            }
        }
    }

    let duration = start.elapsed();
    if args.time {
        println!("Time elapsed for UUID generation is: {:?}", duration);
    }

    Ok(())
}
