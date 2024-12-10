use aoc::utils::{dimensions_cols_rows, load_input_as_char_matrix};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone)]
struct Guard {
    col: usize,
    row: usize,
    dir: char,
    visited: HashSet<(usize, usize)>,
    visited_states: HashSet<(usize, usize, char)>, // Track full state
    finished: bool,
    looped: bool,
    origin_col: usize,
    origin_row: usize,
    turns: HashSet<(usize, usize)>,
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Guard: col: {}, row: {}, dir: {}\n       visited = [ ",
            self.col, self.row, self.dir
        )
        .expect("");
        // print visited coords
        for (col, row) in self.visited.iter() {
            write!(f, "({}, {}), ", col, row).expect("");
        }
        write!(f, " ]\n")
    }
}

impl Guard {
    fn next_step(&mut self, map: &mut Vec<Vec<char>>) {
        let row_count = map.len();
        let col_count = map[0].len();

        if !self.finished {
            if self.dir == '^' {
                // about to leave the map up
                if self.row == 0 {
                    map[self.row][self.col] = '^';
                    self.visited.insert((self.col, self.row));
                    self.finished = true;
                } else if map[self.row - 1][self.col] == '#' || map[self.row - 1][self.col] == 'O' {
                    self.dir = '>';
                    self.turns.insert((self.col, self.row));
                } else {
                    map[self.row][self.col] = '|';
                    self.row -= 1;
                }
            } else if self.dir == '>' {
                // about to leave the map right
                if self.col == col_count - 1 {
                    map[self.row][self.col] = '>';
                    self.visited.insert((self.col, self.row));
                    self.finished = true;
                } else if map[self.row][self.col + 1] == '#' || map[self.row][self.col + 1] == 'O' {
                    // check if next step is a wall
                    self.dir = 'v';
                    self.turns.insert((self.col, self.row));
                } else {
                    map[self.row][self.col] = '-';
                    self.col += 1;
                }
            } else if self.dir == 'v' {
                // about to leave the map down
                if self.row == row_count - 1 {
                    map[self.row][self.col] = 'v';
                    self.visited.insert((self.col, self.row));
                    self.finished = true;
                } else if map[self.row + 1][self.col] == '#' || map[self.row + 1][self.col] == 'O' {
                    // check if next step is a wall
                    self.dir = '<';
                    self.turns.insert((self.col, self.row));
                } else {
                    map[self.row][self.col] = '|';
                    self.row += 1;
                }
            } else if self.dir == '<' {
                // about to leave the map left
                if self.col == 0 {
                    map[self.row][self.col] = '<';
                    self.visited.insert((self.col, self.row));
                    self.finished = true;
                } else if map[self.row][self.col - 1] == '#' || map[self.row][self.col - 1] == 'O' {
                    // check if next step is a wall
                    self.dir = '^';
                    self.turns.insert((self.col, self.row));
                } else {
                    map[self.row][self.col] = '-';
                    self.col -= 1;
                }
            }

            if !self.finished {
                self.visited.insert((self.col, self.row));

                // Check if state has been visited before
                let state = (self.col, self.row, self.dir);
                if !self.visited_states.insert(state) {
                    self.finished = true; // Detected a loop
                    self.looped = true;
                }
            }
        }
    }

    fn do_a_round(&mut self, map: &mut Vec<Vec<char>>, print_after_each_step: bool) {
        let mut steps = 0;
        while !self.finished {
            self.next_step(map);
            if print_after_each_step {
                println!("Step: {}", steps);
                print_map_and_visited(&map, &self);
                println!("======");
                steps += 1;
            }
        }
    }

    fn num_distinct_visited(&self) -> usize {
        self.visited.len()
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for col in row.iter() {
            print!(" {} ", col);
        }
        println!();
    }
}

fn print_map_and_visited(map: &Vec<Vec<char>>, guard: &Guard) {
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, col) in row.iter().enumerate() {
            if guard.turns.contains(&(col_num, row_num)) {
                print!(" + ");
            } else if (col_num, row_num) == (guard.col, guard.row) {
                print!(" {} ", guard.dir);
            } else if guard.visited.contains(&(col_num, row_num)) {
                print!(" X ");
            } else {
                print!(" {} ", col);
            }
        }
        println!();
    }
    println!();
}

fn pinpoint_guard(map: &Vec<Vec<char>>) -> Guard {
    let mut guard = Guard {
        col: 0,
        row: 0,
        dir: '^',
        visited: HashSet::new(),
        visited_states: HashSet::new(),
        finished: false,
        looped: false,
        origin_col: 0,
        origin_row: 0,
        turns: HashSet::new(),
    };
    for (row, row_vec) in map.iter().enumerate() {
        for (col, direction) in row_vec.iter().enumerate() {
            if *direction == '^' || *direction == 'v' || *direction == '<' || *direction == '>' {
                guard.col = col;
                guard.row = row;
                guard.dir = *direction;
                guard.origin_col = col;
                guard.origin_row = row;
                guard.visited.insert((col, row));
                return guard;
            }
        }
    }
    guard
}

fn clone_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for row in map.iter() {
        let mut new_row = Vec::new();
        for col in row.iter() {
            new_row.push(*col);
        }
        new_map.push(new_row);
    }
    new_map
}

fn part1(mut guard: Guard, map: &Vec<Vec<char>>, debug: bool) -> usize {
    let mut cloned_map = clone_map(&map);
    guard.do_a_round(&mut cloned_map, debug);
    //print_map_and_visited(&cloned_map, &guard);
    guard.num_distinct_visited()
}

fn part2(guard: Guard, map: &Vec<Vec<char>>, debug: bool) -> usize {
    let mut alt_guard = guard.clone();

    let (cols, rows) = dimensions_cols_rows(&map);
    let mut distinct_obstructions = HashSet::<(usize, usize)>::new();
    for y in 0..rows {
        for x in 0..cols {
            if map[y][x] == '.' {
                let mut alternate_graph = clone_map(&map);
                alternate_graph[y][x] = 'O'; // block the path in this version of the graph

                //println!("Obstructing at ({}, {})", x, y);
                //println!("BEFORE ROUND:");
                //print_map_and_visited(&alternate_graph, &alt_guard);
                // now let's make the guard do his thing
                alt_guard.do_a_round(&mut alternate_graph, false);
                //println!("AFTER ROUND:");
                if alt_guard.looped {
                    distinct_obstructions.insert((x, y));
                    if debug {
                        println!();
                        println!("======");
                        print_map_and_visited(&alternate_graph, &alt_guard);
                        println!("Loop detected, adding obstruction ({},{})", x, y);
                        println!("======");
                    }
                }
            }
            alt_guard = guard.clone();
        }
    }

    if debug {
        print!("{} Obstructions found: ", distinct_obstructions.len());

        distinct_obstructions
            .iter()
            .enumerate()
            .for_each(|(_i, (x, y))| {
                print!("({}, {}), ", x, y);
            });
        println!();
    }
    distinct_obstructions.len()
}

fn main() {
    println!("Day 6!");
    let map = load_input_as_char_matrix("inputs/6.txt");
    let guard = pinpoint_guard(&map);
    // println!("======");
    // println!("{}", guard);
    // print_map(&map);
    // println!("======\n");

    // Part 1: 4939
    println!("Part 1: {}", part1(guard.clone(), &map, false));

    // Part 2: 1434
    println!("Part 2: {}", part2(guard.clone(), &map, false));
}
