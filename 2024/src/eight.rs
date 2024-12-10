use aoc::utils::{load_input_as_char_matrix, Coord};
use std::collections::{HashMap, HashSet};

fn collect_antennas_by_frequency(
    input: &Vec<Vec<char>>,
    antennas: &mut HashMap<char, Vec<Coord>>,
    rows: usize,
    cols: usize,
) {
    for y in 0..rows {
        for x in 0..cols {
            let c = input[y][x];
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Coord::new(x as i32, y as i32));
            }
        }
    }
}

// Check if a coordinate is within the grid bounds
fn is_within_bounds(coord: Coord, rows: usize, cols: usize) -> bool {
    coord.x >= 0 && coord.y >= 0 && coord.x < cols as i32 && coord.y < rows as i32
}

// Calculate antinodes based on problem constraints
fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    let rows = input.len();
    let cols = input[0].len();

    // Collect antenna positions grouped by frequency
    collect_antennas_by_frequency(input, &mut antennas, rows, cols);

    let mut antinodes: HashSet<Coord> = HashSet::new();

    // For each frequency, calculate antinodes
    for (_, positions) in antennas {
        let n = positions.len();
        for i in 0..n {
            for j in i + 1..n {
                let a = positions[i];
                let b = positions[j];

                // Compute the difference vector
                let dx = b.x - a.x;
                let dy = b.y - a.y;

                // Compute the antinode positions by mirroring
                let antinode1 = Coord::new(a.x - dx, a.y - dy);
                let antinode2 = Coord::new(b.x + dx, b.y + dy);

                // Validate and insert antinodes
                if is_within_bounds(antinode1, rows, cols) {
                    antinodes.insert(antinode1);
                }
                if is_within_bounds(antinode2, rows, cols) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes.len()
}

// Calculate antinodes based on the updated model
fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
    let rows = input.len();
    let cols = input[0].len();

    // Collect antenna positions grouped by frequency
    collect_antennas_by_frequency(input, &mut antennas, rows, cols);

    let mut antinodes: HashSet<Coord> = HashSet::new();

    // For each frequency, calculate antinodes
    for (_, positions) in antennas {
        let n = positions.len();

        // Include all antennas as antinodes
        for &antenna in &positions {
            antinodes.insert(antenna);
        }

        // Process all pairs of antennas
        for i in 0..n {
            for j in i + 1..n {
                let a = positions[i];
                let b = positions[j];

                // Compute the difference vector
                let dx = b.x - a.x;
                let dy = b.y - a.y;

                // Extend outward from a in the direction of -dx, -dy
                let mut current = Coord::new(a.x - dx, a.y - dy);
                while is_within_bounds(current, rows, cols) {
                    antinodes.insert(current);
                    current = Coord::new(current.x - dx, current.y - dy);
                }

                // Extend outward from b in the direction of +dx, +dy
                let mut current = Coord::new(b.x + dx, b.y + dy);
                while is_within_bounds(current, rows, cols) {
                    antinodes.insert(current);
                    current = Coord::new(current.x + dx, current.y + dy);
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = load_input_as_char_matrix("inputs/8.txt");
    println!("Part 1: {}", part1(&input)); // Part 1: 252
    println!("Part 2: {}", part2(&input)); // Part 2: 839
}
