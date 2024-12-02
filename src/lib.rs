extern crate pest;
extern crate pest_derive;
use std::fs;
use std::error::Error;
mod day1;

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    match config.command.as_str() {
        "day1a" => day1::run_a(&config.file_path),
        "day1b" => day1::run_a(&config.file_path),
        _ => Err(format!("Unrecognized command '{}'", &config.command))?
    }
}
pub struct Config {
    pub command: String,
    pub file_path: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        Config {
            command: args[1].clone(),
            file_path: args[2].clone(),
            args: args.into_iter().skip(3).collect(),
        }
    }
}

fn load_file(file_name: &String) -> String {
    let file_string = fs::read_to_string(file_name).unwrap();
    file_string
}
