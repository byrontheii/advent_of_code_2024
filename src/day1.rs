use std::{error, io};
use std::collections::BinaryHeap;

pub fn run_a(file_name: &String) -> Result<String, Box<dyn error::Error>> {
    let mut ids0: BinaryHeap<u64> = BinaryHeap::new();
    let mut ids1: BinaryHeap<u64> = BinaryHeap::new();
    for line in super::load_file(file_name).lines() {
        let trim_line = line.trim();
        println!("{trim_line}");
        let id_strs: Vec<&str> = trim_line.split_whitespace().collect();

        if trim_line.is_empty() {
            continue;
        }
        
        if id_strs.len() != 2 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Got a line of input that didn't have two items")));
        }

        ids0.push(id_strs[0].parse()?);
        ids1.push(id_strs[1].parse()?);
    }

    println!("In order:");
    let mut sum_of_diffs: u64 = 0;
    for (i0, i1) in ids0.into_sorted_vec().iter().zip(ids1.into_sorted_vec().iter()) {
        println!("{i0}   {i1}");
        sum_of_diffs += i0.abs_diff(*i1);
    }
    
    Ok(sum_of_diffs.to_string())
}
