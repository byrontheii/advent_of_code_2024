use std::{collections::HashMap, error, fs};

pub struct Day {
    pub file_path: String
}

struct Region {
    plant: char,
    area: u64,
    perimeter: u64,
}

impl Region {
    fn new(plant: char) -> Region {
        Region {
            plant,
            area: 0,
            perimeter: 0,
        }
    }

    fn cost(&self) -> u64 {
        self.area * self.perimeter
    }
}

fn dfs(row: usize, col: usize, region_id: usize, map: &Vec<Vec<char>>, regions: &mut HashMap<(usize, usize), usize>) -> bool {
    //println!("DFS at ({row}, {col})");
    if regions.contains_key(&(row, col)) {
        //println!("Already in a region");
        return false;
    }
    regions.insert((row, col), region_id);
    let color = map[row][col];
    // up
    if row >= 1 && map[row - 1][col] == color {
        dfs(row - 1, col, region_id, map, regions);
    }
    // right
    if col < map[row].len() - 1 && map[row][col + 1] == color {
        dfs(row, col + 1, region_id, map, regions);
    }
    // down
    if row < map.len() - 1 && map[row + 1][col] == color {
        dfs(row + 1, col, region_id, map, regions);
    }
    // left
    if col >= 1 && map[row][col - 1] == color {
        dfs(row, col - 1, region_id, map, regions);
    }
    true      
}

fn count_edges(row: usize, col: usize, map: &Vec<Vec<char>>, regions: &HashMap<(usize, usize), usize>) -> u64 {
    let mut sum = 0u64;
    let region_id = regions[&(row, col)];
    // up
    if row == 0 || regions[&(row - 1, col)] != region_id {
        sum += 1;
    }
    // right
    if col == map[row].len() - 1 || regions[&(row, col + 1)] != region_id {
        sum += 1;
    }
    // down
    if row == map.len() - 1 || regions[&(row + 1, col)] != region_id {
        sum += 1;
    }
    // left
    if col == 0 || regions[&(row, col - 1)] != region_id {
        sum += 1;
    }    
    sum
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn has_fence(row: usize, col: usize, dir: Direction, map: &Vec<Vec<char>>, regions: &HashMap<(usize, usize), usize>) -> bool {
    let region_id = regions[&(row, col)];
    match dir {
        Direction::UP => row == 0 || regions[&(row - 1, col)] != region_id,
        Direction::RIGHT => col == map[row].len() - 1 || regions[&(row, col + 1)] != region_id,
        Direction::DOWN => row == map.len() - 1 || regions[&(row + 1, col)] != region_id,
        Direction::LEFT => col == 0 || regions[&(row, col - 1)] != region_id,
    }
}

impl super::Runner for Day {

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut new_row: Vec<char> = Vec::new();
            for ch in line.chars() {
                new_row.push(ch);
            }
            map.push(new_row);
        }
        let mut plots_to_regions: HashMap<(usize, usize), usize> = HashMap::new();
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                //println!("**********Starting DFS from ({row}, {col})");
                // depth first search to find everything in a region with id X
                dfs(row, col, plots_to_regions.len(), &mut map, &mut plots_to_regions);
            }
        }
        let mut regions: HashMap<usize, Region> = HashMap::new();
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                let region_id = plots_to_regions[&(row, col)];
                let edges = count_edges(row, col, &map, &plots_to_regions);
                let plant = map[row][col];
                if let Some(region) = regions.get_mut(&region_id) {
                    region.area += 1;
                    region.perimeter += edges;
                }
                else {
                    regions.insert(region_id, Region { plant, area: 1, perimeter: edges});
                }
            }
            println!("|");
        }
        let mut sum = 0u64;
        for (id, region) in regions {
            println!("Region {id} of {} plants with price {} * {} = {}", region.plant, region.area, region.perimeter, region.cost());
            sum += region.cost();
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut new_row: Vec<char> = Vec::new();
            for ch in line.chars() {
                new_row.push(ch);
            }
            map.push(new_row);
        }
        let mut plots_to_regions: HashMap<(usize, usize), usize> = HashMap::new();
        let mut regions: HashMap<usize, Region> = HashMap::new();
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                println!("**********Starting DFS from ({row}, {col})");
                let new_region_id = regions.len();
                // depth first search to find everything in a region with id X
                if dfs(row, col, new_region_id, &mut map, &mut plots_to_regions) {
                    println!("Creating region {new_region_id}");
                    regions.insert(new_region_id, Region::new(map[row][col]));
                }                
            }
        }
        
        // Count the number of horizontal fence pieces above and below each row 
        for row in 0..map.len() {
            let mut upper_fence_region: Option<usize> = None;
            let mut lower_fence_region: Option<usize> = None;
            for col in 0..map[row].len() {
                let region_id = plots_to_regions[&(row, col)];
                println!("Current region is {region_id}");
                regions.get_mut(&region_id).unwrap().area += 1;
                // if a new upper fence starts here, increment that region's side count
                if has_fence(row, col, Direction::UP, &map, &plots_to_regions)  {
                    if upper_fence_region.is_none() || upper_fence_region.unwrap() != region_id {
                        println!("Starting horizontal upper fence at ({row}, {col})");
                        regions.get_mut(&region_id).unwrap().perimeter += 1;
                        upper_fence_region = Some(region_id);
                    }
                }
                else {
                    println!("Ending horizontal upper fence at ({row}, {col})");
                    upper_fence_region = None;
                }
                // if a new lower fence starts here, increment that region's side count
                if has_fence(row, col, Direction::DOWN, &map, &plots_to_regions) {
                    if lower_fence_region.is_none() || lower_fence_region.unwrap() != region_id {
                        println!("Starting horizontal lower fence at ({row}, {col})");
                        regions.get_mut(&region_id).unwrap().perimeter += 1;
                        lower_fence_region = Some(region_id);
                    }
                }
                else {
                    println!("Ending horizontal lower fence at ({row}, {col})");
                    lower_fence_region = None;
                }
            }
        }
        // Count the number of vertical fence pieces left and right of each column
        for col in 0..map[0].len() {
            let mut left_fence_region: Option<usize> = None;
            let mut right_fence_region: Option<usize> = None;
            for row in 0..map.len() {
                let region_id = plots_to_regions[&(row, col)];
                // don't increment area since we already hit every plot in the horizontal sweep
                // if a new left fence starts here, increment that region's side count
                if has_fence(row, col, Direction::LEFT, &map, &plots_to_regions) {
                    if left_fence_region.is_none() || left_fence_region.unwrap() != region_id {
                        println!("Starting vertical left fence at ({row}, {col})");
                        regions.get_mut(&region_id).unwrap().perimeter += 1;
                        left_fence_region = Some(region_id);
                    }
                }
                else {
                    println!("Ending vertical left fence at ({row}, {col})");
                    left_fence_region = None;
                }
                // if a new right fence starts here, increment that region's side count
                if has_fence(row, col, Direction::RIGHT, &map, &plots_to_regions) {
                    if right_fence_region.is_none() || right_fence_region.unwrap() != region_id {
                        println!("Starting vertical right fence at ({row}, {col})");
                        regions.get_mut(&region_id).unwrap().perimeter += 1;
                        right_fence_region = Some(region_id);
                    }
                }
                else {
                    println!("Ending vertical right fence at ({row}, {col})");
                    right_fence_region = None;
                }
            }
        }
        let mut sum = 0u64;
        for (id, region) in regions {
            println!("Region {id} of {} plants with price {} (area) * {} (sides) = {}", region.plant, region.area, region.perimeter, region.cost());
            sum += region.cost();
        }
        Ok(sum.to_string())
    }

}