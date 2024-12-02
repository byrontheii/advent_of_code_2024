extern crate pest;
extern crate pest_derive;
use std::fs;
use std::error::Error;
mod day01;

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    match config.day.as_str() {
        "01a" => day01::run_a(&config.file_path),
        "01b" => day01::run_a(&config.file_path),
        _ => Err(format!("Unrecognized command '{}'", &config.day))?
    }
}
pub struct Config {
    pub day: String,
    pub file_path: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        let ex = match args[2].as_str() {
            "ex" => "_ex",
            _ => "",
        };
        Config {
            day: args[1].clone(),
            file_path: format!("data/{}{}.txt", args[1], ex),
            args: args.into_iter().skip(3).collect(),
        }
    }
}

fn load_file(file_name: &String) -> String {
    let file_string = fs::read_to_string(file_name).unwrap();
    file_string
}
