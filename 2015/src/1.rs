use aoc_2015::utils;

fn part1() {
    let vec_char = utils::load_input_as_vec_char(String::from("src/1.txt"));
    let mut current_floor = 0;
    vec_char.iter().for_each(|c| match *c {
        '(' => current_floor += 1,
        ')' => current_floor -= 1,
        _ => {}
    });
    println!("1. part 1: Current floor is {}", current_floor);
}

fn part2() {
    let vec_char = utils::load_input_as_vec_char(String::from("src/1.txt"));
    let mut current_floor = 0;
    let mut current_character = 1;
    for c in vec_char {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => {}
        }
        if current_floor == -1 {
            break;
        }
        current_character += 1;
    }
    println!(
        "1. part 2: Current floor is {} and first character to get us there is {}",
        current_floor, current_character
    );
}

fn main() {
    part1();
    part2();
}
