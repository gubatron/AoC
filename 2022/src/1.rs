use aoc::utils;

fn part1(calories_list: &Vec<String>) {
    let mut top_max_calories = 0;
    let mut current_max_calories = 0;
    let last_line_index = calories_list.len() - 1;
    let mut current_line = 0;
    calories_list.iter().for_each(|n_str| {
        let n_str = n_str.trim();
        if n_str.is_empty() {
            if current_max_calories > top_max_calories {
                top_max_calories = current_max_calories;
            }
            current_max_calories = 0;
        } else {
            current_max_calories = current_max_calories + n_str.parse::<i32>().unwrap();
            if current_line == last_line_index && current_max_calories > top_max_calories {
                top_max_calories = current_max_calories;
            }
        }
        current_line = current_line + 1;
    });
    print!("part1: {}\n", top_max_calories);
}

fn part2(calories_list: &Vec<String>) {
    let mut current_max_calories = 0;
    let mut elf_calories = vec![];
    let last_line_index = calories_list.len() - 1;
    let mut current_line = 0;
    calories_list.iter().for_each(|n_str| {
        let n_str = n_str.trim();
        if n_str.is_empty() {
            elf_calories.push(current_max_calories);
            current_max_calories = 0;
        } else {
            current_max_calories = current_max_calories + n_str.parse::<i32>().unwrap();
            if current_line == last_line_index {
                elf_calories.push(current_max_calories);
            }
        }
        current_line = current_line + 1;
    });
    elf_calories.sort();
    elf_calories.reverse();
    print!("part2 {}\n", elf_calories.drain(0..3).collect::<Vec<i32>>().into_iter().sum::<i32>());
}

fn main() {
    print!("Advent of Code 2022, day 1\n\n");
    let calories_list = &utils::load_input_lines_as_vec_str("1.txt");
    part1(calories_list); // part1: 72070
    part2(calories_list); // part2 211805
}
