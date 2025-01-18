use aoc::utils;
use std::cmp::PartialEq;

#[derive(PartialEq)]
#[derive(Debug)]
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

#[derive(Debug)]
struct CPU {
    inst_ptr: usize, // instruction pointer
    a: i32,        // 32-bit register
    b: i32,
    c: i32,
    opcode: Opcode,
    operand: i32, // 3-bit number
    instructions: Vec<i32>,
    output: Vec<i32>,
}

fn int_to_opcode(opcode: i32) -> Opcode {
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

fn resolve_operand(cpu: &CPU, operand: i32) -> i32 {
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

    let register_vector: Vec<i32> = inputs[0]
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

    let instructions: Vec<i32> = inputs[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    CPU {
        inst_ptr: 0,
        a: register_vector[0],
        b: register_vector[1],
        c: register_vector[2],
        opcode: Opcode::NIL,
        operand: 0,
        instructions,
        output: vec![],
    }
}

fn part1() -> String {
    let mut cpu = load_cpu("inputs/17.txt");
    while cpu.inst_ptr < cpu.instructions.len()-1 {
        println!("CPU: {:?}", cpu);
        println!("inst_ptr: {}", cpu.inst_ptr);
        println!("total instructions: {}", cpu.instructions.len());
        println!();
        let opcode_i32= cpu.instructions[cpu.inst_ptr];
        let operand_i32= cpu.instructions[cpu.inst_ptr + 1];
        cpu.opcode = int_to_opcode(opcode_i32);
        cpu.operand = resolve_operand(&cpu, operand_i32);

        match cpu.opcode {
            Opcode::ADV => {
                // A division
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
                //cpu.output.push(utils::euclidean_modulo(cpu.operand, 8));
                cpu.output.push(cpu.operand & 7);
            }
            Opcode::BDV => {
                // B division A/2^operand into B
                cpu.b = cpu.a >> cpu.operand;
            }
            Opcode::CDV => {
                // C division A/2^operand into C
                cpu.c = cpu.a >> cpu.operand;
            }
            Opcode::NIL => {}
        }

        cpu.inst_ptr += 2;
    }

    cpu.output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() {
    println!("Part 1: {}", part1()); //3,6,3,7,0,7,0,3,0
}