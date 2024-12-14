use aoc::utils::load_input_as_string;

fn transform(numbers: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();

    for i in 0..numbers.len() {
        let current_stone = numbers[i];
        if current_stone == 0 {
            result.push(1);
            continue;
        }
        let current_stone_str = current_stone.to_string();
        if current_stone_str.len() % 2 == 0 {
            let digits = current_stone_str.len();
            // get the left half of the number
            let left_half = current_stone_str[0..digits / 2].parse::<u64>().unwrap();
            // get the right half of the number
            let right_half = current_stone_str[digits / 2..].parse::<u64>().unwrap();
            result.push(left_half);
            result.push(right_half);
        } else {
            result.push(2024 * current_stone)
        }
    }
    result
}

fn part1(numbers: &Vec<u64>) -> u64 {
    let mut blinks = 25;
    let mut result: Vec<u64> = numbers.clone();
    while blinks > 0 {
        result = transform(&result);
        blinks -= 1;
    }
    result.len() as u64
}

fn part2(numbers: &Vec<u64>) -> u64 {
    let mut blinks = 75;
    let mut result: Vec<u64> = numbers.clone();
    while blinks > 0 {
        result = transform(&result);
        blinks -= 1;
    }
    result.len() as u64
}

fn main() {
    let input = load_input_as_string("inputs/11.txt");

    let numbers: Vec<u64> = input
        .split(&" ".to_string())
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part 1: {:?}", part1(&numbers));
    println!("Part 2: {:?}", part2(&numbers));
}

#[test]
fn test_transform() {
    let numbers = vec![0, 1, 10, 99, 999];
    let expected = [1, 2024, 1, 0, 9, 9, 2021976];
    println!("Initial arrangement:");
    println!("{:?}", &numbers);
    assert_eq!(transform(&numbers), expected);
    println!("Transformed arrangement: \n{:?}", &expected);

    let numbers2 = vec![125, 17];
    let expected2 = [
        2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2,
    ];
    let mut blinks = 6;
    let mut result: Vec<u64> = Vec::new();
    while blinks > 0 {
        if blinks == 6 {
            result = transform(&numbers2);
        } else {
            result = transform(&result);
        }
        blinks -= 1;
    }
    assert_eq!(result, expected2);
    blinks = 19;
    while blinks > 0 {
        result = transform(&result);
        blinks -= 1;
    }
    assert_eq!(55312, result.len());
}
