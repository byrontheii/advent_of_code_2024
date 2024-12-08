use std::{fs, error};
use regex::Regex;

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let re = Regex::new(r"mul\(\d{1,3}?,\d{1,3}?\)").unwrap();
        let mut sum: i64 = 0;

        for capt in re.find_iter(&input).map(|c| c.as_str()) {
            println!("{capt}");
            let args = capt.split('(')
                                 .collect::<Vec<&str>>()[1]
                                 .split(')')
                                 .collect::<Vec<&str>>()[0]
                                 .split(',')
                                 .collect::<Vec<&str>>();
            sum += args[0].parse::<i64>().unwrap() * args[1].parse::<i64>().unwrap();
        }
        
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let re = Regex::new(r"mul\(\d{1,3}?,\d{1,3}?\)|do\(\)|don't\(\)").unwrap();
        let mut sum: i64 = 0;
        let mut enabled = true;

        for capt in re.find_iter(&input).map(|c| c.as_str()) {
            println!("{capt}");
            if capt == "do()" {
                enabled = true;
                continue;
            }
            if capt == "don't()" {
                enabled = false;
            }
            if !enabled {
                continue;
            }
            let args = capt.split('(')
                                 .collect::<Vec<&str>>()[1]
                                 .split(')')
                                 .collect::<Vec<&str>>()[0]
                                 .split(',')
                                 .collect::<Vec<&str>>();
            sum += args[0].parse::<i64>().unwrap() * args[1].parse::<i64>().unwrap();
        }
        
        Ok(sum.to_string())
    }

}