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
    // part 1 tests
    assert_eq!(2, count_houses(&Vec::from([RIGHT])));
    assert_eq!(4, count_houses(&Vec::from([UP, RIGHT, DOWN, LEFT])));
    assert_eq!(2, count_houses(&Vec::from([UP, DOWN, UP, DOWN, UP, DOWN, UP])));

    // part 2 tests
    assert_eq!(3, count_houses_2(&Vec::from([UP, DOWN])));
    assert_eq!(3, count_houses_2(&Vec::from([UP, RIGHT, DOWN, LEFT])));
    assert_eq!(11, count_houses_2(&Vec::from([UP, DOWN, UP, DOWN, UP, DOWN, UP, DOWN, UP, DOWN])));
}

fn count_houses(directions: &Vec<char>) -> i32 {
    let mut visited: HashSet<House> = HashSet::new();
    let mut last = House { x: 0, y: 0 };
    visited.insert(last);

    directions.iter().for_each(|dir| {
        let mut x_delta = 0;
        let mut y_delta = 0;
        match *dir {
            RIGHT => x_delta = 1,
            LEFT => x_delta = -1,
            DOWN => y_delta = 1,
            UP => y_delta = -1,
            _ => {}
        }
        last = House { x: last.x + x_delta, y: last.y + y_delta };
        visited.insert(last);
    });

    visited.len() as i32
}

fn count_houses_2(directions: &Vec<char>) -> i32 {
    let mut visited: HashSet<House> = HashSet::new();
    let mut last_santa = House { x: 0, y: 0 };
    let mut last_robot = House { x: 0, y: 0 };
    visited.insert(last_santa);

    let mut i = 0;
    directions.iter().for_each(|dir| {
        let mut x_delta = 0;
        let mut y_delta = 0;
        match *dir {
            RIGHT => x_delta = 1,
            LEFT => x_delta = -1,
            DOWN => y_delta = 1,
            UP => y_delta = -1,
            _ => {}
        }

        if i % 2 == 0 {
            //santa
            last_santa = House { x: last_santa.x + x_delta, y: last_santa.y + y_delta };
            visited.insert(last_santa);
        } else {
            //robot
            last_robot = House { x: last_robot.x + x_delta, y: last_robot.y + y_delta };
            visited.insert(last_robot);
        }
        i += 1;
    });
    return visited.len() as i32;
}

fn part1(dirs: &Vec<char>) {
    println!("Part 1: {}", count_houses(dirs));
}

fn part2(dirs: &Vec<char>) {
    println!("Part 2: {}", count_houses_2(dirs));
}

fn main() {
    let dirs = &utils::load_input_as_vec_char("src/3.txt");
    part1(dirs);
    part2(dirs);
}
