use aoc_2015::utils;

fn part1() {
    let inputs = utils::load_input_lines_as_vec_str("src/8.txt");
    let mut total_code_chars = 0;
    let mut total_mem_chars = 0;
    for input in inputs {
        total_code_chars += input.len();
        let mut chars = input.chars();
        let mut mem_chars = 0;
        while let Some(c) = chars.next() {
            if c == '\\' {
                let next = chars.next().unwrap();
                if next == 'x' {
                    chars.next();
                    chars.next();
                }
            }
            mem_chars += 1;
        }
        total_mem_chars += mem_chars - 2;
    }
    println!("Part 1: {}", total_code_chars - total_mem_chars);
}
fn main() {
    part1(); // Part 1: 1371
}
