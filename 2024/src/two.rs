use aoc::utils::{load_input_lines_as_vec_str, split_and_parse};

fn is_level_safe(level: &str) -> bool {
    let result_vec_int: Result<Vec<i32>, _> = split_and_parse(level, " ");
    let vec_int = match result_vec_int {
        Ok(v) => v,
        Err(_) => return false
    };
    let mut i = 0;
    let mut j = 1;
    let len = vec_int.len();
    let mut increasing = true;
    while j < len {
        let a = vec_int[i];
        let b = vec_int[j];

        if i == 0 {
            if a > b {
                increasing = false;
            }
        }

        if increasing {
            if a >= b {
                return false;
            }
        } else {
            if a <= b {
                return false;
            }
        }

        if b.abs_diff(a) > 3 {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}
fn part1(lines: &Vec<String>) -> i32 {
    lines.iter().filter(|line| is_level_safe(line)).count() as i32
}

fn main() {
    let lines = load_input_lines_as_vec_str("2.txt");
    println!("{}", format!("Part 1: {}", part1(&lines)));
}