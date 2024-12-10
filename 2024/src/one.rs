use aoc::utils::load_input_lines_as_vec_str;

fn turn_tuples_into_2_vecs(input: Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
    input.into_iter().unzip()
}

fn part1(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).sum()
    // Part 1: 2057374
}

fn part2(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    // Part 2: 23177084
    let histogram_on_b = b
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    a.iter()
        .map(|x| x * histogram_on_b.get(x).unwrap_or(&0))
        .sum()
}

fn main() {
    let lines = load_input_lines_as_vec_str("inputs/1.txt");
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

    println!("{}", format!("Part 1: {}", part1(&a, &b)));
    println!("{}", format!("Part 2: {}", part2(&a, &b)));
}
