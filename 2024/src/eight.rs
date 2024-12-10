use aoc::utils::{load_input_as_char_matrix, Coord};
use std::collections::{HashMap, HashSet};

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

                // println!(
                //     "Pair: {:?} and {:?}, Antinodes: {:?}, {:?}",
                //     a, b, antinode1, antinode2
                // );
            }
        }
    }

    //println!("Validated Antinodes: {:?}", antinodes);
    antinodes.len()
}

fn main() {
    let input = load_input_as_char_matrix("8.txt");
    println!("Part 1: {}", part1(&input)); // Part 1: 252
}
