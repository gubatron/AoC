use std::cmp::Ordering;

fn main() {
    // Day 9: Rope Bridge
    let command_log = aoc_2022::utils::load_input_lines_as_vec_str("9.test.txt");
    let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part1(&moves);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }

}

fn abs(x: i32) -> i32 {
    x.abs()
}

fn part1(moves: &Vec<(i32, i32)>) {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: std::collections::HashMap<Pos, usize> = std::collections::HashMap::new();
    update_visited(&tail, &mut visited);

    let mut i = 0;
    let moves_len = moves.len();
    while i < moves_len {
        let (dx, dy) = moves[i];
        let mut deltas = abs(dx) + abs(dy);

        while deltas > 0 {
            println!("head: ({},{}) deltas={}", head.x, head.y, deltas);
            println!("i={}: ({},{})", i, dx, dy);
            head.x = head.x + dx.signum();
            head.y = head.y + dy.signum();
            deltas -= 1;
            println!("head': ({},{}) deltas={}", head.x, head.y, deltas);
            println!();
            // Start from same row, either direction
            //
            // [T][H]
            // [T] -> [H]
            if head.y == tail.y && abs(head.x - tail.x) == 2 {
                // we'll move the tail towards the new location of the head - 1
                // one by one we mark the tail positions as visited
                // move to the right
                let sign = (head.x - tail.x).signum();
                tail = Pos { x: tail.x + sign, y: tail.y };
                update_visited(&tail, &mut visited);
                //println!("TAIL.same_row -> ({},{}) HEAD -> ({},{}) ; deltas={}", tail.x, tail.y, head.x, head.y, deltas);
            }
            // Start from same row, either direction
            // [T]  [T]
            // [H]  ⬇
            //      [H]
            else if head.x == tail.x && abs(head.y - tail.y) == 2 {
                // we'll move the tail towards the new location of the head - 1
                // one by one we mark the tail positions as visited
                // move up
                let sign = (head.y - tail.y).signum();
                tail = Pos { x: tail.x, y: tail.y + sign };
                update_visited(&tail, &mut visited);
                //println!("TAIL.same_col -> ({},{}) HEAD -> ({},{}) ; deltas={}", tail.x, tail.y, head.x, head.y, deltas);
            }
            // Diagonal from horizontal config
            else if abs(head.x - tail.x) == 1 && abs(head.y - tail.y) == 2 {
                // changed rows
                // [T][H]       [T]
                //
                //                 [H]
                // follow diagonally to the same column
                let sign_x = (head.x - tail.x).signum();
                let sign_y = (head.y - tail.y).signum();
                tail = Pos { x: tail.x + sign_x, y: tail.y + sign_y };
                //println!("TAIL (DIAG-COL) -> ({},{}) signs ({},{}) deltas={}", tail.x, tail.y, sign_x, sign_y, deltas);
                update_visited(&tail, &mut visited);
            }
            // Diagonal from vertical config
            else if abs(head.y - tail.y) == 1 && abs(head.x - tail.x) == 2 {
                // changed columns
                // [T]    [T]
                // [H]          [H]
                //
                // follow diagonally to the same row
                let sign_x = (head.x - tail.x).signum();
                let sign_y = (head.y - tail.y).signum();
                tail = Pos { x: tail.x + sign_x, y: tail.y + sign_y };
                println!("TAIL (DIAG-ROW) -> ({},{}) deltas={}", tail.x, tail.y, deltas);
                update_visited(&tail, &mut visited);
            }
        }
        i += 1;
        println!();
        //print_visited_in_ascii_grid(&head, &visited);
        println!("===================");
    };
    print_visited_in_ascii_grid(&visited);
    println!("Part 1: {}", count_positions_visited_once(&visited));
}

/// Prints the visited positions in an ASCII grid that is centered around the (0,0) position
/// and has a size of 20 x 20
/// non visited positions appear as '.', the head as 'H' and the tail as 'T'
fn print_visited_in_ascii_grid(visited: &std::collections::HashMap<Pos, usize>) {
    let padding = 10;
    let min_x = visited.iter().enumerate().min_by_key(|(_, (pos, _))| pos.x).unwrap().1.0.x + padding;
    let max_x = visited.iter().enumerate().max_by_key(|(_, (pos, _))| pos.x).unwrap().1.0.x + padding;
    let min_y = visited.iter().enumerate().min_by_key(|(_, (pos, _))| pos.y).unwrap().1.0.y + padding;
    let max_y = visited.iter().enumerate().max_by_key(|(_, (pos, _))| pos.y).unwrap().1.0.y + padding;

    let mut grid = vec![vec!['.'; (max_x - min_x) as usize]; (max_y - min_y) as usize];
    let center_x = (max_x - min_x) / 2;
    let center_y = (max_y - min_y) / 2;

    for (pos, count) in visited {
        if count == &1 {
            grid[(pos.y + center_y) as usize][(pos.x + center_x) as usize] = '#';
        }
    }
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn update_visited<'a>(tail: &Pos, visited: &'a mut std::collections::HashMap<Pos, usize>) {
    let tail = tail.clone();
    if visited.contains_key(&tail) {
        let visits = visited.get(&tail).unwrap() + 1;
        visited.insert(tail, visits);
    } else {
        visited.insert(tail, 1);
    }
}

fn count_positions_visited_once(visited: &std::collections::HashMap<Pos, usize>) -> usize {
    let mut result = 0;
    for (pos, visits) in visited {
        println!("Counting: ({}, {}) => {}", pos.x, pos.y, visits);
        if *visits == 1 as usize {
            result += 1;
        }
    }
    result
}


// returns a tuple with (delta x, delta y) to move the head of the rope
fn parse_command(command: &String) -> (i32, i32) {
    let mut command_parts = command.split(" ");
    let direction = command_parts.next().unwrap().to_string();
    let steps = command_parts.next().unwrap().parse::<i32>().unwrap();

    match direction.as_str() {
        "U" => (0, -steps),
        "D" => (0, steps),
        "R" => (steps, 0),
        "L" => (-steps, 0),
        _ => panic!("Unknown direction: {}", direction)
    }
}

#[test]
fn test_pos() {
    let head = Pos { x: 0, y: 0 };
    let tail = Pos { x: 0, y: 0 };
    assert_eq!(head, tail);
    let a: i32 = 4;
    let b: i32 = 1;

    println!("a-b.signum() -> {:?}", (a - b).signum());
    println!("b-a.signum() -> {:?}", (b - a).signum());
    println!("0.signum() -> {:?}", 0_i32.signum());
}