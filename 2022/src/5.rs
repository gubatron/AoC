use std::collections::{HashMap, VecDeque};
use crate::ParsingStage::{CRATES, MOVES};

#[derive(PartialEq)]
enum ParsingStage {
    CRATES,
    MOVES,
}

fn main() {
    println!("Day 5: Supply Stacks");
    let lines = aoc_2022::utils::load_input_lines_as_vec_str("5.txt");
    let mut parsing_stage = CRATES;

    let mut crates = HashMap::<i32, VecDeque<char>>::new();
    let mut crates2 = HashMap::<i32, VecDeque<char>>::new();
    let mut moves = Vec::<[i32; 3]>::new();

    for line in lines {
        if parsing_stage == CRATES {
            assign_crates_to_stacks_from_line(&line, &mut crates);
            assign_crates_to_stacks_from_line(&line, &mut crates2);
        }

        if line.is_empty() || parsing_stage == CRATES && line.starts_with(" 1") {
            parsing_stage = MOVES;
            continue;
        }

        if parsing_stage == MOVES {
            let data = extract_three_integers(&line);
            moves.push(data);
        }
    }
    println!("part 1: {}", part1(&mut crates, &moves)); // VPCDMSLWJ
    println!("part 2: {}", part2(&mut crates2, &moves)); // TPWCGNCCG
}

fn assign_crates_to_stacks_from_line(line: &String, crates: &mut HashMap<i32, VecDeque<char>>) {
    let crates_set = find_upper_case_characters(&*line);
    for (crate_letter, char_offset) in crates_set.iter() {
        let stack_number = ((char_offset / (4 as usize)) as i32) + 1;
        if crates.contains_key(&stack_number) {
            crates.get_mut(&stack_number).unwrap().push_back(*crate_letter);
        } else {
            let mut stack = VecDeque::<char>::new();
            stack.insert(0, *crate_letter);
            crates.insert(stack_number, stack);
        }
    }
}

fn find_upper_case_characters(text: &str) -> std::collections::HashSet<(char, usize)> {
    let mut positions = std::collections::HashSet::new();
    for (i, c) in text.chars().enumerate() {
        if c.is_uppercase() {
            positions.insert((c, i));
        }
    }
    positions
}

fn extract_three_integers(text: &str) -> [i32; 3] {
    let mut numbers = [0, 0, 0];
    let mut i = 0;
    for word in text.split_whitespace() {
        if let Ok(n) = word.parse::<i32>() {
            numbers[i] = n;
            i += 1;
            if i == 3 {
                break;
            }
        }
    }
    numbers
}

fn top_crate_letters(crates: &HashMap<i32, VecDeque<char>>) -> String {
    let mut result = "".to_string();
    let num_crates = crates.len() as i32;
    let mut i = 1 as i32;
    while i <= num_crates {
        let c = crates.get(&i).unwrap().get(0).unwrap();
        result.push(*c);
        i += 1;
    }
    return result;
}

fn part1(crates: &mut HashMap<i32, VecDeque<char>>, moves: &Vec<[i32; 3]>) -> String {
    for data in moves.iter() {
        let (mut num_crates_to_move, from, to) = (data[0], data[1], data[2]);
        while num_crates_to_move > 0 {
            let crate_letter = crates.get_mut(&from).unwrap().pop_front().unwrap();
            crates.get_mut(&to).unwrap().push_front(crate_letter);
            num_crates_to_move -= 1;
        }
    }
    top_crate_letters(&crates)
}

fn part2(crates: &mut HashMap<i32, VecDeque<char>>, moves: &Vec<[i32; 3]>) -> String {
    for data in moves.iter() {
        let (mut num_crates_to_move, from, to) = (data[0], data[1], data[2]);
        let mut temp_stack = VecDeque::<char>::new();
        while num_crates_to_move > 0 {
            let crate_letter = crates.get_mut(&from).unwrap().pop_front().unwrap();
            temp_stack.push_back(crate_letter);
            num_crates_to_move -= 1;
        }
        while !temp_stack.is_empty() {
            crates.get_mut(&to).unwrap().push_front(temp_stack.pop_back().unwrap());
        }
    }
    top_crate_letters(&crates)
}