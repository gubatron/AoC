use aoc::utils::load_input_as_string;
use itertools::Itertools;
// For generating operator permutations
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

/// Recursively check if the target value can be achieved
fn can_match_target(
    numbers: &[i64],
    target: i64,
    current_index: usize,
    current_value: i64,
) -> bool {
    // Base case: If we've used all numbers, check if the result matches the target
    if current_index == numbers.len() {
        return current_value == target;
    }

    // Recursive case: Try both operators for the next number
    let next_number = numbers[current_index];
    // Try addition
    if can_match_target(numbers, target, current_index + 1, current_value + next_number) {
        return true;
    }
    // Try multiplication
    if can_match_target(numbers, target, current_index + 1, current_value * next_number) {
        return true;
    }

    false
}

/// Check if the equation is valid for the given test value using backtracking
fn is_equation_valid(test_value: i64, numbers: &[i64]) -> bool {
    // Start the recursive evaluation with the first number
    can_match_target(numbers, test_value, 1, numbers[0])
}

// /// Generate all possible operator permutations for the given number of spaces
// fn generate_operator_permutations(spaces: usize) -> Vec<Vec<char>> {
//     let operators = vec!['+', '*'];
//     operators
//         .into_iter()
//         .combinations_with_replacement(spaces)
//         .flat_map(|ops| ops.into_iter().permutations(spaces))
//         .unique()
//         .collect()
// }

/// Evaluate the equation with the given operators to see if it matches the test value
fn evaluate_equation(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for (i, &operator) in operators.iter().enumerate() {
        let next_number = numbers[i + 1];
        match operator {
            '+' => result += next_number,
            '*' => result *= next_number,
            _ => panic!("Unknown operator"),
        }
    }
    result
}

// /// Check if the equation is valid for the given test value
// fn is_equation_valid_old(test_value: i64, numbers: &[i64]) -> bool {
//     let spaces = numbers.len() - 1;
//     let operator_permutations = generate_operator_permutations(spaces);
//
//     for operators in operator_permutations {
//         if evaluate_equation(numbers, &operators) == test_value {
//             return true;
//         }
//     }
//     false
// }

/// Part 1: Sum the test values of valid equations
fn part1(input: &str) -> i64 {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|(test_value, numbers)| is_equation_valid(*test_value, numbers))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn main() {
    let input = load_input_as_string("7.txt"); // Replace with your input file path
    let result = part1(input.as_str());
    // Part 1: 2437272016585
    println!("Part 1: {}", result);
}
