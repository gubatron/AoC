use num_bigint::{BigUint};

fn transform(x: BigUint) -> BigUint {
    let x_str : String = x.to_string();
    let mut current_number = ' ';
    let mut count = 0;
    let mut output = String::new();

    for x in x_str.chars() {
        if current_number != x {
            if count > 0 {
                output.push_str(&count.to_string());
                output.push(current_number);
            }
            current_number = x;
            count = 1;
        } else {
            count += 1;
        }
    }
    output.push_str(&count.to_string());
    output.push(current_number);
    output.parse::<BigUint>().unwrap()
}

fn test_1() {
    assert_eq!(transform(BigUint::try_from(1).unwrap()), BigUint::try_from(11).unwrap());
    assert_eq!(transform(BigUint::try_from(11).unwrap()), BigUint::try_from(21).unwrap());
    assert_eq!(transform(BigUint::try_from(21).unwrap()), BigUint::try_from(1211).unwrap());
    assert_eq!(transform(BigUint::try_from(1211).unwrap()), BigUint::try_from(111221).unwrap());
    assert_eq!(transform(BigUint::try_from(111221).unwrap()), BigUint::try_from(312211).unwrap());

    let mut input: BigUint = BigUint::try_from(1).unwrap();
    for _ in 0..5 {
        input = transform(input);
    }
    assert_eq!(input, BigUint::try_from(312211).unwrap());
}

fn part1() {
    let mut input = BigUint::try_from(1113122113).unwrap();
    for _ in 0..40 {
        input = transform(input);
    }
    println!("Part 1: {}", input.to_string().len());
}

fn main() {
  test_1();
  part1();
}