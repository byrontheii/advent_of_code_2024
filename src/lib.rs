extern crate pest;
extern crate pest_derive;
use std::fs;
use std::error::Error;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn make_path(args: &Vec<String>) -> String {
    format!("data/{}{}.txt", args[1], if args.len() >= 4 {format!("_{}", args[3])} else {"".to_string()})
}

pub fn run(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let day: Box<dyn Runner> = match args[1].as_str() {
        "01" => Box::new(day01::Day01{file_path: make_path(args)}),
        "02" => Box::new(day02::Day02{file_path: make_path(args)}),
        "03" => Box::new(day03::Day03{file_path: make_path(args)}),
        "04" => Box::new(day04::Day04{file_path: make_path(args)}),
        "05" => Box::new(day05::Day05{file_path: make_path(args)}),
        "06" => Box::new(day06::Day06{file_path: make_path(args)}),
        _ => Err(format!("Unrecognized command '{}'", args[1]))?
    };
    match args[2].as_str() {
        "a" => day.run_a(),
        "b" => day.run_b(),
        _ => Err(format!("Unrecognized part '{}'", args[2]))?
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
