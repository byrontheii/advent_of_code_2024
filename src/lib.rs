extern crate pest;
extern crate pest_derive;
use std::fs;
use std::error::Error;
mod day01;
mod day02;

fn make_path(args: &Vec<String>) -> String {
    let ex = match args[3].as_str() {
        "ex" => "_ex",
        _ => "",
    };
    format!("data/{}{}.txt", args[1], ex)
}

pub fn run(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let day: Box<dyn Runner> = match args[1].as_str() {
        "01" => Box::new(day01::Day01{file_path: make_path(args)}),
        "02" => Box::new(day02::Day02{file_path: make_path(args)}),
        _ => Err(format!("Unrecognized command '{}'", args[1]))?
    };
    match args[2].as_str() {
        "a" => day.run_a(),
        "b" => day.run_b(),
        _ => Err(format!("Unrecognized part '{}'", args[2]))?
    }
}

pub struct Config {
    pub day: String,
    pub part: String,
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
            part: args[2].clone(),
            file_path: format!("data/{}{}.txt", args[1], ex),
            args: args.into_iter().skip(3).collect(),
        }
    }
}

pub trait Runner {
    fn run_a(&self) -> Result<String, Box<dyn Error>>;
    fn run_b(&self) -> Result<String, Box<dyn Error>>;
}

fn load_file(file_name: &String) -> String {
    println!("Loading file {file_name}");
    let file_string = fs::read_to_string(file_name).unwrap();
    file_string
}
