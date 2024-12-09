use std::{error, fs};

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        
        Ok(String::new())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        
        Ok(String::new())
    }

}