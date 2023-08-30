use aoc_2015::utils;
use std::collections::HashMap;

fn get_value(circuit_map: &HashMap<String, u16>, s: &str) -> Option<u16> {
    s.parse().ok().or_else(|| circuit_map.get(s).cloned())
}

fn evaluate_instructions(mut instructions : Vec<String>) -> HashMap<String, u16> {
  let mut circuit_map: HashMap<String, u16> = HashMap::new();
  let mut i = 0;
  while !instructions.is_empty() {
    while i < instructions.len() {
        let inst = &instructions[i];
        if inst.contains("AND") || inst.contains("OR") {
            let tokens : Vec<&str> = inst.split_whitespace().collect();
            let (a, b, gate_id) = (tokens[0], tokens[2], tokens[4]);

            let a_in_circuit_map = circuit_map.contains_key(a);
            let b_in_circuit_map = circuit_map.get(b) != None;
            let a_is_numeric = a.parse::<u16>().is_ok();
            let b_is_numeric = b.parse::<u16>().is_ok();

            if a_is_numeric || a_in_circuit_map {
                if b_is_numeric || b_in_circuit_map {
                    if inst.contains("AND") {
                        circuit_map.insert(gate_id.to_string(), get_value(&circuit_map, a).unwrap() & get_value(&circuit_map, b).unwrap());
                    } else if inst.contains("OR") {
                        circuit_map.insert(gate_id.to_string(), get_value(&circuit_map, a).unwrap() | get_value(&circuit_map, b).unwrap());
                    }
                    instructions.remove(i);
                    continue;
                }
            }        
        } else if inst.contains("LSHIFT") || inst.contains("RSHIFT") {
            let tokens : Vec<&str> = inst.split_whitespace().collect();
            let (a, n, gate_id) = (tokens[0], tokens[2].parse::<u16>().unwrap(), tokens[4]);
            if let Some(val) = get_value(&circuit_map, a) {
                if inst.contains("LSHIFT") {
                    circuit_map.insert(gate_id.to_string(), val << n);
                } else if inst.contains("RSHIFT") {
                    circuit_map.insert(gate_id.to_string(), val >> n);
                }

                instructions.remove(i);
                continue;
            }
        } else if inst.contains("NOT") {
            let tokens : Vec<&str> = inst.split_whitespace().collect();
            let (a, gate_id) = (tokens[1], tokens[3]);
            if let Some(val) = get_value(&circuit_map, a) {
                circuit_map.insert(gate_id.to_string(), !val);
                instructions.remove(i);
                continue;
            }
        } else {
            let tokens : Vec<&str> = inst.split(" -> ").collect();
            let (a, gate_id) = (tokens[0], tokens[1]);
            let a_is_numeric = a.parse::<u16>().is_ok();
            let a_in_circuit_map = circuit_map.contains_key(a);
            if a_is_numeric || a_in_circuit_map {
                circuit_map.insert(gate_id.to_string(), get_value(&circuit_map, a).unwrap());
                instructions.remove(i);
                continue;
            }
        }
        i += 1; 
    }
  }
  circuit_map
}

fn part1() -> i32 {
    let instructions = utils::load_input_lines_as_vec_str("src/7.txt");
    let mut circuit_map = evaluate_instructions(instructions);

    // get all gate names in alphabetical order into a vector
    let mut gate_names: Vec<String> = circuit_map.iter().map(|(id, _)| id.clone()).collect();
    // sort gate_names alphabetically, ascending
    gate_names.sort();

    // print all gate names and their signals
    for name in gate_names {
        if circuit_map.contains_key(&name) {
            println!("{}: {:?}", name, get_value(&circuit_map, &name).unwrap());
        } else {
            println!("got a gate for '{}'?", name);
        }
    }
    0
} // part1

fn main() {
    println!("Day 7, Part 1: {}", part1());
    println!("Day 7, Part 2: {}", "TODO");
}
