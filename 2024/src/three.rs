use aoc::utils::load_input_as_string;

// Part 1: 166905464
fn part1(input: &str) -> i32 {
    let regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let x = regex.captures_iter(&input);
    x.into_iter()
        .map(|matched| {
            let a = matched.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = matched.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>()
}

fn main() {
    let input = load_input_as_string("3.txt");
    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("Part 1: {}", part1(input.as_str()));
}
