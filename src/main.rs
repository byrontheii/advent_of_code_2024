use std::{env, process};
use advent_of_code_2024::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args);

    println!("running {}", config.command);
    println!("On file {}", config.file_path);
    let result = advent_of_code_2024::run(config);
    match result {
        Ok(v) => {
            println!("Answer: {v}");
            process::exit(0);
        }
        Err(e) => {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
}
