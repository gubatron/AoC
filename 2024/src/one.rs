use aoc::utils::load_input_lines_as_vec_str;

fn turn_tuples_into_2_vecs(input: Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (x, y) in input {
        a.push(x);
        b.push(y);
    }
    (a, b)
}

fn part1(a: Vec<u32>, b: Vec<u32>) -> u32 {
    a.iter().zip(b).map(|(x, y)| x.abs_diff(y)).sum()
}

fn main() {
    let lines = load_input_lines_as_vec_str("1.txt");
    let tuples: Vec<(u32, u32)> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            (
                parts[0].to_string().parse::<u32>().unwrap(),
                parts[1].to_string().parse::<u32>().unwrap(),
            )
        })
        .collect();
    let mut a_b_tuple = turn_tuples_into_2_vecs(tuples);
    a_b_tuple.0.sort();
    a_b_tuple.1.sort();
    let (a, b) = a_b_tuple;

    println!("{}", format!("Part 1: {}", part1(a, b)));
}
