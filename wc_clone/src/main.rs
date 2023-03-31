use clap::Parser;
use anyhow::{Context, Result};
use std::io::BufRead;
use bytecount;

// Simple wc -l clone
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // the input file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = std::fs::File::open(&args.path)
        .with_context(|| format!("failed to read file: {}", args.path.display()))?;

    let mut reader = std::io::BufReader::new(f);


    // Count the number of lines
    let mut nr_lines = 0;
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            nr_lines += bytecount::count(&buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    };

    println!("{}", nr_lines);
    Ok(())
}
