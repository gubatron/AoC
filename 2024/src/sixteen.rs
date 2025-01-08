use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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

/// Performs Dijkstra's algorithm to find the shortest paths from `start` to `'E'`.
///
/// This function is modified to track parents for Part Two.
/// It returns a tuple containing:
/// - A HashMap of the minimum cost to reach each `(pos, dir)` state.
/// - A HashMap of parents for each `(pos, dir)` state.
fn dijkstra(
    start: (i32, i32),
    initial_dir: (i32, i32),
    board: &HashMap<(i32, i32), char>,
) -> (HashMap<((i32, i32), (i32, i32)), u32>, HashMap<((i32, i32), (i32, i32)), Vec<((i32, i32), (i32, i32))>>) {
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, pos: start, dir: initial_dir });

    // Tracks the minimum cost to reach each `(pos, dir)` state.
    let mut costs: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();

    // Tracks the parents for each `(pos, dir)` state.
    let mut parents: HashMap<((i32, i32), (i32, i32)), Vec<((i32, i32), (i32, i32))>> = HashMap::new();

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
                // Continue processing to find all possible end states with minimal cost.
                // Do not return immediately.
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
                if new_cost <= *costs.get(&new_key).unwrap_or(&u32::MAX) {
                    heap.push(State { cost: new_cost, pos: new_pos, dir });
                    parents.entry(new_key).or_default().push(key);
                }
            }
        }

        // **Turn Left:**
        let left_dir = rotate_left(dir);
        let new_cost_left = cost + 1000;
        let key_left = (pos, left_dir);
        if new_cost_left <= *costs.get(&key_left).unwrap_or(&u32::MAX) {
            heap.push(State { cost: new_cost_left, pos, dir: left_dir });
            parents.entry(key_left).or_default().push(key);
        }

        // **Turn Right:**
        let right_dir = rotate_right(dir);
        let new_cost_right = cost + 1000;
        let key_right = (pos, right_dir);
        if new_cost_right <= *costs.get(&key_right).unwrap_or(&u32::MAX) {
            heap.push(State { cost: new_cost_right, pos, dir: right_dir });
            parents.entry(key_right).or_default().push(key);
        }
    }

    (costs, parents)
}

fn part1(
    start: (i32, i32),
    initial_dir: (i32, i32),
    board: &HashMap<(i32, i32), char>,
) -> Option<u32> {
    let (costs, _) = dijkstra(start, initial_dir, board);

    // Find all end states with minimal cost.
    // Since direction matters, iterate over possible directions at 'E'.
    // Assuming 'E' can be approached from any direction.
    // Collect all positions that are 'E'.
    let end_positions: Vec<&(i32, i32)> = board
        .iter()
        .filter(|&(_, &c)| c == 'E')
        .map(|(pos, _)| pos)
        .collect();

    // Find the minimal cost among all end states.
    let mut best_cost = None;
    for &end in &end_positions {
        for &dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            // Assuming four possible directions.
            if let Some(&cost) = costs.get(&((*end), dir)) {
                if best_cost.is_none() || cost < best_cost.unwrap() {
                    best_cost = Some(cost);
                }
            }
        }
    }

    best_cost
}

/// Part Two: Count all unique tiles that are part of any best path from 'S' to 'E'.
fn part2(
    start: (i32, i32),
    initial_dir: (i32, i32),
    board: &HashMap<(i32, i32), char>,
) -> usize {
    let (costs, parents) = dijkstra(start, initial_dir, board);

    // Find the minimal cost among all end states.
    let end_positions: Vec<&(i32, i32)> = board
        .iter()
        .filter(|&(_, &c)| c == 'E')
        .map(|(pos, _)| pos)
        .collect();

    let mut best_cost = std::u32::MAX;
    let mut end_states: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for &end_pos in &end_positions {
        for &dir in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            // Assuming four possible directions.
            if let Some(&cost) = costs.get(&((*end_pos), dir)) {
                if cost < best_cost {
                    best_cost = cost;
                    end_states.clear();
                    end_states.push(((*end_pos), dir));
                } else if cost == best_cost {
                    end_states.push(((*end_pos), dir));
                }
            }
        }
    }

    // Now, traverse the parents from each end_state with best_cost.
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();
    let mut pending: Vec<((i32, i32), (i32, i32))> = end_states.clone();

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    while let Some(state) = pending.pop() {
        if !visited.insert(state) {
            continue; // Already processed
        }
        let (pos, dir) = state;
        tiles.insert(pos);

        if let Some(parents_list) = parents.get(&(pos, dir)) {
            for parent in parents_list {
                pending.push(parent.clone());
            }
        }
    }

    tiles.len()
}

fn main() {
    // Open the input file.
    let file = File::open("inputs/16.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut board: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = None;
    let mut end = None;

    // Parse the grid and identify the start position.
    for (y, line_res) in reader.lines().enumerate() {
        let line = line_res.expect("Could not read line");
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            board.insert(pos, c);
            if c == 'S' {
                start = Some(pos);
            } else if c == 'E' {
                end = Some(pos);
            }
        }
    }
    // Ensure that a start position was found.
    let start = start.expect("Start position 'S' not found in the grid.");

    // Define the initial direction: (1, 0) corresponds to moving right.
    let initial_dir = (1, 0); // Equivalent to `v = 1` in complex numbers.

    match part1(start, initial_dir, &board) {
        Some(cost) => println!("Part 1: Minimal cost to reach 'E' is {}", cost), // 66404
        None => println!("Part 1: No path found from 'S' to 'E'."),
    }

    // Part 2: Count all unique tiles that are part of any best path.
    let tiles_count = part2(start, initial_dir, &board);
    println!(
        "Part 2: Number of tiles in any best path is {}", // 433
        tiles_count
    );
}
