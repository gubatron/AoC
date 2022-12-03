use std::collections::{HashMap, HashSet};
use aoc_2022::utils;

fn main() {
    println!("day 3: Rucksack Reorganization");
    let rucksacks = utils::load_input_lines_as_vec_str("3.txt");
    part1(&rucksacks); // part1: 7826
    part2(&rucksacks); // part2: 2577
}

fn part1(rucksacks: &Vec<String>) {
    println!("part1: {}", rucksacks.iter().map(|items| {
        char_priority(find_common_char_in_each_half(items))
    }).sum::<u32>()); // part1: 7826
}

fn part2(rucksacks: &Vec<String>) {
    println!("part2: {}", rucksacks.
        iter().
        map(|item| { distinct_chars_only(item) }).
        collect::<Vec<String>>().
        chunks(3).
        map(|item_group| {
            let mut char_count_map: HashMap<char, u32> = HashMap::new();
            item_group.iter().for_each(|item| count_chars(&mut char_count_map, item));
            char_priority(char3x(char_count_map))
        }).sum::<u32>()); // part2: 2577
}

fn char3x(char_count_map: HashMap<char, u32>) -> char {
    for (k, v) in char_count_map.into_iter() {
        if v == 3 {
            return k;
        }
    }
    '0'
}

fn count_chars(char_map: &mut HashMap<char, u32>, item: &String) {
    item.chars().for_each(|c| {
        if char_map.contains_key(&c) {
            char_map.insert(c, char_map[&c] + 1);
        } else {
            char_map.insert(c, 1);
        }
    });
}

fn distinct_chars_only(item: &String) -> String {
    let mut char_set: HashSet<char> = HashSet::new();
    item.chars().for_each(|c| {
        char_set.insert(c);
    });
    let mut unique_string: String = String::new();
    char_set.iter().for_each(|c| { unique_string.push(*c); });
    let mut sorted_char_vec = unique_string.chars().collect::<Vec<char>>();
    sorted_char_vec.sort_by(|a, b| b.cmp(a));
    String::from_iter(sorted_char_vec)
}

fn find_common_char_in_each_half(items: &String) -> char {
    let (left, right) = items.split_at((items.len() / 2) as usize);
    for i in distinct_chars_only(&left.to_string()).chars() {
        for j in distinct_chars_only(&right.to_string()).chars() {
            if i == j {
                return i;
            }
        }
    }
    '0'
}

fn char_priority(c: char) -> u32 {
    let ord: u32 = c as u32;
    // a - z => 1 .. 26
    if 97 <= ord && ord <= 122 {
        return ord - 96;
    }
    // A - Z => 27 .. 58
    if 65 <= ord && ord <= 90 {
        return ord - 38;
    }
    return ord;
}