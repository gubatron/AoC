use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Represents a state in the priority queue.
#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    pos: (i32, i32),
    dir: (i32, i32),
}

/// Implements ordering for the priority queue based on cost.
/// This ensures that the state with the lowest cost is processed first.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to turn BinaryHeap into a min-heap.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Rotates the current direction to the left (90 degrees).
fn rotate_left(dir: (i32, i32)) -> (i32, i32) {
    (-dir.1, dir.0)
}

/// Rotates the current direction to the right (90 degrees).
fn rotate_right(dir: (i32, i32)) -> (i32, i32) {
    (dir.1, -dir.0)
}

/// Performs Dijkstra's algorithm to find the shortest path from `start` to `'E'`.
///
/// # Arguments
///
/// * `start` - The starting position as `(x, y)`.
/// * `initial_dir` - The initial direction as `(dx, dy)`.
/// * `board` - A reference to the grid represented as a `HashMap`.
///
/// # Returns
///
/// * `Some(cost)` if a path is found with the total cost.
/// * `None` if no path exists.
fn dijkstra(start: (i32, i32), initial_dir: (i32, i32), board: &HashMap<(i32, i32), char>) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, pos: start, dir: initial_dir });

    // Tracks the minimum cost to reach each `(pos, dir)` state.
    let mut costs: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();

    while let Some(state) = heap.pop() {
        let cost = state.cost;
        let pos = state.pos;
        let dir = state.dir;

        let key = (pos, dir);

        // If a cheaper cost has already been recorded for this state, skip processing.
        if cost > *costs.get(&key).unwrap_or(&u32::MAX) {
            continue;
        }

        // Record the cost for this state.
        costs.insert(key, cost);

        // Check if the current position is the end.
        if let Some(&c) = board.get(&pos) {
            if c == 'E' {
                return Some(cost);
            }
        } else {
            // If the position is not on the board, skip.
            continue;
        }

        // **Move Forward:**
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if let Some(&c) = board.get(&new_pos) {
            if c == '.' || c == 'E' {
                let new_cost = cost + 1;
                let new_key = (new_pos, dir);
                if new_cost < *costs.get(&new_key).unwrap_or(&u32::MAX) {
                    heap.push(State { cost: new_cost, pos: new_pos, dir });
                }
            }
        }

        // **Turn Left:**
        let left_dir = rotate_left(dir);
        let new_cost_left = cost + 1000;
        let key_left = (pos, left_dir);
        if new_cost_left < *costs.get(&key_left).unwrap_or(&u32::MAX) {
            heap.push(State { cost: new_cost_left, pos, dir: left_dir });
        }

        // **Turn Right:**
        let right_dir = rotate_right(dir);
        let new_cost_right = cost + 1000;
        let key_right = (pos, right_dir);
        if new_cost_right < *costs.get(&key_right).unwrap_or(&u32::MAX) {
            heap.push(State { cost: new_cost_right, pos, dir: right_dir });
        }
    }

    // If the end was not reached, return `None`.
    None
}

fn part1(start:(i32,i32), initial_dir : (i32,i32), board: HashMap<(i32, i32), char>) -> u32 {
    dijkstra(start, initial_dir, &board).unwrap_or_else(|| 0)
}

fn main() {
    // Open the input file.
    let file = File::open("inputs/16.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut board: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = None;

    // Parse the grid and identify the start position.
    for (y, line_res) in reader.lines().enumerate() {
        let line = line_res.expect("Could not read line");
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            board.insert(pos, c);
            if c == 'S' {
                start = Some(pos);
            }
        }
    }
        // Ensure that a start position was found.
    let start = start.expect("Start position 'S' not found in the grid.");

    // Define the initial direction: (1, 0) corresponds to moving right.
    let initial_dir = (1, 0); // Equivalent to `v = 1` in complex numbers.

    println!("Part 1: {:?}", part1(start, initial_dir, board));
}
