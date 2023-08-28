use aoc_2015::utils;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Instruction {
  AND,
  OR,
  LSHIFT,
  RSHIFT,
  NOT,
  NONE
}

#[derive(Clone)]
struct gate {
  input1: String,
  input2: Option<String>,
  instruction: Option<Instruction>,
  signal: Option<u16>
}

fn gate_has_signal(gate_id : &String, circuit_map : &HashMap<String, gate>) -> bool {
  if let Some(g) = circuit_map.get(gate_id) {
    if let Some(_) = g.signal {
      return true;
    }
  }
  false
}

fn gate_has_signal_on_both_inputs(gate_id: &String, circuit_map: &HashMap<String, gate>) -> bool {
  if let Some(gate) = circuit_map.get(gate_id) {
    // is the first input convertable to a u16
    if let Ok(_) = gate.input1.parse::<u16>() {
      // is the second input convertable to a u16
      if let Ok(_) = gate.input2.as_ref().unwrap().parse::<u16>() {
        return true;
      } else {
        return gate_has_signal(&gate.input2.as_ref().unwrap(), circuit_map);
      }
    } else {
      // first input is not a u16, so we need to check if it has a signal
        if gate_has_signal(&gate.input1, circuit_map) {
            // first input has a signal, so we need to check if the second input is a u16
            if let Ok(_) = gate.input2.as_ref().unwrap().parse::<u16>() {
              return true;
            } else {
            // second input is not a u16, so we need to check if it has a signal
              return gate_has_signal(&gate.input2.as_ref().unwrap(), circuit_map);
            }
        }
    }
  }
  false
}

fn evaluate_gate(input1 : u16, input2 : u16, instruction : Instruction) -> Option<u16> {
  match instruction {
    Instruction::AND => Some(input1 & input2),
    Instruction::OR => Some(input1 | input2),
    Instruction::LSHIFT => Some(input1 << input2),
    Instruction::RSHIFT => Some(input1 >> input2),
    _ => None
  }
}

