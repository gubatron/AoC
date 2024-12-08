use aoc::utils::load_input_as_char_matrix;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Clone)]
struct Guard {
    col: usize,
    row: usize,
    dir: char,
    visited: HashSet<(usize, usize)>,
    finished: bool,
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
    fn next_step(&mut self, map: &Vec<Vec<char>>) {
        let row_count = map.len();
        let col_count = map[0].len();

        loop {
            if self.dir == '^' {
                // about to leave the map up
                if self.row == 0 {
                    self.finished = true;
                    break;
                }

                // check if next step is a wall
                if map[self.row - 1][self.col] == '#' {
                    self.dir = '>';
                    break;
                }

                self.row -= 1;
            } else if self.dir == '>' {
                // about to leave the map right
                if self.col == col_count - 1 {
                    self.finished = true;
                    break;
                }

                // check if next step is a wall
                if map[self.row][self.col + 1] == '#' {
                    self.dir = 'v';
                    break;
                }

                self.col += 1;
            } else if self.dir == 'v' {
                // about to leave the map down
                if self.row == row_count - 1 {
                    self.finished = true;
                    break;
                }

                // check if next step is a wall
                if map[self.row + 1][self.col] == '#' {
                    self.dir = '<';
                    break;
                }

                self.row += 1;
            } else if self.dir == '<' {
                // about to leave the map left
                if self.col == 0 {
                    self.finished = true;
                    break;
                }

                // check if next step is a wall
                if map[self.row][self.col - 1] == '#' {
                    self.dir = '^';
                    break;
                }

                self.col -= 1;
            }
            self.visited.insert((self.col, self.row));
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
            if guard.visited.contains(&(col_num, row_num)) {
                print!(" X ");
            } else {
                print!(" {} ", col);
            }
        }
        println!();
    }
}

fn pinpoint_guard(map: &Vec<Vec<char>>) -> Guard {
    let mut guard = Guard {
        col: 0,
        row: 0,
        dir: '^',
        visited: HashSet::new(),
        finished: false,
    };
    for (row, row_vec) in map.iter().enumerate() {
        for (col, direction) in row_vec.iter().enumerate() {
            if *direction == '^' || *direction == 'v' || *direction == '<' || *direction == '>' {
                guard.col = col;
                guard.row = row;
                guard.dir = *direction;
                guard.visited.insert((col, row));
                return guard;
            }
        }
    }
    guard
}

fn part1(mut guard: Guard, map: &Vec<Vec<char>>) -> usize {
    while !guard.finished {
        guard.next_step(map);
    }
    print_map_and_visited(map, &guard);
    guard.num_distinct_visited()
}

fn main() {
    println!("Day 6!");
    let map = load_input_as_char_matrix("6.txt");
    let guard = pinpoint_guard(&map);
    println!("{}", guard);
    print_map(&map);

    // Part 1: 4939
    println!("Part 1: {}", part1(guard.clone(), &map));
}
