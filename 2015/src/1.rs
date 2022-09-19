use aoc_2015::utils;

fn part1() {
    let vec_char = utils::load_input_as_vec_char(String::from("src/1.txt"));
    let mut current_floor = 0;
    vec_char.iter().for_each(|c| {
        match *c {
            '(' =>current_floor += 1,
            ')' =>current_floor -= 1,
            _ => {}
        }
    });
    println!("1. part 1: Current floor is {}", current_floor);
}

fn main() {
    part1();
}