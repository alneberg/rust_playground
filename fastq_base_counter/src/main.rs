use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader, Result};

struct Config {
    file_path: String,
}

fn parse_config(args: Vec<String>) -> Config {
    if args.len() < 2 {
        panic!("not enough parameters");
    }

    let file_path = args[1].clone();

    Config { file_path }
}

fn read_file_line_by_line(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(args);

    if let Err(e) = read_file_line_by_line(&config.file_path) {
        eprintln!("Error reading file: {}", e);
        return Err(e);
    }

    Ok(())
}

