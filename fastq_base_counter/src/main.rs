use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader, Result};

fn read_file_line_by_line(filepath: &str) -> Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path: &str = &args[1];
    if let Err(e) = read_file_line_by_line(file_path) {
        eprintln!("Error reading file: {}", e);
    }

    Ok(())
}

