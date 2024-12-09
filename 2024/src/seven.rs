use aoc::utils::load_input_as_string;
use std::str::FromStr;

/// Parse the input into a vector of tuples: `(test_value, numbers)`
fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() != 2 {
                return None;
            }
            let test_value = i64::from_str(parts[0].trim()).ok()?;
            let numbers = parts[1]
                .split_whitespace()
                .filter_map(|n| i64::from_str(n).ok())
                .collect();
            Some((test_value, numbers))
        })
        .collect()
}


/// Calculate the number of digits in a number
fn num_digits(n: i64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}


/// Recursively check if the target value can be achieved
fn can_match_target(
    numbers: &[i64],
    target: i64,
    current_index: usize,
    current_value: i64,
    enable_concatenation: bool,
) -> bool {
    // Base case: If we've used all numbers, check if the result matches the target
    if current_index == numbers.len() {
        return current_value == target;
    }

    // Recursive case: Try all operators for the next number
    let next_number = numbers[current_index];

    // Try addition
    if can_match_target(numbers, target, current_index + 1, current_value + next_number, enable_concatenation) {
        return true;
    }

    // Try multiplication
    if can_match_target(numbers, target, current_index + 1, current_value * next_number, enable_concatenation) {
        return true;
    }

    // Try concatenation if enabled
    if enable_concatenation {
        let concatenated = current_value * 10i64.pow(num_digits(next_number)) + next_number;
        if can_match_target(numbers, target, current_index + 1, concatenated, enable_concatenation) {
            return true;
        }
    }

    false
}


/// Check if the equation is valid for the given test value using backtracking
fn is_equation_valid(test_value: i64, numbers: &[i64], enable_concatenation: bool) -> bool {
    // Start the recursive evaluation with the first number
    can_match_target(numbers, test_value, 1, numbers[0], enable_concatenation)
}

/// Part 1: Sum the test values of valid equations
fn part1(input: &str) -> i64 {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|(test_value, numbers)| is_equation_valid(*test_value, numbers, false))
        .map(|(test_value, _)| test_value)
        .sum()
}

/// Part 2: Sum the test values of valid equations with all three operators
fn part2(input: &str) -> i64 {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|(test_value, numbers)| is_equation_valid(*test_value, numbers, true))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn main() {
    let input = load_input_as_string("7.txt"); // Replace with your input file path
    println!("Part 1: {}", part1(input.as_str())); // Part 1: 2437272016585
    println!("Part 2: {}", part2(input.as_str())); // Part 2: 162987117690649
}
