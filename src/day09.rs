use std::{error, fs};

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut blocks = Vec::<Option<usize>>::new();
        let mut nonempty_count = 0;
        for (i, space) in input.trim().chars().map(|c| c.to_string().parse::<u64>().unwrap()).enumerate() {
            for _ in 0..space {
                blocks.push(if i % 2 == 0 {Some(i/2)} else {None});
                if i%2 == 0 {
                    nonempty_count += 1;
                }
            }
        }

        // condense
        let mut empty_ptr = blocks.iter().position(|b| b.is_none()).unwrap();
        'condenser: loop {
            for i in empty_ptr..empty_ptr+10 {
                if i >= blocks.len() {
                    break;
                }
                print!("({})", if blocks[i].is_none() {".".to_string()} else {blocks[i].unwrap().to_string()});
            }
            println!();
            if let Some(id) = blocks.pop().unwrap() {
                println!("Popped {}; len is now {}, empty_ptr is {}", id, blocks.len(), empty_ptr);
                if empty_ptr >= blocks.len() {
                    blocks.push(Some(id));
                    break 'condenser;
                }
                blocks[empty_ptr] = Some(id);
            }
            else {
                continue;
            }

            loop {
                empty_ptr += 1;
                if empty_ptr >= blocks.len() {
                    break 'condenser;
                }
                if blocks[empty_ptr].is_none() {
                    break;
                }
            }
        }

        println!("Original count: {}, final count: {}", nonempty_count, blocks.len());
        let mut sum: u64 = 0;
        for (i, id) in blocks.iter().enumerate() {
            sum += i as u64 * id.unwrap() as u64;
        }
        
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut blocks = Vec::<Option<usize>>::new();
        let mut full_blocks: Vec<(usize, usize, usize)> = Vec::new();
        let mut even = true;
        for (i, space) in input.trim().chars().map(|c| c.to_string().parse::<usize>().unwrap()).enumerate() {
            let mut entry = None;
            if i%2 == 0 {
                entry = Some(i/2);
                // record id, start index, and end index
                full_blocks.push((i/2, blocks.len(), blocks.len() + space));
            }
            even = i%2 == 0;
            for _ in 0..space { 
                blocks.push(entry);
            }
            
        }
        println!("Total blocks: {}", blocks.len());
        println!("Ended on {}", if even {"full block"} else {"empty block"});

        for (id, block_start, block_end) in full_blocks[1..].iter().rev() {
            //println!("Trying to move {} from [{}, {}]", id, block_start, block_end);
            let mut empty_start = blocks.iter().position(|b| b.is_none()).unwrap();
            if empty_start >= *block_start {
                //println!("Checked everything left of block, moving on");
                break;
            }
            loop {
                let empty_end = empty_start + blocks[empty_start..].iter().position(|b| b.is_some()).unwrap();
                //println!("Considering empty space from {} to {}", empty_start, empty_end);
                if empty_end - empty_start >= block_end - block_start {
                    // move the block to the new location and continue to the next rightmost block
                    for i in 0..block_end - block_start {
                        //println!("Moving {} from {} to {}", blocks[block_start + i].unwrap(), block_start + i, empty_start + i);
                        blocks[empty_start + i] = Some(*id);
                        blocks[block_start + i] = None;
                    }
                    break;
                }
                let new_empty_start = blocks[empty_end..].iter().position(|b| b.is_none());
                if new_empty_start.is_none() {
                    //println!("Checked everything left of block, moving on");
                    break;
                }
                empty_start = new_empty_start.unwrap() + empty_end;
                if empty_start >= *block_start {
                    //println!("Checked everything left of block, moving on");
                    break;
                }
                //println!("New empty start is {empty_start}");
            }
        }

        let mut sum: u64 = 0;
        for (i, block) in blocks.iter().enumerate() {
            if let Some(id) = block {
                //print!("({i}*{id})+");
                sum += i as u64 * *id as u64;
            }
            else {
                //print!("({i}*0)+");
            }
        }
        
        Ok(sum.to_string())
    }

}