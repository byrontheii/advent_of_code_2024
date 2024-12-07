use std::{fs, error};

pub struct Day04 {
    pub file_path: String
}

const WORD_F: &str = "XMAS";
const WORD_R: &str = "SAMX";

fn check_line(line: &str) -> i64 {
    let mut sum: i64 = 0;
    for col in 0..(line.len() - 3) {
        let subs: &str = &line[col..(col + 4)];
        println!("{subs}");
        if subs == WORD_F || subs == WORD_R {
            sum += 1;
            println!("+1");
        }
    }
    sum
}

fn check_x(lines: &[&str], col: usize) -> bool {
    let mut corners = String::new();
    corners.push(lines[0].chars().nth(col).unwrap());
    corners.push(lines[0].chars().nth(col + 2).unwrap());
    corners.push(lines[2].chars().nth(col).unwrap());
    corners.push(lines[2].chars().nth(col + 2).unwrap());
    let center = lines[1].chars().nth(col + 1).unwrap();
    println!("{corners}; {center}");
    center == 'A' && (
        corners == "MMSS" || corners == "MSMS" || corners == "SSMM" || corners == "SMSM"
    )
}

impl super::Runner for Day04 {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let mut sum = 0;
        let mut puzzle: Vec<String> = Vec::new();
        let mut puzzle_down: Vec<String> = Vec::new();
        println!("Checking rows");
        for line in super::load_file(&self.file_path).lines() {
            puzzle.push(line.to_string());
            sum += check_line(line);
            for (col, ch) in line.chars().enumerate() {
                if puzzle_down.len() <= col {
                    puzzle_down.push(String::new());
                }
                puzzle_down[col].push(ch);
            }
        }

        println!("Checking diagonals");
        for row in 0..puzzle.len() - 3 {
            println!("Row {row}");
            for col in 0..puzzle[row].len() - 3 {
                // check diagonals
                let mut down_right = String::new();
                let mut down_left = String::new();
                for i in 0..4 {
                    down_right.push(puzzle[row+i].chars().nth(col+i).unwrap());
                    down_left.push(puzzle[row+i].chars().nth(col+3-i).unwrap());
                }
                println!("{down_right} and {down_left}");
                if down_right == WORD_F || down_right == WORD_R {
                    sum += 1;
                    println!("+1");
                }
                if down_left == WORD_F || down_left == WORD_R {
                    sum += 1;
                    println!("+1");
                }
            }            
        }

        println!("Checking columns");
        for down_row in puzzle_down {
            sum += check_line(&down_row);
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let puzzle: Vec<&str> = input.lines().collect();
        let mut sum = 0;
        for row in 0..puzzle.len() - 2 {
            for col in 0..puzzle[row].len() - 2 {
                if check_x(&puzzle[row..], col) {
                    sum += 1;
                }
            }
        }
        Ok(sum.to_string())
    }

}