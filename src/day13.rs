use std::{cmp, error, fs};

pub struct Day {
    pub file_path: String
}

struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn get_assignment(assign_str: &str, op: char) -> i64 {
    let parts: Vec<&str> = assign_str.split(op).collect();
    parts[1].parse().unwrap()
}

fn calc_min_tokens(machine: Machine) -> Option<i64> {
    let mut best_cost = 401i64;  // worst possible cost is 100 * 1 + 100 * 3 = 400
    for num_a in 0..100 {
        let pos_a = (machine.button_a.0 * num_a, machine.button_a.1 * num_a);
        let diff = (machine.prize.0 - pos_a.0, machine.prize.1 - pos_a.1);
        if diff.0 % machine.button_b.0 == 0 && diff.1 % machine.button_b.1 == 0 {
            let num_b = diff.0 / machine.button_b.0;
            let final_pos = (0i64, diff.1 - machine.button_b.1 * num_b);
            if final_pos == (0, 0) {
                best_cost = cmp::min(best_cost, num_a * 3 + num_b);
            }
        }        
    }
    if best_cost < 401i64 { Some(best_cost) } else { None }
}

fn calc_min_tokens_advanced(machine: Machine) -> Option<i64> {
    let denom = machine.button_a.0 * machine.button_b.1 - machine.button_b.0 * machine.button_a.1;
    let numer_a = machine.prize.0 * machine.button_b.1 - machine.prize.1 * machine.button_b.0;
    let numer_b = machine.prize.1 * machine.button_a.0 - machine.prize.0 * machine.button_a.1;
    if numer_a % denom != 0 || numer_b % denom != 0 {
        None
    }
    else {
        Some((3 * numer_a + numer_b) / denom)
    }
}

const OFFSET: i64 = 10000000000000;

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut machines: Vec<Machine> = Vec::new();
        let mut button_a: (i64, i64) = (0, 0);
        let mut button_b: (i64, i64) = (0, 0);
        let mut prize: (i64, i64) = (0, 0);
        for line in input.lines() {
            let sentence: Vec<&str> = line.split(':').collect();
            if sentence.len() < 2 {
                continue;
            }
            let assigns: Vec<&str> = sentence[1].split(',').map(|s| s.trim()).collect();
            if sentence[0].starts_with("Button A") {
                button_a = (get_assignment(assigns[0], '+'), get_assignment(assigns[1], '+'));
            }
            if sentence[0].starts_with("Button B") {
                button_b = (get_assignment(assigns[0], '+'), get_assignment(assigns[1], '+'));
            }
            if sentence[0].starts_with("Prize") {
                prize = (get_assignment(assigns[0], '=') + OFFSET, get_assignment(assigns[1], '=') + OFFSET);
                machines.push(Machine { button_a, button_b, prize });
            }
        }
        let mut sum = 0i64;
        for machine in machines {
            if let Some(tokens) = calc_min_tokens(machine) {
                sum += tokens;
            }
        }
        
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut machines: Vec<Machine> = Vec::new();
        let mut button_a: (i64, i64) = (0, 0);
        let mut button_b: (i64, i64) = (0, 0);
        let mut prize: (i64, i64) = (0, 0);
        for line in input.lines() {
            let sentence: Vec<&str> = line.split(':').collect();
            if sentence.len() < 2 {
                continue;
            }
            let assigns: Vec<&str> = sentence[1].split(',').map(|s| s.trim()).collect();
            if sentence[0].starts_with("Button A") {
                button_a = (get_assignment(assigns[0], '+'), get_assignment(assigns[1], '+'));
            }
            if sentence[0].starts_with("Button B") {
                button_b = (get_assignment(assigns[0], '+'), get_assignment(assigns[1], '+'));
            }
            if sentence[0].starts_with("Prize") {
                prize = (get_assignment(assigns[0], '=') + OFFSET, get_assignment(assigns[1], '=') + OFFSET);
                machines.push(Machine { button_a, button_b, prize });
            }
        }
        let mut sum = 0i64;
        for machine in machines {
            if let Some(tokens) = calc_min_tokens_advanced(machine) {
                sum += tokens;
            }
        }
        
        Ok(sum.to_string())
    }

}