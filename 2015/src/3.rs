use std::collections::HashSet;
use aoc_2015::utils;

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';

#[derive(Hash, Eq, Clone, Copy)]
struct House {
    x: i32,
    y: i32,
}

impl PartialEq<Self> for House {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn tests() {
    assert_eq!(2, count_houses(Vec::from([RIGHT])));
    assert_eq!(4, count_houses(Vec::from([UP, RIGHT, DOWN, LEFT])));
    assert_eq!(2, count_houses(Vec::from([UP, DOWN, UP, DOWN, UP, DOWN, UP])));
}

fn part1() {
    let dirs = utils::load_input_as_vec_char("src/3.txt");
    println!("Part 1: {}", count_houses(dirs));
}

fn count_houses(directions: Vec<char>) -> i32 {
    let mut visited: HashSet<House> = HashSet::new();
    let mut last = House { x: 0, y: 0 };
    visited.insert(last);

    directions.iter().for_each(|dir| {
        let mut x_delta = 0;
        let mut y_delta = 0;
        if *dir == RIGHT {
            x_delta = 1;
        } else if *dir == LEFT {
            x_delta = -1;
        } else if *dir == DOWN {
            y_delta = 1;
        } else if *dir == UP {
            y_delta = -1;
        }
        last = House { x:last.x + x_delta, y: last.y + y_delta};
        visited.insert(last);
    });

    visited.len() as i32
}

fn main() {
    part1();
}