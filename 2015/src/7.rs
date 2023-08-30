use aoc_2015::utils;
use std::collections::HashMap;

fn get_value(circuit_map: &HashMap<String, u16>, s: &str) -> Option<u16> {
    s.parse().ok().or_else(|| circuit_map.get(s).cloned())
}

fn evaluate_instructions(mut instructions: Vec<String>, mut circuit_map: &mut HashMap<String, u16>) {
    let mut iterations_left = 15000;
    while !instructions.is_empty() && iterations_left > 0 {
        iterations_left -= 1;
        let mut i: usize = 0;
        while i < instructions.len() {
            let inst = &instructions[i];
            if inst.contains("AND") || inst.contains("OR") {
                let tokens: Vec<&str> = inst.split_whitespace().collect();
                let (a, b, gate_id) = (tokens[0], tokens[2], tokens[4]);

                let a_in_circuit_map = circuit_map.contains_key(a);
                let b_in_circuit_map = circuit_map.get(b) != None;
                let a_is_numeric = a.parse::<u16>().is_ok();
                let b_is_numeric = b.parse::<u16>().is_ok();

                if a_is_numeric || a_in_circuit_map {
                    if b_is_numeric || b_in_circuit_map {
                        if inst.contains("AND") {
                            circuit_map.insert(
                                gate_id.to_string(),
                                get_value(&circuit_map, a).unwrap()
                                    & get_value(&circuit_map, b).unwrap(),
                            );
                        } else if inst.contains("OR") {
                            circuit_map.insert(
                                gate_id.to_string(),
                                get_value(&circuit_map, a).unwrap()
                                    | get_value(&circuit_map, b).unwrap(),
                            );
                        }
                        instructions.remove(i);
                        if i > 0 {
                            i -= 1;
                        }
                        continue;
                    }
                }
            } else if inst.contains("LSHIFT") || inst.contains("RSHIFT") {
                let tokens: Vec<&str> = inst.split_whitespace().collect();
                let (a, n, gate_id) = (tokens[0], tokens[2].parse::<u16>().unwrap(), tokens[4]);
                if let Some(val) = get_value(&circuit_map, a) {
                    if inst.contains("LSHIFT") {
                        circuit_map.insert(gate_id.to_string(), val << n);
                    } else if inst.contains("RSHIFT") {
                        circuit_map.insert(gate_id.to_string(), val >> n);
                    }

                    instructions.remove(i);
                    if i > 0 {
                        i -= 1;
                    }
                    continue;
                }
            } else if inst.contains("NOT") {
                let tokens: Vec<&str> = inst.split_whitespace().collect();
                let (a, gate_id) = (tokens[1], tokens[3]);
                if let Some(val) = get_value(&circuit_map, a) {
                    circuit_map.insert(gate_id.to_string(), !val);
                    instructions.remove(i);
                    if i > 0 {
                        i -= 1;
                    }
                    continue;
                }
            } else {
                let tokens: Vec<&str> = inst.split(" -> ").collect();
                let (a, gate_id) = (tokens[0], tokens[1]);
                if let Some(val) = get_value(&circuit_map, a) {
                    circuit_map.insert(gate_id.to_string(), val);
                    instructions.remove(i);
                    if i > 0 {
                        i -= 1;
                    }
                    continue;
                }
            }
            i += 1;
        }
    }
}

fn part1() -> u16 {
    let mut instructions = utils::load_input_lines_as_vec_str("src/7.txt");
    let mut circuit_map: HashMap<String, u16> = HashMap::new();
    evaluate_instructions(instructions, &mut circuit_map);
    circuit_map.get("a").unwrap().clone()
} // part1

fn part2() -> u16 {
    let mut instructions = utils::load_input_lines_as_vec_str("src/7.2.txt");
    let mut circuit_map: HashMap<String, u16> = HashMap::new();
    evaluate_instructions(instructions, &mut circuit_map);
    circuit_map.get("a").unwrap().clone()
}

fn main() {
    println!("Day 7, Part 1: {}", part1());
    println!("Day 7, Part 2: {}", part2());
    // 33706: Too low.
    // 14146: Too low.
}
