use std::{collections::HashMap, error, fs};

pub struct Day {
    pub file_path: String,
    pub num_blinks: u8,
}

fn blink_recursive(stone: u64, blinks: u8, shortcuts: &mut HashMap<(u64, u8), u64>) -> u64 {
    // if we can look up the answer in shortcuts, just return that
    if blinks == 0 {
        return 1;
    }
    if let Some(sum) = shortcuts.get(&(stone, blinks)) {
        return *sum;
    }
    if stone == 0 {
        let sum = blink_recursive(1, blinks - 1, shortcuts);
        shortcuts.insert((stone, blinks), sum);
        return sum;
    }
    let s_chars: Vec<char> = stone.to_string().chars().collect();
    if s_chars.len() % 2 == 0 {
        let sum =
            blink_recursive(s_chars[..s_chars.len() / 2].iter().collect::<String>().parse().unwrap(), blinks - 1, shortcuts)
            +
            blink_recursive(s_chars[s_chars.len() / 2..].iter().collect::<String>().parse().unwrap(), blinks - 1, shortcuts);
            shortcuts.insert((stone, blinks), sum);
        return sum;
    }
    let sum = blink_recursive(stone * 2024, blinks - 1, shortcuts);
    shortcuts.insert((stone, blinks), sum);
    sum
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut prev_stones: Vec<u64> = Vec::new();
        let mut new_stones: Vec<u64> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for _ in 0..self.num_blinks {
            std::mem::swap(&mut prev_stones, &mut new_stones);
            new_stones.clear();
            //println!("{:?}", prev_stones);
            for s in &prev_stones {
                if *s == 0 {
                    //println!("zero");
                    new_stones.push(1);
                    continue;
                }
                let s_chars: Vec<char> = s.to_string().chars().collect();
                if s_chars.len() % 2 == 0 {
                    //println!("even");
                    new_stones.push(s_chars[..s_chars.len() / 2].iter().collect::<String>().parse().unwrap());
                    new_stones.push(s_chars[s_chars.len() / 2..].iter().collect::<String>().parse().unwrap());
                    continue;
                }
                //println!("mult");
                new_stones.push(*s * 2024);
            }            
        }
        //println!("{:?}", new_stones);
        Ok(new_stones.len().to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let init_stones: Vec<(u64, u8)> = input.split_whitespace().map(|s| (s.parse::<u64>().unwrap(), self.num_blinks)).collect();
        let mut sum = 0u64;
        let mut shortcuts: HashMap<(u64, u8), u64> = HashMap::new();
        for (stone, blinks) in init_stones {
            sum += blink_recursive(stone, blinks, &mut shortcuts);
        }
        Ok(sum.to_string())
    }
}
