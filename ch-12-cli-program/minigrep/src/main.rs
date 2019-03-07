use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for \"{}\" in \"{}\"", config.query, config.filename);

    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read the file");
    println!("With text: \n{}", contents);
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

struct Config {
    query: String,
    filename: String,
}
