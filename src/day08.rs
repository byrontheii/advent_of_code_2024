use std::{collections::HashMap, error, fs};

pub struct Day {
    pub file_path: String
}

struct Loc {
    antinode: bool,
}

impl Loc {

    fn new(antinode: bool) -> Loc {
        Loc{antinode}
    }
}

fn exists(loc: (isize, isize), map: &Vec<Vec<Loc>>) -> bool {
    loc.0 >= 0 && loc.0 < map.len().try_into().unwrap() && loc.1 >= 0 && loc.1 < map[loc.0 as usize].len() as isize
}

fn add_antinode(loc: (isize, isize), map: &mut Vec<Vec<Loc>>) -> bool {
    if exists(loc, map) {
        //println!("{:?} is on the map", loc);
        let loc: (usize, usize) = (loc.0.try_into().unwrap(), loc.1.try_into().unwrap());
        if !map[loc.0][loc.1].antinode {
            map[loc.0][loc.1].antinode = true;
            //println!("Added antinode");
            return true;
        }
    }
    false
}

fn add_resonant_antinodes(loc: &(isize, isize), diff: &(isize, isize), map: &mut Vec<Vec<Loc>>) -> i32 {
    let norm_col_diff = diff.1 as f32 / diff.0 as f32;
    let mut curr_col_loc = loc.1 as f32 - loc.0 as f32 * norm_col_diff;
    let mut num_adds = 0;
    for i in 0..map.len() {
        //println!("{:?} rounds to {:?}, diff={}", (i, curr_col_loc), (i, curr_col_loc.round()), curr_col_loc - curr_col_loc.round());
        if (curr_col_loc - curr_col_loc.round()).abs() <= 0.0001 {
            let loc = (i as isize, curr_col_loc.round() as isize);
            //println!("Trying to add {:?}", loc);
            if add_antinode(loc, map) {
                num_adds += 1;
            }
        }
        curr_col_loc += norm_col_diff;
    }
    num_adds
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut map: Vec<Vec<Loc>> = Vec::new();
        let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            let mut map_row: Vec<Loc> = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                map_row.push(Loc::new(false));
                if ch != '.' {
                    match antennas.get_mut(&ch) {
                        None => antennas.insert(ch, vec![(row as isize, col as isize)]),
                        Some(v) => {v.push((row as isize, col as isize)); None},
                    };
                }
            }
            map.push(map_row);
        }

        let mut sum = 0;
        for (freq, positions) in antennas {
            println!("Checking frequency {freq}");
            for (i, left_pos) in positions.iter().enumerate() {
                for right_pos in &positions[i+1..] {
                    let diff = (right_pos.0 - left_pos.0, right_pos.1 - left_pos.1);
                    let try_loc_left = (left_pos.0 - diff.0, left_pos.1 - diff.1);
                    let try_loc_right = (right_pos.0 + diff.0, right_pos.1 + diff.1);
                    //println!("{:?} - {:?} = {:?} -> {:?}, {:?}", right_pos, left_pos, diff, try_loc_left, try_loc_right);
                    if add_antinode(try_loc_left, &mut map) {
                        sum += 1;
                    }
                    if add_antinode(try_loc_right, &mut map) {
                        sum += 1;
                    }
                }
            }
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut map: Vec<Vec<Loc>> = Vec::new();
        let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            let mut map_row: Vec<Loc> = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                map_row.push(Loc::new(false));
                if ch != '.' {
                    match antennas.get_mut(&ch) {
                        None => antennas.insert(ch, vec![(row as isize, col as isize)]),
                        Some(v) => {v.push((row as isize, col as isize)); None},
                    };
                }
            }
            map.push(map_row);
        }

        let mut sum = 0;
        for (freq, positions) in antennas {
            println!("Checking frequency {freq}");
            for (i, left_pos) in positions.iter().enumerate() {
                for right_pos in &positions[i+1..] {
                    let diff = (right_pos.0 - left_pos.0, right_pos.1 - left_pos.1);
                    //println!("{:?} to {:?}: {:?}", left_pos, right_pos, diff);
                    sum += add_resonant_antinodes(left_pos, &diff, &mut map);
                }
            }
        }

        // for row in map {
        //     for loc in row {
        //         print!("{}", if let Some(a) = loc.antenna {a} else if loc.antinode {'#'} else {'.'});
        //     }
        //     println!();
        // }
        Ok(sum.to_string())
    }

}