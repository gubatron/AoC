use aoc::utils::{load_input_break_by_empty_lines_as_vec_str, CoordU64};
use regex::Regex;

#[derive(Debug)]
struct ButtonConfig {
    movement: CoordU64,
    cost: i32,
}

#[derive(Debug)]
struct PrizeConfig {
    button_a: ButtonConfig,
    button_b: ButtonConfig,
    prize: CoordU64,
}

fn parse_button(input: &str, input_cost: i32) -> Option<ButtonConfig> {
    let button_regex = "X\\+(\\d+), Y\\+(\\d+)";
    let re = Regex::new(button_regex).unwrap();
    if let Some(captures) = re.captures(input) {
        let x_group: u64 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group: u64 = captures.get(2).unwrap().as_str().parse().expect("NaN");

        Some(ButtonConfig {
            movement: CoordU64 {
                x: x_group,
                y: y_group,
            },
            cost: input_cost,
        })
    } else {
        None
    }
}

fn parse_prize_coord(input: &str) -> Option<CoordU64> {
    //Prize: X=8400, Y=5400
    let prize_regex = "X=(\\d+), Y=(\\d+)";
    let re = Regex::new(prize_regex).unwrap();
    if let Some(captures) = re.captures(input) {
        let x_group: u64 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group: u64 = captures.get(2).unwrap().as_str().parse().expect("NaN");

        Some(CoordU64 {
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
    let parsed_prize: CoordU64 = parse_prize_coord(prize_str).unwrap();
    PrizeConfig {
        button_a: parsed_button_a,
        button_b: parsed_button_b,
        prize: parsed_prize,
    }
}
// fn build_possible_graph(config: &PrizeConfig) -> HashMap<CoordU64, Vec<(CoordU64, i32)>> {
//     let mut graph: HashMap<CoordU64, Vec<(CoordU64, i32)>> = HashMap::new();
//     let mut visited: HashSet<CoordU64> = HashSet::new();
//     let mut loc = CoordU64 { x: 0, y: 0 };
//     let mut queue: Vec<CoordU64> = vec![];
//     loop {
//         while visited.contains(&loc) && !queue.is_empty() {
//             loc = queue.pop().unwrap();
//         }
//         let a_loc = CoordU64 {
//             x: loc.x + config.button_a.movement.x,
//             y: loc.y + config.button_a.movement.y,
//         };
//         let b_loc = CoordU64 {
//             x: loc.x + config.button_b.movement.x,
//             y: loc.y + config.button_b.movement.y,
//         };
//         if a_loc.x == config.prize.x && a_loc.y == config.prize.y {
//             graph.insert(loc, vec![(a_loc, config.button_a.cost)]);
//             return graph;
//         }
//         if b_loc.x == config.prize.x && b_loc.y == config.prize.y {
//             graph.insert(loc, vec![(b_loc, config.button_b.cost)]);
//             return graph;
//         }
//         if a_loc.x < config.prize.x && a_loc.y < config.prize.y && !visited.contains(&a_loc) {
//             queue.push(a_loc);
//             graph.insert(loc, vec![(a_loc, config.button_a.cost)]);
//         }
//         if b_loc.x < config.prize.x && b_loc.y < config.prize.y && !visited.contains(&b_loc) {
//             queue.push(b_loc);
//             if let Some(loc_vec) = graph.get(&loc) {
//                 let mut new_vec = loc_vec.clone();
//                 new_vec.push((b_loc, config.button_b.cost));
//                 graph.insert(loc, new_vec);
//             } else {
//                 graph.insert(loc, vec![(b_loc, config.button_b.cost)]);
//             }
//         }
//         visited.insert(loc);
//         // NO PATH IS POSSIBLE
//         if queue.is_empty() {
//             return HashMap::new();
//         }
//         loc = queue.pop().unwrap();
//     }
// }

// fn least_cost_path(
//     start: CoordU64,
//     graph: &HashMap<CoordU64, Vec<(CoordU64, i32)>>,
// ) -> HashMap<CoordU64, i32> {
//     let mut distances = HashMap::<CoordU64, i32>::new();
//     let mut queue = BinaryHeap::<Reverse<(CoordU64, i32)>>::new();
//     queue.push(Reverse((start, 0)));
//     distances.insert(start, 0);
//     while !queue.is_empty() {
//         let Reverse((node, dist)) = queue.pop().unwrap();
//         if let Some(neighbors) = graph.get(&node) {
//             for neigh_tuple in neighbors {
//                 let new_dist = neigh_tuple.1 + dist;
//                 if !distances.contains_key(&neigh_tuple.0) || new_dist < distances[&neigh_tuple.0] {
//                     distances.insert(neigh_tuple.0, new_dist);
//                     queue.push(Reverse((neigh_tuple.0, new_dist)));
//                 }
//             }
//         }
//     }
//     distances
// }

// Create a matrix and vector from the given PrizeConfig
//
// If there's a solution it has to be a combination of the two linear equations
// n and m are the number of times the buttons are pressed for A and B
//
// Prize X = Xa * n + Xb * m
// Prize Y = Ya * n + Yb * m
//
// [ Xa, Xb ] [ n ] = [ Prize X ]
// [ Ya, Yb ] [ m ] = [ Prize Y ]
//
// We will use Linear algebra to find the values of n and m if they exist
//
// So we return the coefficient matrix and target vector
//
fn create_matrix_and_vector(config: &PrizeConfig) -> ([[u64; 2]; 2], [u64; 2]) {
    // Extract coefficients from button movements
    let matrix = [
        [config.button_a.movement.x, config.button_b.movement.x], // X coefficients
        [config.button_a.movement.y, config.button_b.movement.y], // Y coefficients
    ];

    // Extract prize coordinates
    let vector = [config.prize.x, config.prize.y];

    (matrix, vector)
}

fn has_solutions(matrix: [[u64; 2]; 2], vector: [u64; 2]) -> bool {
    // Convert to signed integers for calculations, as we need to check for negative values
    let a = matrix[0][0] as i64;
    let b = matrix[0][1] as i64;
    let c = matrix[1][0] as i64;
    let d = matrix[1][1] as i64;

    // Calculate determinant as signed
    let determinant = a * d - b * c;

    if determinant != 0 {
        // Determinant is non-zero: the system has a unique solution
        return true;
    }

    // Determinant is zero: check if the prize vector is in the span of the buttons
    // Calculate ratios for both rows
    let (a1, b1, c1) = (matrix[0][0], matrix[0][1], vector[0]);
    let (a2, b2, c2) = (matrix[1][0], matrix[1][1], vector[1]);

    let ratio1 = c1 * b2 == c2 * b1; // Cross-multiplication avoids floating-point errors
    let ratio2 = c1 * a2 == c2 * a1;

    ratio1 && ratio2
}

//
// Coefficient Matrix = [ Xa, Xb ]
//                      [ Ya, Yb ]
//
// [ Xa, Xb ] [ n ] = [ Prize X ]
// [ Ya, Yb ] [ m ] = [ Prize Y ]
//
// n = detN / det
// m = detM / det
//
// det = Xa * Yb - Xb * Ya
//
// detN is the determinant of the matrix formed by replacing the FIRST column of the coefficient matrix with the target vector
// detM is the determinant of the matrix formed by replacing the SECOND column of the coefficient matrix with the target vector
////
// detN = [ Prize X, Xb] = Prize X * Yb - Xb * Prize Y
//        [ Prize Y, Yb]
//
// detM = [ Xa, Prize X] = Xa * Prize Y - Prize X * Ya
//        [ Ya, Prize Y]
//
fn solve_for_n_and_m(matrix: [[u64; 2]; 2], vector: [u64; 2]) -> Option<(u64, u64)> {
    let determinant =
        matrix[0][0] as i64 * matrix[1][1] as i64 - matrix[0][1] as i64 * matrix[1][0] as i64;

    // this shouldn't happen because we should make this check before calling this function
    // so we panic
    if determinant == 0 {
        panic!("No solutions found");
    }

    let xa = matrix[0][0] as i64;
    let xb = matrix[0][1] as i64;
    let ya = matrix[1][0] as i64;
    let yb = matrix[1][1] as i64;
    let prize_x = vector[0] as i64;
    let prize_y = vector[1] as i64;

    let det_n = prize_x * yb - xb * prize_y;
    let det_m = xa * prize_y - prize_x * ya;

    // Check if both n and m are integers
    if det_n % determinant != 0 || det_m % determinant != 0 {
        return None; // No integer solutions
    }

    let n = det_n / determinant;
    let m = det_m / determinant;

    if n < 0 || m < 0 {
        return None;
    }

    let reconstructed_x = xa * n + xb * m;
    let reconstructed_y = ya * n + yb * m;

    if reconstructed_x != prize_x || reconstructed_y != prize_y {
        return None;
    }

    Some((n as u64, m as u64))
}

fn part1(prize_configs: &Vec<PrizeConfig>) -> i32 {
    // timestamp now
    let mut _i = 1;
    let mut total_cost = 0;
    for config in prize_configs {
        _i += 1;
        let (coefficient_matrix, target_vector) = create_matrix_and_vector(&config);
        if !has_solutions(coefficient_matrix, target_vector) {
            //println!("No path possible");
            continue;
        }
        if let Some((n, m)) = solve_for_n_and_m(coefficient_matrix, target_vector) {
            let cost = n as i32 * config.button_a.cost + m as i32 * config.button_b.cost;
            //println!("n: {}, m: {}, cost: {}", n, m, cost);
            total_cost += cost;
        }
    }
    total_cost
}

fn part2(prize_configs: Vec<PrizeConfig>) -> i64 {
    // timestamp now
    let mut _i = 1;
    let mut total_cost: i64 = 0;
    for config in prize_configs {
        _i += 1;
        let mut config = config;
        config.prize.x += 10000000000000;
        config.prize.y += 10000000000000;
        let (coefficient_matrix, target_vector) = create_matrix_and_vector(&config);
        if !has_solutions(coefficient_matrix, target_vector) {
            continue;
        }
        if let Some((n, m)) = solve_for_n_and_m(coefficient_matrix, target_vector) {
            let cost =
                n as i64 * config.button_a.cost as i64 + m as i64 * config.button_b.cost as i64;
            total_cost += cost;
        }
    }
    total_cost
}

fn main() {
    let inputs: Vec<String> = load_input_break_by_empty_lines_as_vec_str("inputs/13.txt");
    let mut prize_configs: Vec<PrizeConfig> = vec![];
    for input in inputs {
        prize_configs.push(parse_config_prize(input));
    }
    println!("Part 1: {:?}", part1(&prize_configs)); // Part 1: 29023
    println!("Part 2: {:?}", part2(prize_configs)); // Part 2: 96787395375634
}
