use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = advent_of_code_2024::run(&args);
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
