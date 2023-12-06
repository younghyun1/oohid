use clap::Parser;
use uuid::Uuid;
use rayon::prelude::*;
use std::collections::HashSet;
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

    // Check for duplicates
    #[arg(long, help = "Goes over initial output to check for duplicates. Fixes silently.")]
    check: bool,

    // Verbose output
    #[arg(long, help = "Displays benchmarking info, also displays check results if applicable.")]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let args: Args = Args::parse();
    let mut formatted_uuids: Vec<String> = generate_uuids(args.count, &args.format);
    
    let (had_duplicates, duplicates_count) = if args.check {
        remove_duplicates(&mut formatted_uuids, &args.format)
    } else {
        (false, 0)
    };

    let output_content = format_output(&formatted_uuids, &args.format);

    match args.output {
        Some(file_name) => {
            let mut file = File::create(file_name)?;
            writeln!(file, "{}", output_content)?;
        }
        None => {
            println!("{}", output_content);
        }
    }

    if args.verbose {
        let duration = start.elapsed();
        println!("Time elapsed for UUID generation: {:?}", duration);
        if args.check{
            if had_duplicates {
                println!("Duplicates were found and replaced. Total duplicates: {}", duplicates_count);
            } else {
                println!("No duplicates were found.");
            }
        }
    }

    Ok(())
}

#[inline]
fn generate_uuids(count: u32, format: &str) -> Vec<String> {
    (0..count)
        .into_par_iter()
        .map(|_| Uuid::new_v4())
        .map(|uuid| format_uuid(&uuid, format))
        .collect()
}

#[inline]
fn format_uuid(uuid: &Uuid, format: &str) -> String {
    let base_format = uuid.to_string();
    match format {
        "u" => base_format,
        "ul" => format!("{},", base_format),
        "q" => format!("\"{}\"", base_format),
        "ql" | "qlb" | "qlbl" => format!("\"{}\",", base_format),
        _ => base_format,
    }
}

#[inline]
fn remove_duplicates(uuids: &mut Vec<String>, format: &str) -> (bool, u32) {
    let mut unique_uuids = HashSet::new();
    let mut duplicates: u32 = 0;

    // Identify and remove duplicates
    uuids.retain(|uuid| {
        if unique_uuids.contains(uuid) {
            duplicates += 1;
            false
        } else {
            unique_uuids.insert(uuid.clone());
            true
        }
    });

    // Generate and add new UUIDs for each duplicate removed
    for _ in 0..duplicates {
        loop {
            let new_uuid = format_uuid(&Uuid::new_v4(), format);
            if unique_uuids.insert(new_uuid.clone()) {
                uuids.push(new_uuid);
                break;
            }
        }
    }

    (duplicates > 0, duplicates)
}

#[inline]
fn format_output(uuids: &[String], format: &str) -> String {
    match format {
        "qlb" => format!("[\n\t{}\n]", uuids.join("\n\t")),
        "qlbl" => format!("{{\n\t{}\n}}", uuids.join("\n\t")),
        _ => uuids.join("\n").trim_end_matches(',').to_owned(),
    }
}
