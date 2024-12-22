use aoc::utils::{load_input_break_by_empty_lines_as_vec_str, Coord};
use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct ButtonConfig {
    movement: Coord,
    cost: i32,
}

#[derive(Debug)]
struct PrizeConfig {
    button_a: ButtonConfig,
    button_b: ButtonConfig,
    prize: Coord,
}

fn parse_button(input: &str, input_cost: i32) -> Option<ButtonConfig> {
    let button_regex = "X\\+(\\d+), Y\\+(\\d+)";
    let re = Regex::new(button_regex).unwrap();
    if let Some(captures) = re.captures(input) {
        let x_group: i32 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group: i32 = captures.get(2).unwrap().as_str().parse().expect("NaN");

        Some(ButtonConfig {
            movement: Coord {
                x: x_group,
                y: y_group,
            },
            cost: input_cost,
        })
    } else {
        None
    }
}

fn parse_prize_coord(input: &str) -> Option<Coord> {
    //Prize: X=8400, Y=5400
    let prize_regex = "X=(\\d+), Y=(\\d+)";
    let re = Regex::new(prize_regex).unwrap();
    if let Some(captures) = re.captures(input) {
        let x_group: i32 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group: i32 = captures.get(2).unwrap().as_str().parse().expect("NaN");

        Some(Coord {
            x: x_group,
            y: y_group,
        })
    } else {
        None
    }
}

fn parse_config_prize(input: String) -> PrizeConfig {
    let input_vec: Vec<&str> = input.split("\n").collect();
    let button_a_str = input_vec[0];
    let button_b_str = input_vec[1];
    let prize_str = input_vec[2];
    let parsed_button_a: ButtonConfig = parse_button(button_a_str, 3).unwrap();
    let parsed_button_b: ButtonConfig = parse_button(button_b_str, 1).unwrap();
    let parsed_prize: Coord = parse_prize_coord(prize_str).unwrap();
    PrizeConfig {
        button_a: parsed_button_a,
        button_b: parsed_button_b,
        prize: parsed_prize,
    }
}
fn build_possible_graph(config: &PrizeConfig) -> HashMap<Coord, Vec<(Coord, i32)>> {
    let mut graph: HashMap<Coord, Vec<(Coord, i32)>> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut loc = Coord { x: 0, y: 0 };
    let mut queue: Vec<Coord> = vec![];
    loop {
        while visited.contains(&loc) && !queue.is_empty() {
            loc = queue.pop().unwrap();
        }
        let a_loc = Coord {
            x: loc.x + config.button_a.movement.x,
            y: loc.y + config.button_a.movement.y,
        };
        let b_loc = Coord {
            x: loc.x + config.button_b.movement.x,
            y: loc.y + config.button_b.movement.y,
        };
        if a_loc.x == config.prize.x && a_loc.y == config.prize.y {
            graph.insert(loc, vec![(a_loc, config.button_a.cost)]);
            return graph;
        }
        if b_loc.x == config.prize.x && b_loc.y == config.prize.y {
            graph.insert(loc, vec![(b_loc, config.button_b.cost)]);
            return graph;
        }
        if a_loc.x < config.prize.x && a_loc.y < config.prize.y {
            queue.push(a_loc);
            graph.insert(loc, vec![(a_loc, config.button_a.cost)]);
        }
        if b_loc.x < config.prize.x && b_loc.y < config.prize.y {
            queue.push(b_loc);
            if let Some(loc_vec) = graph.get(&loc) {
                let mut new_vec = loc_vec.clone();
                new_vec.push((b_loc, config.button_b.cost));
                graph.insert(loc, new_vec);
            } else {
                graph.insert(loc, vec![(b_loc, config.button_b.cost)]);
            }
        }
        visited.insert(loc);
        // NO PATH IS POSSIBLE
        if queue.is_empty() {
            return HashMap::new()
        }
        loc = queue.pop().unwrap();
    }
}

fn least_cost_path(start: Coord, graph: &HashMap<Coord, Vec<(Coord, i32)>>) -> HashMap<Coord, i32> {
    let mut distances = HashMap::<Coord, i32>::new();
    let mut queue = BinaryHeap::<(i32, Coord)>::new();
    queue.push((0, start));
    distances.insert(start, 0);
    while !queue.is_empty() {
        let (dist, node) = queue.pop().unwrap();
        if let Some(neighbors) = graph.get(&node) {
            for neigh_tuple in neighbors {
                let new_dist = neigh_tuple.1 + dist;
                if !distances.contains_key(&neigh_tuple.0) || new_dist < distances[&neigh_tuple.0] {
                    distances.insert(neigh_tuple.0, new_dist);
                    queue.push((new_dist, neigh_tuple.0));
                }
            }
        }
    }
    distances
}

fn part1(prize_configs: Vec<PrizeConfig>) -> i32 {
    // timestamp now
    let mut i = 1;
    let mut total_cost = 0;
    for config in prize_configs {
        println!("{}", i);
        i += 1;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let g = build_possible_graph(&config);
        if g.is_empty() {
            println!("No path possible");
            continue;
        }
        let distances: HashMap<Coord, i32> = least_cost_path(Coord { x: 0, y: 0 }, &g);
        total_cost += distances[&config.prize];
        let then = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("Time taken: {:?}ms", then - now);
        println!("=====================");
    }
    total_cost
}

fn main() {
    let inputs: Vec<String> = load_input_break_by_empty_lines_as_vec_str("inputs/13.txt");

    let mut prize_configs: Vec<PrizeConfig> = vec![];
    for input in inputs {
        prize_configs.push(parse_config_prize(input));
    }

    println!("Part 1: {:?}", part1(prize_configs)); // Part 1: 29023
}
