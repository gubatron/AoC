fn main() {
    // Day 10: Cathode-Ray Tube
    let mut addx_instructions: Vec<i32> = vec![];
    let program = aoc_2022::utils::load_input_lines_as_vec_str("10.txt");
    parse_program(program, &mut addx_instructions);
    let signal_strength_sum = execute_instructions(addx_instructions);

    // test: part 1: 13140
    // real: part 1: 12740
    println!("part 1: {}", signal_strength_sum);
}

fn parse_program(program: Vec<String>, addx_instructions: &mut Vec<i32>) {
    for line in program {
        let mut inst_parts = line.split(" ");
        let op = inst_parts.next().unwrap();
        match op {
            "noop" => {
                // add 0
                addx_instructions.push(0);
            }
            "addx" => {
                addx_instructions.push(0); // first cycle is like a noop.
                addx_instructions.push(inst_parts.next().unwrap().parse::<i32>().unwrap());
            }
            _ => {
                panic!("unknown instruction");
            }
        }
    }
}

fn execute_instructions(mut addx_instructions: Vec<i32>) -> i32 {
    let checkpoints = vec![20,60,100,140,180,220];
    let mut x = 1;
    let mut pc = 1; // maybe starts at 0, gotta test
    let mut signal_sum = 0;

    while addx_instructions.is_empty() == false {
        let addme = addx_instructions.remove(0);
        if checkpoints.contains(&pc) {
            signal_sum = signal_sum + (pc * x);
        }
        println!("pc@{}: x={} addme={}",pc, x, addme);
        x += addme;
        println!("pc@{}: x={}",pc, x);
        println!();
        pc += 1;
    }
    signal_sum
}