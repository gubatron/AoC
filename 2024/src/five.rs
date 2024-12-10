use std::collections::{HashMap, HashSet, VecDeque};

// Graph is a map of pages, to a list of pages that should come after it
fn parse_rules(rules: &Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let parts: Vec<i32> = rule.split('|').map(|x| x.parse().unwrap()).collect();
        let (x, y) = (parts[0], parts[1]);
        graph.entry(x).or_default().push(y);
    }
    graph
}

fn is_valid_update(update: &[i32], graph: &HashMap<i32, Vec<i32>>) -> bool {
    // Map page numbers to their indices for quick lookup
    // [a,b,c,d] => {a: 0, b: 1, c: 2, d: 3}
    let positions: HashMap<i32, usize> = update
        .iter()
        .enumerate() // we get (i, item) on each iteration
        .map(|(i, &page)| (page, i))
        .collect();

    // Validate the order of pages
    // GRAPH:
    // P => [Other page, Other page 2, ... ]
    // P2 => [Other page, Other page 2, ... ]
    //
    for (&x, ys) in graph.iter() {
        if let Some(&pos_x) = positions.get(&x) {
            // Go through list of pages for this page's position
            for &y in ys {
                // if the position of this page is greater than the position of the other page
                // then the update is invalid
                if let Some(&pos_y) = positions.get(&y) {
                    if pos_x > pos_y {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn part1(rules: &Vec<String>, updates: &Vec<String>) -> i32 {
    let graph = parse_rules(rules);
    let mut valid_middle_sum = 0;

    for update in updates {
        let update_pages: Vec<i32> = update.split(',').map(|x| x.parse().unwrap()).collect();
        if is_valid_update(&update_pages, &graph) {
            let middle_page = update_pages[update_pages.len() / 2];
            valid_middle_sum += middle_page;
        }
    }

    valid_middle_sum
}

fn part2(rules: &Vec<String>, updates: &Vec<String>) -> i32 {
    let graph = parse_rules(&rules);
    let mut reordered_middle_sum = 0;

    for update in updates {
        let update_pages: Vec<i32> = update.split(',').map(|x| x.parse().unwrap()).collect();
        if !is_valid_update(&update_pages, &graph) {
            let subgraph = build_subgraph(&update_pages, &graph);
            let reordered = topological_sort(&subgraph);
            let middle_page = reordered[reordered.len() / 2];
            reordered_middle_sum += middle_page;
        }
    }

    reordered_middle_sum
}

// Topological sort to reorder an update
fn topological_sort(subgraph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut result = Vec::new();

    // Calculate in-degree of each node
    for (&node, edges) in subgraph.iter() {
        in_degree.entry(node).or_insert(0);
        for &neighbor in edges {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }

    // Find all nodes with in-degree 0
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = subgraph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    result
}

// Create a subgraph for the update
fn build_subgraph(update: &[i32], graph: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let update_set: HashSet<i32> = update.iter().cloned().collect();
    let mut subgraph = HashMap::new();

    for (&node, edges) in graph.iter() {
        if update_set.contains(&node) {
            subgraph.insert(
                node,
                edges
                    .iter()
                    .filter(|&&neighbor| update_set.contains(&neighbor))
                    .cloned()
                    .collect(),
            );
        }
    }

    subgraph
}

// https://adventofcode.com/2024/day/5
fn main() {
    let input = aoc::utils::load_input_lines_as_vec_str("inputs/5.txt");

    // the rules will be the first lines until we get an empty line
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut is_rules = true;
    for line in input {
        if line.is_empty() {
            is_rules = false;
            continue;
        }
        if is_rules {
            rules.push(line);
        } else {
            updates.push(line);
        }
    }

    // Part 1: 4790
    println!("Part 1: {}", part1(&rules, &updates));
    // Part 2: 6319
    println!("Part 2: {}", part2(&rules, &updates));
}
