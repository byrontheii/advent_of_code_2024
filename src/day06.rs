use std::{collections::HashSet, error, fs};

pub struct Day {
    pub file_path: String
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_right(dir: Direction) -> Direction {
        match dir {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}


#[derive(Clone)]
struct Cell {
    blocked: bool,
    visited: bool,
}

impl Cell {
    fn new(blocked: bool) -> Cell {
        Cell{blocked: blocked, visited: false}
    }
}

#[derive(Clone)]
struct Map {
    cells: Vec<Vec<Cell>>
}

fn next_pos(pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::UP => (pos.0, pos.1 - 1),
        Direction::DOWN => (pos.0, pos.1 + 1),
        Direction::LEFT => (pos.0 - 1, pos.1),
        Direction::RIGHT => (pos.0 + 1, pos.1),
    }
}

fn has_loop(map: &Map, path: &mut Vec<(i32, i32, Direction)>) -> bool {
    let mut state: (i32, i32, Direction) = path.last().unwrap().clone();
    let max_x = map.cells[0].len() - 1;
    let max_y = map.cells.len() - 1;
    // mark whole path except the last step (which we'll add in the loop) as visited
    let mut visited: HashSet<(i32, i32, Direction)> = path[..path.len() - 1].into_iter().map(|s| s.clone()).collect();
    //println!("Loaded original path:");
    // for (chk_col, chk_row, chk_dir) in &path[..path.len() - 1] {
    //     println!("({chk_col}, {chk_row}) {:?}", chk_dir);
    // }
    
    //println!("Checking remaining path:");
    loop {
        //let (col, row): (usize, usize) = (state.0.try_into().unwrap(), state.1.try_into().unwrap());
        //println!("({col}, {row}) {:?}", state.2);
        
        if visited.contains(&state){
            // we've been here before, so we're in a loop
            //println!("Revisited--in a loop!");
            return true;
        }
        visited.insert((state.0, state.1, state.2.clone()));
        
        let potential_pos: (i32, i32) = next_pos((state.0, state.1), &state.2);        
        if potential_pos.0 < 0 || potential_pos.0 > max_x.try_into().unwrap() || potential_pos.1 < 0 || potential_pos.1 > max_y.try_into().unwrap() {
            return false; // exited the area
        }
        let ( col, row): (usize, usize) = (potential_pos.0.try_into().unwrap(), potential_pos.1.try_into().unwrap());
        let next_cell: &Cell = &map.cells[row][col];
        if next_cell.blocked {
            state.2 = Direction::turn_right(state.2);
        }
        else {
            state = (potential_pos.0, potential_pos.1, state.2.clone());
        }
        path.push(state);
    }        
    
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut sum = 0;
        let mut direction = Direction::UP;
        let mut pos: (i32, i32) = (0, 0);
        let mut map: Vec<Vec<Cell>> = Vec::new();
        for (row, line) in input.lines().enumerate() {
            map.push(Vec::new());
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' | '^' => map.last_mut().unwrap().push(Cell::new(false)),
                    '#' => map.last_mut().unwrap().push(Cell::new(true)),
                    _ => panic!("Unrecognized input symbol {ch}"),
                }
                if ch == '^' {
                    pos = (col.try_into().unwrap(), row.try_into().unwrap());
                }
            }
        }

        let max_x = map[0].len() - 1;
        let max_y = map.len() - 1;
        loop {
            let (row, col): (usize, usize) = (pos.1.try_into().unwrap(), pos.0.try_into().unwrap());
            println!("({col}, {row}) {:?}", direction);
            let current_cell: &mut Cell = &mut map[row][col];
            if !current_cell.visited {
                sum += 1;
            }
            current_cell.visited = true;
            let potential_pos: (i32, i32) = next_pos(pos, &direction);
            if potential_pos.0 < 0 || potential_pos.0 > max_x.try_into().unwrap() || potential_pos.1 < 0 || potential_pos.1 > max_y.try_into().unwrap() {
                break;
            }
            let (row, col): (usize, usize) = (potential_pos.1.try_into().unwrap(), potential_pos.0.try_into().unwrap());
            let next_cell: &Cell = &map[row][col];
            if next_cell.blocked {
                direction = Direction::turn_right(direction);
            }
            else {
                pos = potential_pos;
            }
        }        

        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut sum = 0;
        let mut start_pos: (i32, i32) = (0, 0);
        let mut init_map = Map{cells: Vec::new()};
        for (row, line) in input.lines().enumerate() {
            init_map.cells.push(Vec::new());
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '.' | '^' => init_map.cells.last_mut().unwrap().push(Cell::new(false)),
                    '#' => init_map.cells.last_mut().unwrap().push(Cell::new(true)),
                    _ => panic!("Unrecognized input symbol {ch}"),
                }
                if ch == '^' {
                    start_pos = (col.try_into().unwrap(), row.try_into().unwrap());
                }
            }
        }

        // build path without extra obstacles
        let mut init_path: Vec<(i32, i32, Direction)> = Vec::new();
        init_path.push((start_pos.0, start_pos.1, Direction::UP));
        assert!(!has_loop(&mut init_map, &mut init_path));
        let mut checked_cells: HashSet<(i32, i32)> = HashSet::new();

        for (start_index, state) in init_path[1..].iter().enumerate() {
            if checked_cells.contains(&(state.0, state.1)) {
                continue;
            }
            println!("Checking ({}, {})", state.0, state.1);
            // copy the path from 0 to the cell prior to where the obstacle will be inserted
            let mut path = init_path[..=start_index].iter().map(|s| s.clone()).collect();
            let mut map = init_map.clone();
            let (row, col): (usize, usize) = (state.1.try_into().unwrap(), state.0.try_into().unwrap());
            let cell_to_mod: &mut Cell = &mut map.cells[row][col];
            cell_to_mod.blocked = true;
            if has_loop(&mut map, &mut path) {
                sum += 1;
            }
            checked_cells.insert((state.0, state.1));
        }

        Ok(sum.to_string())
    }

}