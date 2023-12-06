use clap::Parser;
use uuid::Uuid;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    count: u32,
    #[arg(short, long, default_value = "b")]
    format: String,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let args: Args = Args::parse();
    let num_ids: u32 = args.count;

    let formatted_uuids: Vec<String> = (0..num_ids)
        .into_par_iter()
        .map(|_| Uuid::new_v4())
        .map(|uuid| match args.format.as_str() {
            "b" => uuid.to_string(),
            "q" => format!("\"{}\"", uuid),
            "l" => format!("\"{}\",", uuid),
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
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    
    Ok(())
}
