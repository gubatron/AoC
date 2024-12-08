use std::collections::{HashMap};

// Graph is a map of pages, to a list of pages that should come after it
fn parse_rules(rules: Vec<String>) -> HashMap<i32, Vec<i32>> {
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

fn part1(rules: Vec<String>, updates: Vec<String>) -> i32 {
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

// https://adventofcode.com/2024/day/5
fn main() {
    let input = aoc::utils::load_input_lines_as_vec_str("5.txt");

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

    let result = part1(rules, updates);
    // Part 1: 4790
    println!("Part 1: {}", result);
}
