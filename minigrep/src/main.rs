use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(filename)
        .expect("Could not read file.");

    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = &args[1].clone();
        let filename = &args[2].clone();

        Config { query, filename }
    }
}

