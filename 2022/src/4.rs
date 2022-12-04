use aoc_2022::utils;

macro_rules! parse_next_part_to_i32 {
    ($parts:expr) => {$parts.next().unwrap().to_string().parse::<i32>().unwrap()}
}

fn main() {
    let vec = utils::load_input_lines_as_vec_str("4.txt");
    println!("Day 4: Camp Cleanup");
    println!("part1: {}", part1(&vec)); //part1: 536
    println!("part2: {}", part2(&vec)); //part2: 845
}

type Section = [i32; 2];

fn part1(sections: &Vec<String>) -> i32 {
    return sections.iter().map(|assignments| {
        if section_contains_another(sections_from_assignments(assignments)) {
            return 1;
        }
        0
    }).sum();
}

fn part2(sections: &Vec<String>) -> i32 {
    return sections.iter().map(|assignments| {
        if section_overlaps_another(sections_from_assignments(assignments)) {
            return 1;
        }
        0
    }).sum();
}

fn sections_from_assignments(assignments: &String) -> (Section, Section) {
    let mut parts = assignments.split(",");
    return (section_from_str(parts.next().unwrap()), section_from_str(parts.next().unwrap()));
}

fn section_from_str(segment_str: &str) -> Section {
    let mut parts = segment_str.split("-");
    let start: i32 = parse_next_part_to_i32!(parts);
    let end: i32 = parse_next_part_to_i32!(parts);
    return [start, end];
}

fn section_contains_another((a, b): (Section, Section)) -> bool {
    return a[0] <= b[0] && b[1] <= a[1] || b[0] <= a[0] && a[1] <= b[1];
}

fn section_overlaps_another((a, b): (Section, Section)) -> bool {
    return !(a[1] < b[0] || a[0] > b[1]);
}