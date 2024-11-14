use std::collections::HashMap;
use itertools::Itertools;
use aoc_2015::utils;
use utils::load_input_lines_as_vec_str;

// Define the Node struct to hold the name and distances to other nodes.
#[derive(Debug, Clone)]
struct Node {
    name: String,
    distances: HashMap<String, u32>,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            name: name.to_string(),
            distances: HashMap::new(),
        }
    }

    fn add_distance(&mut self, destination: String, distance: u32) {
        self.distances.insert(destination, distance);
    }

    fn get_distance(&self, destination: &String) -> Option<u32> {
        self.distances.get(destination).cloned()
    }
}

// Helper function to calculate the total distance of a route
fn calculate_route_distance(graph: &HashMap<String, Node>, route: &[&String]) -> u32 {
    let mut total_distance = 0;
    for window in route.windows(2) {
        if let Some(dist) = graph[window[0]].get_distance(window[1]) {
            total_distance += dist;
        }
    }
    total_distance
}

// The part1 function calculates the shortest route distance
fn part1(graph: &HashMap<String, Node>) -> u32 {
    let city_names: Vec<String> = graph.keys().cloned().collect();
    let mut shortest_distance = u32::MAX;

    for perm in city_names.iter().permutations(city_names.len()) {
        let distance = calculate_route_distance(graph, &perm);
        if distance < shortest_distance {
            shortest_distance = distance;
        }
    }

    shortest_distance
}

// The part2 function calculates the longest route distance
fn part2(graph: &HashMap<String, Node>) -> u32 {
    let city_names: Vec<String> = graph.keys().cloned().collect();
    let mut longest_distance = 0;

    for perm in city_names.iter().permutations(city_names.len()) {
        let distance = calculate_route_distance(graph, &perm);
        if distance > longest_distance {
            longest_distance = distance;
        }
    }

    longest_distance
}

fn main() {
    // Initialize the graph as a HashMap of nodes
    let mut graph: HashMap<String, Node> = HashMap::new();

    // Load cities and distances from the file
    let mut cities: Vec<(String, String, u32)> = Vec::new();
    load_input_lines_as_vec_str("src/9.txt").iter().for_each(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let destinations: Vec<&str> = parts[0].split(" to ").collect();
        let dist = parts[1].parse::<u32>().unwrap();
        cities.push((destinations[0].to_string(), destinations[1].to_string(), dist));
    });

    // Populate the graph with nodes and distances
    for (from, to, distance) in cities {
        graph.entry(from.clone()).or_insert_with(|| Node::new(from.clone()))
            .add_distance(to.clone(), distance);
        graph.entry(to.clone()).or_insert_with(|| Node::new(to.clone()))
            .add_distance(from, distance);
    }

    println!("Part 1: Shortest route distance: {}", part1(&graph));
    println!("Part 2: Longest route distance: {}", part2(&graph));
}
