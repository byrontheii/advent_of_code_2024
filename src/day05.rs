use std::{cmp::Ordering, error, fs};
use regex::Regex;
use std::collections::HashMap;

pub struct Day {
    pub file_path: String
}

fn check(a: &i32, b: &i32, order_lt: &HashMap<i32, Vec<i32>>, order_gt: &HashMap<i32, Vec<i32>>) -> Ordering {
    if let Some(x) = order_lt.get(a) {
        if x.contains(&b) {
            return Ordering::Less
        }
    }
    if let Some(y) = order_gt.get(b) {
        if y.contains(&a) {
            return Ordering::Greater
        }
    }
    Ordering::Equal    
}

fn new_center(list: &Vec<i32>, order_lt: &HashMap<i32, Vec<i32>>, order_gt: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort_by(|a, b| check(a, b, &order_lt, &order_gt));
    println!("Reordered:");
    println!("{:?} ({})", sorted_list, sorted_list[sorted_list.len() / 2]);
    sorted_list[sorted_list.len() / 2]
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let re_ord = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();
        let re_list = Regex::new(r"[0-9]+,[0-9]+(,[0-9]+)*").unwrap();
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut order: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut sum = 0;

        for (_, [left, right]) in re_ord.captures_iter(&input).map(|c| c.extract()) {
            let from: i32 = left.parse().unwrap();
            let to: i32 = right.parse().unwrap();
            match order.get_mut(&from) {
                Some(r) => r.push(to),
                None => {
                    order.insert(from, vec![to]);
                    ()
                }
            }
        }
        'updates: for capt in re_list.find_iter(&input).map(|c| c.as_str()) {
            let pages: Vec<i32> = capt.split(',').map(|s| s.parse().unwrap()).collect();
            for i in 0..pages.len()-1 {
                for j in i+1..pages.len() {
                    if let Some(val) = order.get(&pages[j]) {
                        if val.contains(&pages[i]) {
                            // the page on the right is ordered before the page on the left
                            continue 'updates;
                        }
                    }
                }
            }
            sum += pages[pages.len() / 2];
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let re_ord = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();
        let re_list = Regex::new(r"[0-9]+,[0-9]+(,[0-9]+)*").unwrap();
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut order_lt: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut order_gt: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut sum = 0;

        for (_, [left, right]) in re_ord.captures_iter(&input).map(|c| c.extract()) {
            let from: i32 = left.parse().unwrap();
            let to: i32 = right.parse().unwrap();
            match order_lt.get_mut(&from) {
                Some(r) => r.push(to),
                None => {
                    order_lt.insert(from, vec![to]);
                    ()
                }
            }
            match order_gt.get_mut(&to) {
                Some(r) => r.push(from),
                None => {
                    order_gt.insert(to, vec![from]);
                    ()
                }
            }
        }
        
        'updates: for capt in re_list.find_iter(&input).map(|c| c.as_str()) {
            let pages: Vec<i32> = capt.split(',').map(|s| s.parse().unwrap()).collect();
            for i in 0..pages.len()-1 {
                println!("{}", pages[i]);
                for j in i+1..pages.len() {
                    if let Some(val) = order_lt.get(&pages[j]) {
                        if val.contains(&pages[i]) {
                            // the page on the right is ordered before the page on the left
                            sum += new_center(&pages, &order_lt, &order_gt);
                            continue 'updates;
                        }
                    }
                }
            }
        }
        Ok(sum.to_string())
    }

}