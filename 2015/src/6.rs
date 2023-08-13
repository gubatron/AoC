use aoc_2015::utils;

#[allow(dead_code)]
struct Light {
    x: i32,
    y: i32,
    on: bool,
}

#[allow(dead_code)]
struct Light2 {
    x: i32,
    y: i32,
    brightness: i32,
}

#[derive(Debug)]
enum InstructionCommand {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    command: InstructionCommand,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn build_light_matrix() -> Vec<Vec<Light>> {
    let mut light_matrix = Vec::new();
    for _ in 0..1000 {
        let mut row = Vec::new();
        for _ in 0..1000 {
            row.push(Light {
                x: 0,
                y: 0,
                on: false,
            });
        }
        light_matrix.push(row);
    }
    light_matrix
}

fn build_light2_matrix() -> Vec<Vec<Light2>> {
    let mut light_matrix = Vec::new();
    for _ in 0..1000 {
        let mut row = Vec::new();
        for _ in 0..1000 {
            row.push(Light2 {
                x: 0,
                y: 0,
                brightness: 0,
            });
        }
        light_matrix.push(row);
    }
    light_matrix
}

/**
 * Parse a string like "turn on 0,0 through 999,999" into an Instruction struct
 */
fn parse_instruction(s: &str) -> Instruction {
    let mut instruction = Instruction {
        command: InstructionCommand::TurnOn,
        start_x: 0,
        start_y: 0,
        end_x: 0,
        end_y: 0,
    };
    let mut words = s.split_whitespace();
    let command = words.next().unwrap();
    match command {
        "turn" => {
            let command = words.next().unwrap();
            match command {
                "on" => instruction.command = InstructionCommand::TurnOn,
                "off" => instruction.command = InstructionCommand::TurnOff,
                _ => panic!("Invalid command"),
            }
        }
        "toggle" => instruction.command = InstructionCommand::Toggle,
        _ => panic!("Invalid command"),
    }
    let start = words.next().unwrap();
    words.next();
    let end = words.next().unwrap();
    let mut start_coords = start.split(",");
    let mut end_coords = end.split(",");
    instruction.start_x = start_coords.next().unwrap().parse::<i32>().unwrap();
    instruction.start_y = start_coords.next().unwrap().parse::<i32>().unwrap();
    instruction.end_x = end_coords.next().unwrap().parse::<i32>().unwrap();
    instruction.end_y = end_coords.next().unwrap().parse::<i32>().unwrap();
    instruction
}

fn count_lights_on(light_matrix: &Vec<Vec<Light>>) -> i32 {
    let mut count = 0;
    for row in light_matrix {
        for light in row {
            if light.on {
                count += 1;
            }
        }
    }
    count
}

fn count_lights2_total_brightness(light_matrix: &Vec<Vec<Light2>>) -> i32 {
    let mut total_brightness = 0;
    for row in light_matrix {
        for light2 in row {
            if light2.brightness > 0 {
                total_brightness += light2.brightness;
            }
        }
    }
    total_brightness
}

fn part1() -> i32 {
    let mut light_matrix = build_light_matrix();
    let lines = utils::load_input_lines_as_vec_str("src/6.txt");
    // let lines: Vec<String> = vec![
    //     "turn on 0,0 through 999,999",
    //     "toggle 0,0 through 999,0",
    //     "turn off 499,499 through 500,500",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect();

    let instructions = lines
        .iter()
        .map(|l| parse_instruction(l))
        .collect::<Vec<Instruction>>();
    // apply instructions on light_matrix
    for i in instructions {
        for x in i.start_x..i.end_x + 1 {
            for y in i.start_y..i.end_y + 1 {
                match i.command {
                    InstructionCommand::TurnOn => light_matrix[x as usize][y as usize].on = true,
                    InstructionCommand::TurnOff => light_matrix[x as usize][y as usize].on = false,
                    InstructionCommand::Toggle => {
                        light_matrix[x as usize][y as usize].on =
                            !light_matrix[x as usize][y as usize].on
                    }
                }
            }
        }
        //println!("Lights on: {}", count_lights_on(&light_matrix));
    }
    count_lights_on(&light_matrix)
}

fn part2() -> i32 {
    let mut light2_matrix = build_light2_matrix();
    let lines = utils::load_input_lines_as_vec_str("src/6.txt");
    // let lines: Vec<String> = vec![
    //     "turn on 0,0 through 999,999",
    //     "toggle 0,0 through 999,0",
    //     "turn off 499,499 through 500,500",
    // ]
    // .iter()
    // .map(|s| s.to_string())
    // .collect();

    let instructions = lines
        .iter()
        .map(|l| parse_instruction(l))
        .collect::<Vec<Instruction>>();
    // apply instructions on light_matrix
    for i in instructions {
        for x in i.start_x..i.end_x + 1 {
            for y in i.start_y..i.end_y + 1 {
                match i.command {
                    InstructionCommand::TurnOn => {
                        light2_matrix[x as usize][y as usize].brightness += 1
                    }
                    InstructionCommand::TurnOff => {
                        light2_matrix[x as usize][y as usize].brightness =
                            core::cmp::max(0, light2_matrix[x as usize][y as usize].brightness - 1);
                    }
                    InstructionCommand::Toggle => {
                        light2_matrix[x as usize][y as usize].brightness += 2
                    }
                }
            }
        }
        // println!(
        //     "Lights2 total brightness: {}",
        //     count_lights2_total_brightness(&light2_matrix)
        // );
    }
    count_lights2_total_brightness(&light2_matrix)
}

fn main() {
    println!("Part 1: {}", part1()); // Part 1: 400410
    println!("Part 2: {}", part2()); // Part 2: 15343601
}
