use aoc::utils;
use std::cmp::PartialEq;

#[derive(PartialEq, Debug, Clone)]
enum Opcode {
    ADV = 0, // A division, num in register A, denominator is operand^2, result is truncated int and saved into a
    BXL = 1, // bitwise XOR of register B and instruction's literal operand, result stored into B
    BST = 2, // combo operand % 8, into B
    JNZ = 3, // do nothing if A==0. Otherwise, jump by value of literal operand, if it jumps IP is NOT increased by 2
    BXC = 4, // bitwise XOR of B and C, result stored into B, reads the operand but ignores it
    OUT = 5, // combo operand % 8, then outputs value. Multiple values are separated by commas
    BDV = 6, // B division, num in a, denominator is operand^2, result into B
    CDV = 7, // C division, num in a, denominator is operand^2, result into C
    NIL = 8, // no operation
}

#[derive(Debug, Clone)]
struct CPU {
    inst_ptr: usize, // instruction pointer
    a: u64,          // 32-bit register
    b: u64,
    c: u64,
    opcode: Opcode,
    operand: u64, // 3-bit number
    instructions: Vec<u64>,
    output: Vec<u64>,
    aborted: bool,
}

fn int_to_opcode(opcode: u64) -> Opcode {
    match opcode {
        0 => Opcode::ADV,
        1 => Opcode::BXL,
        2 => Opcode::BST,
        3 => Opcode::JNZ,
        4 => Opcode::BXC,
        5 => Opcode::OUT,
        6 => Opcode::BDV,
        7 => Opcode::CDV,
        8 => Opcode::NIL,
        _ => Opcode::NIL,
    }
}

fn resolve_operand(cpu: &CPU, operand: u64) -> u64 {
    if cpu.opcode == Opcode::BXL || cpu.opcode == Opcode::JNZ {
        return operand;
    }

    match operand {
        0..=3 => operand,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        7 => panic!("Operand cannot be 7"),
        _ => panic!("Operand cannot be {}", operand),
    }
}