fn part1() -> i32 {
  let instructions = utils::load_input_lines_as_vec_str("src/7.txt");
  let mut circuit_map : HashMap<String, gate> = HashMap::new();
  let mut n = 0;
  for i in instructions {
    let v: Vec<&str> = i.split(" -> ").collect();
    let id = v[1];
    let instruction_string = v[0];
    // if instruction string is an integer, then it's a value
    if let Ok(value) = instruction_string.parse::<u16>() {
      let g = gate {
        input1: instruction_string.to_string(),
        input2: None,
        instruction: None,
        signal: Some(value)
      };
      circuit_map.insert(id.to_string(), g);
      continue;
    } else {
      // otherwise, it's a gate
      let v2: Vec<&str> = instruction_string.split(" ").collect();

      if v2[0] == "NOT" {
        let g = gate {
          input1: v2[1].to_string(),
          input2: None,
          instruction: Some(Instruction::NOT),
            // if the circuit_map has a gate with the same id as input1, and it has a signal, our signal
            // will be the negation of that one
          signal: if let Some(g2) = circuit_map.get(&v2[1].to_string()) {
                    if let Some(s) = g2.signal {
                      Some(!s)
                    } else {
                      None
                    }
                  } else {
                    None
                  }
        };
        circuit_map.insert(id.to_string(), g);
        continue;
      }

      // lx -> a
      if v2.len() == 1 {
        // check if the circuit already has a gate with id v2[0]
        // if it does, then we need to copy that gate to the gate with id id
        // if it doesn't, then we need to create a new gate with id id
        if let Some(g) = circuit_map.get(&v2[0].to_string()) {
          circuit_map.insert(id.to_string(), g.clone());
        } else {
          let g = gate {
            input1: v2[0].to_string(),
            input2: None,
            instruction: None,
            signal: None
          };
          circuit_map.insert(id.to_string(), g);
        }
        continue;
      }

      //println!("i: {}. v2: {:?}", i, v2);

      let input1 = v2[0];
      let input2 = v2[2];
      // operator/instruction in the middle
      let instruction_opt:Option<Instruction> = match v2[1] {
        "AND" => Some(Instruction::AND),
        "OR" => Some(Instruction::OR),
        "LSHIFT" => Some(Instruction::LSHIFT),
        "RSHIFT" => Some(Instruction::RSHIFT),
        _ => None
      };
      //println!("instruction: {:?}", v2[1]);
      //println!("instruction: {:?}", instruction);

        // Easy case, both inputs are integers
        if let Ok(val1) = input1.parse::<u16>() {
            if let Ok(val2) = input2.parse::<u16>() {
                let g = gate {
                    input1: input1.to_string(),
                    input2: Some(input2.to_string()),
                    instruction: instruction_opt,
                    signal: evaluate_gate(val1, val2, instruction_opt.unwrap())
                };
                circuit_map.insert(id.to_string(), g);
            }
        }
        // input1 is u16 but input2 points to gate and it has signal
        else if let Ok(val1) = input1.parse::<u16>() {
           if gate_has_signal(&input2.to_string(), &circuit_map) {
           let gate2 = circuit_map.get(&input2.to_string()).unwrap();
           let val2 = gate2.signal.unwrap();
            let g = gate {
                input1: input1.to_string(),
                input2: Some(input2.to_string()),
                instruction: instruction_opt,
                signal: evaluate_gate(val1, val2, instruction_opt.unwrap())
            };
            circuit_map.insert(id.to_string(), g);
           }
        }
        // input2 is u16 but input1 points to gate and it has signal
        else if let Ok(val2) = input2.parse::<u16>() {
           if gate_has_signal(&input1.to_string(), &circuit_map) {
           let gate1 = circuit_map.get(&input1.to_string()).unwrap();
           let val1 = gate1.signal.unwrap();
            let g = gate {
                input1: input1.to_string(),
                input2: Some(input2.to_string()),
                instruction: instruction_opt,
                signal: evaluate_gate(val1, val2, instruction_opt.unwrap())
            };
            circuit_map.insert(id.to_string(), g);
           }
        }
        // input1 and input2 point to gates, and both have signals
        else if gate_has_signal(&input1.to_string(), &circuit_map) && gate_has_signal(&input2.to_string(), &circuit_map) {
           let gate1 = circuit_map.get(&input1.to_string()).unwrap();
           let gate2 = circuit_map.get(&input2.to_string()).unwrap();
           let val1 = gate1.signal.unwrap();
           let val2 = gate2.signal.unwrap();
            let g = gate {
                input1: input1.to_string(),
                input2: Some(input2.to_string()),
                instruction: instruction_opt,
                signal: evaluate_gate(val1, val2, instruction_opt.unwrap())
            };
            circuit_map.insert(id.to_string(), g);
        }
        // input1 and input2 point to gates, but one or both don't have signals
        else if !gate_has_signal(&input1.to_string(), &circuit_map) || !gate_has_signal(&input2.to_string(), &circuit_map) {
            let g = gate {
                input1: input1.to_string(),
                input2: Some(input2.to_string()),
                instruction: instruction_opt,
                signal: None
            };
            circuit_map.insert(id.to_string(), g);
        }
    }
  } // for

  // list of ids that need evaluating/updating
  let ids_to_update: Vec<String> = circuit_map
    .iter()
    .filter(|(_, gate)| gate.signal.is_none())
    .map(|(id, _)| id.clone())
    .collect();

  // evaluate
  for id in ids_to_update {
      let mut signal: Option<u16> = None;
       if gate_has_signal_on_both_inputs(&id, &circuit_map) {
          let gate = circuit_map.get(&id).unwrap();
          let g1 = circuit_map.get(&gate.input1.to_string()).unwrap();
          let g2 = circuit_map.get(&gate.input2.as_ref().unwrap().to_string()).unwrap();
          signal = evaluate_gate(g1.signal.unwrap(), g2.signal.unwrap(), gate.instruction.unwrap());
      }

      if signal.is_some() {
          let gate = circuit_map.get_mut(&id).unwrap();
          gate.signal = signal;
      }
  }


  // get all gate names in alphabetical order into a vector
    let mut gate_names: Vec<String> = circuit_map
        .iter()
        .map(|(id, _)| id.clone())
        .collect();
  // sort gate_names alphabetically, ascending
  gate_names.sort();

  // print all gate names and their signals
    for name in gate_names {
        let gate = circuit_map.get(&name).unwrap();
        println!("{}: {:?}", name, gate.signal.unwrap());
    }

0
} // part1

fn main() {
 println!("Day 7, Part 1: {}", part1());
 println!("Day 7, Part 2: {}", "TODO");
}