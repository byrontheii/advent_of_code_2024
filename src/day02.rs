use std::error;

fn safe_move(a: i32, b: i32, increasing: bool) -> bool {
    a != b && a.abs_diff(b) <= 3 && if increasing {b>a} else {a>b}  
}

fn safe_report(levels: &[i32]) -> bool {
    let increasing = levels[1] > levels[0];
    for i in 0..(levels.len() - 1) {
        if !safe_move(levels[i], levels[i+1], increasing) {
            return false;
        }
    }
    true
}

fn safe_when_dampened(levels: &Vec<i32>) -> bool {
    for x in 0..levels.len() {
        let mut damp = levels.clone();
        damp.remove(x);
        if safe_report(&damp) {
            return true
        }
    }
    false
}

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let mut safe_count = 0;
        for line in super::load_file(&self.file_path).lines() {
            let levels: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            if safe_report(&levels) {
                safe_count += 1;
            }        
        }
        Ok(safe_count.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let mut safe_count = 0;
        for line in super::load_file(&self.file_path).lines() {
            let levels: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            if safe_report(&levels) || safe_when_dampened(&levels) {
                safe_count += 1;
            }        
        }
        Ok(safe_count.to_string())
    }

}