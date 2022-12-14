use std::time::Instant;
use log::info;

// Day 11, Monkey in the middle
fn main() {
    env_logger::try_init().unwrap();

    let mut monkeys = load_monkeys("11.txt");

    let now = Instant::now();
    play_rounds(&mut monkeys, 20, true); // Monkey business level: 107822
    println!("Part 1: {} in {:?}", monkey_business_level(&monkeys), now.elapsed());

    let mut monkeys = load_monkeys("11.txt");
    let now = Instant::now();
    play_rounds(&mut monkeys, 10000, false); // Monkey business level: 27267163742
    println!("Part 2: {} in {:?}", monkey_business_level(&monkeys), now.elapsed());
}

fn play_rounds(monkeys: &mut Vec<Monkey>, rounds: usize, divide_by_3: bool) {
    let monkeys_len = monkeys.len();
    let gcd = gcd(monkeys);
    for round in 0..rounds {
        for i in 0..monkeys_len {
            info!("Monkey {}:", &monkeys[i].index);
            while monkeys[i].items.len() > 0 {
                monkeys[i].inspected_items += 1;
                let mut worry_level = monkeys[i].items.remove(0);
                info!("  Monkey inspects an item with a worry level of {}", worry_level);
                let mut right_operand = worry_level;
                if monkeys[i].right_operand != -1 {
                    right_operand = monkeys[i].right_operand as u128;
                }
                if monkeys[i].operation == Operation::Add {
                    worry_level += right_operand;
                    info!("    Worry level is increases by {} to {}", right_operand, worry_level);
                } else if monkeys[i].operation == Operation::Multiply {
                    worry_level *= right_operand;
                    info!("    Worry level is multiplied by {} to {}", right_operand, worry_level);
                }

                if divide_by_3 {
                    worry_level /= 3;
                    info!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
                } else {
                    worry_level = worry_level % gcd;
                    info!("    Monkey gets bored with item. Worry level is decreased by 1 to {}.", worry_level);
                }

                let mut target_monkey_index = monkeys[i].false_monkey_index as usize;
                if worry_level % monkeys[i].test_divisor as u128 == 0 {
                    info!("    Current worry level is divisible by {}.", monkeys[i].test_divisor);
                    target_monkey_index = monkeys[i].true_monkey_index as usize;
                } else {
                    info!("    Current worry level is not divisible by {}.", monkeys[i].test_divisor);
                }
                info!("    Item with worry level {} is thrown to monkey {}.", worry_level, target_monkey_index);
                monkeys[target_monkey_index].items.push(worry_level);
            }
        }
        info!("");
        //print_monkeys(monkeys, round + 1);
    }
    //print_monkey_activity(monkeys);
}

fn gcd(monkeys: &Vec<Monkey>) -> u128 {
    let mut gcd = 1;
    for monkey in monkeys {
        gcd *= monkey.test_divisor as u128;
    }
    gcd
}

fn monkey_business_level(monkeys: &Vec<Monkey>) -> u128 {
    let mut inspected_vector: Vec<u128> = monkeys.iter().map(|monkey| monkey.inspected_items).collect();
    inspected_vector.sort_by(|a, b| b.cmp(a));
    info!("MKBZ -> {:?}", inspected_vector);

    (inspected_vector[0] * inspected_vector[1]) as u128
}

fn print_monkeys(monkeys: &Vec<Monkey>, round: usize) {
    info!("After round {}, the monkeys are holding items with these worry levels:", round);
    for monkey in monkeys {
        info!("Monkey {}: {:?}", monkey.index, monkey.items);
    }
    info!("");
}

fn print_monkey_activity(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        info!("Monkey {} inspected items {} items.", monkey.index, monkey.inspected_items);
    }
    info!("Monkey business level: {}", monkey_business_level(monkeys));
}

fn load_monkeys(filename: &str) -> Vec<Monkey> {
    aoc_2022::utils::load_input_break_by_empty_lines_as_vec_str(filename)
        .iter()
        .map(|monkey_str| monkey_str.clone().into()).collect::<Vec<Monkey>>()
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Monkey {
    //not needed but why not.
    index: u32,

    items: Vec<u128>,

    // Add | Multiply
    operation: Operation,

    // if not a number and it's 'old' assign -1
    right_operand: i64,

    test_divisor: u128,

    // who to throw
    true_monkey_index: u32,
    false_monkey_index: u32,

    inspected_items: u128,
}

impl From<String> for Monkey {
    fn from(input: String) -> Self {
        let mut monkey = Monkey {
            index: 0,
            items: vec![],
            operation: Operation::Add,
            right_operand: -1,
            test_divisor: 0,
            true_monkey_index: 0,
            false_monkey_index: 0,
            inspected_items: 0,
        };
        let lines = input.split("\n");
        for line in lines {
            let mut line = line.trim().to_string();
            if line.starts_with("Monkey") {
                monkey.index = line.chars().collect::<Vec<char>>()[7].to_digit(10).unwrap();
            } else if line.starts_with("Starting") {
                monkey.items = aoc_2022::utils::convert_comma_separated_number_list_to_vec_t::<u128>(&line.to_string().split_off(16));
            } else if line.starts_with("Operation") {
                match line.contains("*") {
                    true => monkey.operation = Operation::Multiply,
                    false => monkey.operation = Operation::Add,
                }
                let right_operand_str = line.split_off(23);
                if right_operand_str != "old".to_string() {
                    monkey.right_operand = right_operand_str.parse::<i64>().unwrap();
                }
            } else if line.starts_with("Test") {
                monkey.test_divisor = line.split_off(19).parse::<u128>().unwrap();
            } else if line.starts_with("If true") {
                monkey.true_monkey_index = line.split_off(25).parse::<u32>().unwrap();
            } else if line.starts_with("If false") {
                monkey.false_monkey_index = line.split_off(26).parse::<u32>().unwrap();
            }
        }
        monkey
    }
}

#[test]
fn str_tests() {
    let mut s = "Starting items: 79, 98".to_string();
    let rest = &s.split_off(16);
    let items = aoc_2022::utils::convert_comma_separated_number_list_to_vec_T::<u32>(rest);
    assert_eq!(items, vec![79, 98]);
    info!("{:?}", items);

    let input = r###"Monkey 0:
        Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"###;
    let monkey = Monkey::from(input.to_string());
    info!("{:?}", monkey);
}