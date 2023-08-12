use aoc_2015::utils;

fn part1() {
    let inputs: Vec<String> = utils::load_input_lines_as_vec_str(String::from("src/5.txt"));
    println!("Part 1: {}", inputs.iter().filter(|s| is_nice(s)).count());
}

fn contains_forbidden_sequence(s: &str) -> bool {
    let forbidden_sequences = vec!["ab", "cd", "pq", "xy"];
    for seq in forbidden_sequences {
        if s.contains(seq) {
            return true;
        }
    }
    false
}

fn is_nice(s: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut last_char = ' ';
    for c in s.chars() {
        if vowels.contains(&c) {
            vowel_count += 1;
        }
        if !has_double_letter && c == last_char {
            has_double_letter = true;
        }
        last_char = c;
    }
    vowel_count >= 3 && has_double_letter && !contains_forbidden_sequence(s)
}

fn has_pair_without_overlap(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        let pair = (chars[i], chars[i + 1]);
        for j in i + 2..chars.len() - 1 {
            if pair == (chars[j], chars[j + 1]) {
                return true;
            }
        }
    }
    false
}

fn has_repeat_with_one_between(s: &str) -> bool {
    s.chars()
        .enumerate()
        .any(|(i, c)| i >= 2 && s.chars().nth(i - 2) == Some(c))
}

fn is_nice2(s: &str) -> bool {
    has_pair_without_overlap(s) && has_repeat_with_one_between(s)
}

fn part2() {
    let inputs: Vec<String> = utils::load_input_lines_as_vec_str(String::from("src/5.txt"));
    println!("Part 2: {}", inputs.iter().filter(|s| is_nice2(s)).count());
}

fn main() {
    part1(); // Part 1: 258
    part2(); // Part 2: 53
}
