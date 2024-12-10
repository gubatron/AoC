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

fn part2(input: &str) -> i32 {
    let regex = regex::Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let captures = regex.captures_iter(&input);
    let mut sum = 0;
    let mut do_mul = true;
    let mul_group = 1;
    let mul_a_group = 2;
    let mul_b_group = 3;
    let do_group = 4;
    let dont_group = 5;
    for cap in captures {
        if let Some(_mul_match) = cap.get(mul_group) {
            if do_mul {
                let a = cap
                    .get(mul_a_group)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let b = cap
                    .get(mul_b_group)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                sum += a * b;
            }
        }
        if let Some(_) = cap.get(do_group) {
            do_mul = true;
        }
        if let Some(_) = cap.get(dont_group) {
            do_mul = false;
        }
    }
    sum
}

fn main() {
    let input = load_input_as_string("inputs/3.txt");
    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("Part 1: {}", part1(&input));
    //let input2_test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    println!("Part 2: {}", part2(&input));
}
