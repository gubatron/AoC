fn transform(input: &str) -> String {
    let mut current_char = ' ';
    let mut count = 0;
    let mut output = String::new();

    for ch in input.chars() {
        if current_char != ch {
            if count > 0 {
                output.push_str(&count.to_string());
                output.push(current_char);
            }
            current_char = ch;
            count = 1;
        } else {
            count += 1;
        }
    }
    // Add the last group to the output
    output.push_str(&count.to_string());
    output.push(current_char);
    output
}

fn test_1() {
    assert_eq!(transform("1"), "11");
    assert_eq!(transform("11"), "21");
    assert_eq!(transform("21"), "1211");
    assert_eq!(transform("1211"), "111221");
    assert_eq!(transform("111221"), "312211");

    let mut input = "1".to_string();
    for _ in 0..5 {
        input = transform(&input);
    }
    assert_eq!(input, "312211");
}

fn part1() -> String{
    let mut input = "1113122113".to_string();
    for _ in 0..40 {
        input = transform(&input);
    }
    println!("Part 1: {}", input.to_string().len());
    input.to_string()
}

fn conway_length(initial_length: f64, iterations: u32) -> u32 {
    const CONWAY_CONSTANT: f64 = 1.303577269; // Conway's constant
    let final_length = initial_length * CONWAY_CONSTANT.powi(iterations as i32);
    final_length.round() as u32 // Round to nearest integer 5712667 too high, 5103358 too low
}

fn part2(mut input: String) {
    for _ in 0..10 {
        input = transform(&input);
    }
    println!("Part 2: {}", input.len());
}

fn main() {
    let transformed = part1();
    part2(transformed);
}
