use std::{error, io};
use std::collections::{BinaryHeap, HashMap};

fn get_pair(line: &str) -> Result<Vec<&str>, Box<dyn error::Error>> {
    let trim_line = line.trim();
    println!("{trim_line}");
    let id_strs: Vec<&str> = trim_line.split_whitespace().collect();
    match id_strs.len() {
        0 | 2 => Ok(id_strs),
        _ => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Got a line of input that didn't have two items"))),
    }
}

pub fn run_a(file_name: &String) -> Result<String, Box<dyn error::Error>> {
    let mut ids0: BinaryHeap<u64> = BinaryHeap::new();
    let mut ids1: BinaryHeap<u64> = BinaryHeap::new();
    for line in super::load_file(file_name).lines() {
        let id_strs = get_pair(line)?;
        if id_strs.len() == 0 {
            continue;
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

pub fn run_b(file_name: &String) -> Result<String, Box<dyn error::Error>> {
    let mut left_list: Vec<u64> = Vec::new();
    let mut right_counts: HashMap<u64, u64> = HashMap::new();
    for line in super::load_file(file_name).lines() {
        let id_strs = get_pair(line)?;
        if id_strs.len() == 0 {
            continue;
        }

        left_list.push(id_strs[0].parse()?);
        let right_id: u64 = id_strs[1].parse()?;
        if let Some(count) = right_counts.get_mut(&right_id) {
            *count += 1;
        }
        else {
            right_counts.insert(right_id, 1);
        }
    }
    
    let mut sim_score: u64 = 0;
    for left_id in left_list {
        if let Some(count) = right_counts.get_mut(&left_id) {
            sim_score += left_id * *count;
        }
    }

    Ok(sim_score.to_string())
}