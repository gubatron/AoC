use aoc::utils::Coord;
use std::collections::{HashSet, VecDeque};

pub fn bfs_find_trailhead_score(
    start: Coord,
    grid: &Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
) -> HashSet<Coord> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut reachable_9s = HashSet::new();

    queue.push_back(start);
    seen.insert(start);

    while let Some(node) = queue.pop_front() {
        let current_height = grid[node.y as usize][node.x as usize];

        if current_height == 9 {
            reachable_9s.insert(node);
            continue; // Stop exploring past height 9
        }

        for neighbor in node.neighbors(rows as i32, cols as i32, false) {
            if !seen.contains(&neighbor) {
                let neighbor_height = grid[neighbor.y as usize][neighbor.x as usize];
                if neighbor_height == current_height + 1 {
                    seen.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    reachable_9s
}

pub fn bfs_find_distinct_paths(
    start: Coord,
    grid: &Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
) -> HashSet<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut distinct_paths = HashSet::new();

    // Each queue item contains the current path and current position
    queue.push_back((vec![start], start));

    while let Some((current_path, current_node)) = queue.pop_front() {
        let current_height = grid[current_node.y as usize][current_node.x as usize];

        if current_height == 9 {
            // If we've reached a 9, record the path as a distinct hiking trail
            distinct_paths.insert(current_path.clone());
            continue;
        }

        for neighbor in current_node.neighbors(rows as i32, cols as i32, false) {
            let neighbor_height = grid[neighbor.y as usize][neighbor.x as usize];

            if neighbor_height == current_height + 1 && !current_path.contains(&neighbor) {
                // Append the neighbor to the current path and add to the queue
                let mut new_path = current_path.clone();
                new_path.push(neighbor);
                queue.push_back((new_path, neighbor));
            }
        }
    }

    distinct_paths
}

fn part1(heights: &Vec<Vec<i32>>, cols: usize, rows: usize) -> usize {
    let mut total_score = 0;
    for y in 0..rows {
        for x in 0..cols {
            // found an origin
            if heights[y][x] == 0 {
                // returns a Hashset of all the 9s reachable Coords from this origin
                total_score +=
                    bfs_find_trailhead_score(Coord::new(x as i32, y as i32), &heights, rows, cols)
                        .len();
            }
        }
    }
    total_score
}

fn part2(heights: &Vec<Vec<i32>>, cols: usize, rows: usize) -> usize {
    let mut total_score = 0;
    for y in 0..rows {
        for x in 0..cols {
            // found an origin
            if heights[y][x] == 0 {
                // returns a Hashset of all the valid paths that reach a 9
                total_score +=
                    bfs_find_distinct_paths(Coord::new(x as i32, y as i32), &heights, rows, cols)
                        .len();
            }
        }
    }
    total_score
}

fn main() {
    let heights = aoc::utils::load_input_as_single_digit_matrix("inputs/10.txt");
    let (cols, rows) = aoc::utils::dimensions_cols_rows(&heights);
    println!("Part 1: {}", part1(&heights, cols, rows)); // Part 1: 733
    println!("Part 2: {}", part2(&heights, cols, rows)); // Part 2:
}
