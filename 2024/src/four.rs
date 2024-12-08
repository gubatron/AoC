use aoc::utils::load_input_as_char_matrix;
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


//fn print_grid_with_matches(grid: &Vec<Vec<char>>, matches: &HashSet<(usize, usize)>) {
//    for i in 0..grid.len() {
//        for j in 0..grid[i].len() {
//            if matches.contains(&(i, j)) {
//                print!("{}", grid[i][j]);
//            } else {
//                print!(".");
//            }
//        }
//        println!();
//    }
//}

fn count_x_mas_patterns(haystack_grid: &Vec<Vec<char>>) -> usize {
    let rows = haystack_grid.len();
    let cols = haystack_grid[0].len();
    let mut pattern_count = 0;

    let is_x_mas = |x: usize, y: usize| -> bool {
        if x == 0 || x >= rows - 1 || y == 0 || y >= cols - 1 {
            return false; // Out of bounds for the pattern
        }

        let top_left = haystack_grid[x - 1][y - 1];
        let bottom_right = haystack_grid[x + 1][y + 1];
        let top_right = haystack_grid[x - 1][y + 1];
        let bottom_left = haystack_grid[x + 1][y - 1];
        let center = haystack_grid[x][y];

        println!(
            "Checking ({}, {}): TL={}, BR={}, TR={}, BL={}, C={}",
            x, y, top_left, bottom_right, top_right, bottom_left, center
        );

        center == 'A'
            && ((top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'M')
                || (top_left == 'S'
                    && top_right == 'M'
                    && bottom_left == 'S'
                    && bottom_right == 'M')
                || (top_left == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'S'))
    };

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if is_x_mas(i, j) {
                println!("Pattern found at ({}, {})", i, j);
                pattern_count += 1;
            }
        }
    }

    pattern_count
}

#[test]
fn test_count_x_mas_patterns() {
    let grid1 = vec![
        vec!['.', 'M', '.', 'S', '.'],
        vec!['.', '.', 'A', '.', '.'],
        vec!['.', 'M', '.', 'S', '.'],
    ];
    let grid2 = vec![
        vec!['.', 'S', '.', 'M', '.'],
        vec!['.', '.', 'A', '.', '.'],
        vec!['.', 'S', '.', 'M', '.'],
    ];
    let grid3 = vec![
        vec!['.', 'S', '.', 'S', '.'],
        vec!['.', '.', 'A', '.', '.'],
        vec!['.', 'M', '.', 'M', '.'],
    ];
    let grid4 = vec![
        vec!['.', 'M', '.', 'M', '.'],
        vec!['.', '.', 'A', '.', '.'],
        vec!['.', 'S', '.', 'S', '.'],
    ];

    assert_eq!(count_x_mas_patterns(&grid1), 1);
    assert_eq!(count_x_mas_patterns(&grid2), 1);
    assert_eq!(count_x_mas_patterns(&grid3), 1);
    assert_eq!(count_x_mas_patterns(&grid4), 1);
}

// Part 1: 2662
fn part1(pin: &str, haystack_grid: &Vec<Vec<char>>) -> usize {
    let (match_count, _matches) = count_and_find_matches(pin, haystack_grid);
    //print_grid_with_matches(&haystack_grid, &matches);
    match_count
}

fn part2(haystack_grid: &Vec<Vec<char>>) -> usize {
    count_x_mas_patterns(haystack_grid)
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
    let haystack: Vec<Vec<char>> = load_input_as_char_matrix("4.txt");
    println!("Part 1: {}", part1("XMAS", &haystack));
    println!("Part 2: {}", part2(&haystack));
}
