use std::{collections::HashSet, error, fs, iter::Enumerate};

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_right(dir: Direction) -> Option<Direction> {
        match dir {
            Direction::UP => Some(Direction::RIGHT),
            Direction::RIGHT => Some(Direction::DOWN),
            Direction::DOWN => Some(Direction::LEFT),
            Direction::LEFT => None,
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT]
    }
}

struct Step {
    row: usize,
    col: usize,
    height: usize,
    dir: Direction,  // next direction to be taken
}

struct Grid {
    locs: Vec<Vec<usize>>,
}

impl Grid {
    fn new() -> Grid {
        Grid { locs: Vec::new() }
    }

    fn trailheads(&self) -> impl Iterator<Item = (usize, usize, usize)> + use<'_> {
        self.locs_iter().filter(|loc| loc.2 == 0usize)
    }

    fn locs_iter(&self) -> LocIter {
        LocIter{row: 0, row_iter: self.locs.iter().enumerate(), col_iter: None}
    }

    fn trail_score(&self, row: usize, col: usize) -> usize {
        let mut path = Vec::<Step>::new();
        let mut trail_ends = HashSet::<(usize, usize)>::new();
        path.push(Step{row, col, height: self.locs[row][col], dir: Direction::UP});
        let mut iters = 0;

        loop {
            iters += 1;
            if let Some(mut step) = path.pop() {
                if step.height == 9 {
                    trail_ends.insert((step.row, step.col));
                    continue;
                }
                loop {  // TODO: iterate here until we find a usable direction
                    // try to extend the path in the current direction
                    if let Some(next_loc) = self.try_move(step.row, step.col, step.dir) {
                        let next_step_height = step.height + 1;
                        if self.locs[next_loc.0][next_loc.1] == next_step_height {
                            if let Some(next_dir) = Direction::turn_right(step.dir) {
                                step.dir = next_dir;
                                path.push(step);
                            }
                            path.push(Step{row: next_loc.0, col: next_loc.1, height: next_step_height, dir: Direction::UP});
                            break;
                        }
                    }
                    if let Some(next_dir) = Direction::turn_right(step.dir) {
                        step.dir = next_dir;
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                println!("{:?}", trail_ends.iter().collect::<Vec<_>>());
                return trail_ends.len();
            }
        }
    }

    fn trail_rating(&self, row: usize, col: usize) -> usize {
        let mut path = Vec::<Step>::new();
        let mut trail_ends = HashSet::<(usize, usize)>::new();
        path.push(Step{row, col, height: self.locs[row][col], dir: Direction::UP});
        let mut sum = 0usize;

        // try using a visited list

        loop {
            if let Some(mut step) = path.pop() {
                if step.height == 9 {
                    trail_ends.insert((step.row, step.col));
                    sum += 1;
                    continue;
                }
                loop {
                    // try to extend the path in the current direction
                    if let Some(next_loc) = self.try_move(step.row, step.col, step.dir) {
                        let next_step_height = step.height + 1;
                        if self.locs[next_loc.0][next_loc.1] == next_step_height {
                            if let Some(next_dir) = Direction::turn_right(step.dir) {
                                step.dir = next_dir;
                                path.push(step);
                            }
                            path.push(Step{row: next_loc.0, col: next_loc.1, height: next_step_height, dir: Direction::UP});
                            break;
                        }
                    }
                    if let Some(next_dir) = Direction::turn_right(step.dir) {
                        step.dir = next_dir;
                    }
                    else {
                        break;
                    }
                }
            }
            else {
                println!("{:?}", trail_ends.iter().collect::<Vec<_>>());
                return sum;
            }
        }
    }

    fn try_move(&self, row: usize, col: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::UP => if row <= 0 { None } else  { Some((row - 1, col)) },
            Direction::DOWN => if row >= self.locs.len() - 1 { None } else { Some((row + 1, col)) },
            Direction::LEFT => if col <= 0 { None } else { Some((row, col - 1)) },
            Direction::RIGHT => if col >= self.locs.len() - 1 { None } else { Some((row, col + 1)) },
        }
    }
}

struct LocIter<'a> {
    row: usize,
    row_iter: Enumerate<std::slice::Iter<'a, Vec<usize>>>,
    col_iter: Option<Enumerate<std::slice::Iter<'a, usize>>>,
}

impl<'a> Iterator for LocIter<'a> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut c_it) = self.col_iter {
            if let Some(next_col) = &c_it.next() {
                return Some((self.row, next_col.0, *next_col.1));
            }
        }        
        if let Some((row_i, next_row)) = self.row_iter.next() {
            self.row = row_i;
            self.col_iter = Some(next_row.iter().enumerate());
            return self.next();
        }
        None        
    }
}

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut grid = Grid::new();
        for line in input.lines() {
            let mut row_locs = Vec::<usize>::new();
            for ch in line.chars() {
                row_locs.push(ch.to_string().parse().unwrap());
            }
            grid.locs.push(row_locs);
        }

        let mut sum: u64 = 0;
        for (row, col, height) in grid.trailheads() {
            println!("Trailhead {:?}", (row, col, height));
            sum += grid.trail_score(row, col) as u64;
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut grid = Grid::new();
        for line in input.lines() {
            let mut row_locs = Vec::<usize>::new();
            for ch in line.chars() {
                row_locs.push(ch.to_string().parse().unwrap());
            }
            grid.locs.push(row_locs);
        }

        let mut sum: u64 = 0;
        for (row, col, height) in grid.trailheads() {
            println!("Trailhead {:?}", (row, col, height));
            sum += grid.trail_rating(row, col) as u64;
        }
        Ok(sum.to_string())
    }

}