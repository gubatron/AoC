use aoc::utils::load_input_lines_as_vec_str;
use std::collections::HashSet;

fn count_and_find_matches(pin: &str, grid: &Vec<Vec<char>>) -> (usize, HashSet<(usize, usize)>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let pin_length = pin.len();
    let directions = [
        (0, 1),   // Horizontal (right)
        (0, -1),  // Horizontal (left)
        (1, 0),   // Vertical (down)
        (-1, 0),  // Vertical (up)
        (1, 1),   // Diagonal (down-right)
        (-1, -1), // Diagonal (up-left)
        (1, -1),  // Diagonal (down-left)
        (-1, 1),  // Diagonal (up-right)
    ];

    let is_valid = |x: isize, y: isize| -> bool {
        x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
    };

    let check_direction = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        (0..pin_length).all(|k| {
            let nx = x as isize + k as isize * dx;
            let ny = y as isize + k as isize * dy;
            is_valid(nx, ny) && grid[nx as usize][ny as usize] == pin.chars().nth(k).unwrap()
        })
    };

    let mut matches = HashSet::new();
    let mut match_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if check_direction(i, j, dx, dy) {
                    match_count += 1;
                    for k in 0..pin_length {
                        let nx = i as isize + k as isize * dx;
                        let ny = j as isize + k as isize * dy;
                        matches.insert((nx as usize, ny as usize));
                    }
                }
            }
        }
    }

    (match_count, matches)
}

fn print_grid_with_matches(grid: &Vec<Vec<char>>, matches: &HashSet<(usize, usize)>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if matches.contains(&(i, j)) {
                print!("{}", grid[i][j]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// Part 1: 2662
fn part1(pin: &str, haystack_grid: &Vec<Vec<char>>) -> usize {
    let (match_count, matches) = count_and_find_matches(pin, haystack_grid);
    //print_grid_with_matches(&haystack_grid, &matches);
    match_count
}

fn main() {
    // let grid = vec![
    //     vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
    //     vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
    //     vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
    //     vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
    //     vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
    //     vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
    //     vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
    //     vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
    //     vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
    //     vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    // ];
    let input = load_input_lines_as_vec_str("4.txt");
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    println!("Part 1: {}", part1("XMAS", &grid));
}
