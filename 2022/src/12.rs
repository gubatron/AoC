use std::collections::HashMap;
use log::info;

use aoc_2022::utils::{bfs, dijkstra, Coord};

fn main() {
    // Hill Climbing Algorithm (BFS)
    env_logger::try_init().unwrap();
    let input = aoc_2022::utils::load_input_lines_as_vec_str("inputs/12.txt");
    let (start, end) = init_start_end(&input);
    info!("S: {:?}, E: {:?}", start, end);
    let height_matrix: Vec<Vec<i32>> = build_height_matrix(&input);
    //print_heights(&height_matrix);

    let graph_uphill = create_graph(&height_matrix, true, false);

    let now = std::time::Instant::now();
    let (steps_bfs, _, ) = bfs::<Coord>(start, end, &graph_uphill);
    // test: 31
    // puzzle input: 528
    println!("Part 1 (BFS): {} in {:?}", steps_bfs, now.elapsed());

    let lowest_points = find_lowest_points(&height_matrix);
    let graph_downhill = create_graph(&height_matrix, false, false);
    let now = std::time::Instant::now();
    let shortest_path_distances_bfs = lowest_points
        .iter()
        .map(|p| {
            // we start from the end
            let (steps, _) = bfs::<Coord>(end, *p, &graph_downhill);
            steps
        })
        .min().unwrap();

    // BFS was 26 seconds
    // test: 29
    // puzzle input: 522
    println!("Part 2 (BFS): {} in {:?}", shortest_path_distances_bfs, now.elapsed());
}

fn find_lowest_points(height_matrix: &Vec<Vec<i32>>) -> Vec<Coord> {
    let mut lowest = vec![];
    for y in 0..height_matrix.len() {
        for x in 0..height_matrix[y as usize].len() {
            if height_matrix[y as usize][x as usize] == 1 {
                lowest.push(Coord::new(x as i32, y as i32));
            }
        }
    }
    lowest
}

// convert input into a Vec<Vec<char>> and extract the coordinates of 'S' and 'E' into 2 variables made of (u32,u32)
fn init_start_end(input: &Vec<String>) -> (Coord, Coord) {
    let map: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                start.x = x as i32;
                start.y = y as i32;
            } else if *c == 'E' {
                end.x = x as i32;
                end.y = y as i32;
            }
        }
    }
    (start, end)
}

// print the Vec<Vec<u32>> as an NxN matrix
fn print_heights(height_matrix: &Vec<Vec<i32>>) {
    for row in height_matrix {
        for col in row {
            info!("{:02} ", col);
        }
        info!("");
    }
}

// convert input into a Vec<Vec<u32> where a=1, b=2, c=3, .. z=26
// special cases are S=1 and E=26
fn build_height_matrix(input: &Vec<String>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'a'..='z' => c as i32 - 96,
                    'S' => 1,
                    'E' => 26,
                    _ => panic!("unexpected char"),
                })
                .collect()
        })
        .collect()
}

// Creates a graph of all the valid paths between the start and end points
// It's represented as a Map<Coord, Vec<Coord>> where the key is the current node and the value is a list of all the valid nodes that can be reached from the current node
fn create_graph(height_matrix: &Vec<Vec<i32>>, up: bool, consider_diagonals: bool) -> HashMap<Coord, Vec<Coord>> {
    let mut graph = HashMap::<Coord, Vec<Coord>>::new();

    let rows = height_matrix.len() as i32;
    let cols = height_matrix[0].len() as i32;
    info!("create_graph rows:{} cols:{}", rows, cols);

    for y in 0..rows {
        for x in 0..cols {
            let coord = Coord::new(x, y);
            let mut climbable_neighbors = vec![];

            let possible_neighbors =
                aoc_2022::utils::neighbors(&coord, rows, cols, consider_diagonals);

            for candidate in possible_neighbors {
                let h1 = height_matrix[y as usize][x as usize];
                let h2 = height_matrix[candidate.y as usize][candidate.x as usize];

                if (up && h2 > h1 + 1) || (!up && h1 > h2 + 1) {
                    continue;
                }

                climbable_neighbors.push(candidate);
            }
            if !climbable_neighbors.is_empty() {
                graph.insert(coord, climbable_neighbors);
            }
        }
    }
    graph
}