fn load_cpu(input_file: &str) -> CPU {
    let inputs = utils::load_input_break_by_empty_lines_as_vec_str(input_file);

    let register_vector: Vec<u64> = inputs[0]
        .split("\n")
        .map(|s| {
            s.to_string()
                .split(": ")
                .nth(1)
                .unwrap()
                .to_string()
                .parse()
                .unwrap()
        })
        .collect();

    let instructions: Vec<u64> = inputs[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    CPU {
        inst_ptr: 0,
        a: register_vector[0],
        b: register_vector[1],
        c: register_vector[2],
        opcode: Opcode::NIL,
        operand: 0,
        instructions,
        output: vec![],
        aborted: false,
    }
}

fn u64vec_as_string(u64_vec: &Vec<u64>) -> String {
    u64_vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run(cpu: &mut CPU, opt_expected_output: Option<String>) -> Vec<u64> {
    while cpu.inst_ptr < cpu.instructions.len() - 1 {
        let opcode_i32 = cpu.instructions[cpu.inst_ptr];
        let operand_i32 = cpu.instructions[cpu.inst_ptr + 1];
        cpu.opcode = int_to_opcode(opcode_i32);
        cpu.operand = resolve_operand(&cpu, operand_i32);

        match cpu.opcode {
            Opcode::ADV => {
                // A division
                // cpu.a = cpu.a / 2_i32.pow(cpu.operand as u32);
                // 8>>1 == 8/2 == 4
                // 8>>2 == 8/4 == 2
                // 8>>3 == 8/8 == 1
                cpu.a >>= cpu.operand;
            }
            Opcode::BXL => {
                // bitwise XOR
                cpu.b ^= cpu.operand;
            }
            Opcode::BST => {
                // combo operand % 8, into B
                cpu.b = cpu.operand & 7;
            }
            Opcode::JNZ => {
                // do nothing if A==0. Otherwise, jump by value of literal operand, if it jumps IP is NOT increased by 2
                if cpu.a != 0 {
                    cpu.inst_ptr = cpu.operand as usize;
                    continue;
                }
            }
            Opcode::BXC => {
                // bitwise XOR of B and C, result stored into B, reads the operand but ignores it
                cpu.b ^= cpu.c;
            }
            Opcode::OUT => {
                // combo operand % 8, then outputs value. Multiple values are separated by commas
                // cpu.output.push(utils::euclidean_modulo(cpu.operand, 8));
                // cpu.output.push(cpu.operand % 8);
                cpu.output.push(cpu.operand & 7);

                if let Some(expected_output) = &opt_expected_output {
                    let output_str = u64vec_as_string(&cpu.output);
                    // if the current output is not a prefix of the expected output, then we can stop
                    if !expected_output.starts_with(&output_str) {
                        cpu.aborted = true;
                        break;
                    }
                }
            }
            Opcode::BDV => {
                // B division A/2^operand into B
                // cpu.b = cpu.a / 2_i32.pow(cpu.operand as u32);
                cpu.b = cpu.a >> cpu.operand;
            }
            Opcode::CDV => {
                // C division A/2^operand into C
                // cpu.c = cpu.a / 2_i32.pow(cpu.operand as u32);
                cpu.c = cpu.a >> cpu.operand;
            }
            Opcode::NIL => {}
        }

        cpu.inst_ptr += 2;
    }

    cpu.output.clone()
}

fn part1() -> String {
    let mut cpu = load_cpu("inputs/17.txt");
    let result = run(&mut cpu, None);
    println!("CPU: {:?}", cpu);
    u64vec_as_string(&result)
}
// naive bruteforce approach, works only for the test input
// fn part2() -> u64 {
//     let cpu_original = load_cpu("inputs/17.2.test.txt");
//
//     // make a copy of the original cpu state, so we can run it multiple times
//     // without loading from disk
//     let mut nonce = 2*6900555780;//cpu_original.a;
//     let expected_output = u64vec_as_string(&cpu_original.instructions);
//     println!();
//     println!("ORIGINAL CPU: {:?}, A: {}, expected output: {:?}", cpu_original, nonce, expected_output);
//     loop {
//         let mut cpu = CPU {
//             inst_ptr: cpu_original.inst_ptr,
//             a: nonce,
//             b: cpu_original.b,
//             c: cpu_original.c,
//             opcode: cpu_original.opcode.clone(),
//             operand: cpu_original.operand,
//             instructions: cpu_original.instructions.clone(),
//             output: vec![],
//             aborted: false
//         };
//
//         run(&mut cpu, Some(expected_output.clone()));
//
//         if cpu.instructions != cpu.output {
//             nonce += 1;
//         } else {
//             println!("Got it! input nonce on register a: {}", nonce);
//             println!("CPU: {:?}", cpu);
//             return nonce;
//         }
//     }
// }

fn part2_quine() -> u64 {
    let cpu_original = load_cpu("inputs/17.txt");
    let full_program = &cpu_original.instructions;
    let n = full_program.len();

    // We'll define a helper that:
    //   1) Clones the original CPU,
    //   2) Sets A to our candidate,
    //   3) Runs the program,
    //   4) Returns the entire output.
    fn run_with_a(cpu_orig: &CPU, candidate_a: u64) -> Vec<u64> {
        let mut cpu = cpu_orig.clone();
        cpu.a = candidate_a;
        run(&mut cpu, None)
    }

    let mut a: u64 = 0; // We'll build from right to left
    for i in (0..n).rev() {
        // Shift A by 3 bits (multiply by 8)
        a <<= 3;

        // Now find the 3 bits that make the suffix match `program[i..]`
        loop {
            let out = run_with_a(&cpu_original, a);
            // Compare output with program[i..]
            // if they match, break
            if out.as_slice() == &full_program[i..] {
                break;
            }
            a += 1;
        }
    }

    a
}

fn main() {
    println!("Part 1: [{}]", part1()); // 3,6,3,7,0,7,0,3,0
    println!("Part 2: [{}]", part2_quine()); // 136904920099226 test:117440
}
