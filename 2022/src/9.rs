fn main() {
    // Day 9: Rope Bridge
    let command_log = aoc_2022::utils::load_input_lines_as_vec_str("9.txt");
    let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part1(&moves);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn abs(x: i32) -> i32 {
    x.abs()
}

fn part1(moves: &Vec<(i32, i32)>) {
    let min_distance = 1;
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: std::collections::HashMap<Pos, usize> = std::collections::HashMap::new();
    visited.insert(head, 1);

    //moves.iter().for_each(move |(dx, dy)| {
    let mut i = 0;
    let moves_len = moves.len();
    while i < moves_len {
        let (dx, dy) = moves[i];

        println!("i={}: ({},{})", i, dx, dy);
        head.x = head.x + dx;
        head.y = head.y + dy;
        println!("HEAD -> ({},{})", head.x, head.y);
        // Start from same row, either direction
        //
        // [T][H]
        // [T] -> [H]
        if head.y == tail.y && abs(head.x - tail.x) > 1 {
            // we'll move the tail towards the new location of the head - 1
            // one by one we mark the tail positions as visited
            // move to the right
            let sign = (head.x - tail.x).signum();
            while abs(head.x - tail.x) > 1 {
                tail = Pos { x: tail.x + sign, y: tail.y };
                visited.insert(tail, 1);
                println!("TAIL -> ({},{})", tail.x, tail.y);
            }
        }
        // Start from same row, either direction
        // [T]  [T]
        // [H]  ⬇
        //      [H]
        else if head.x == tail.x && abs(head.y - tail.y) > min_distance {
            // we'll move the tail towards the new location of the head - 1
            // one by one we mark the tail positions as visited
            // move up
            let sign = (head.x - tail.x).signum();
            while abs(head.y - tail.y) > 1 {
                tail = Pos { x: tail.x, y: tail.y + sign };
                visited.insert(tail, 1);
                println!("TAIL -> ({},{})", tail.x, tail.y);
            }
        }
        // Diagonal diff
        else if head.x != tail.x && head.y != tail.y {
            // we'll move the tail towards the new location of the head - 1
            // one by one we mark the tail positions as visited
            // move up
            let sign_x = (head.x - tail.x).signum();
            let sign_y = (head.y - tail.y).signum();

            // changed rows
            // [T][H]       [T]
            //
            //                 [H]
            if abs(head.y - tail.y) > 1 {
                // follow diagonally to the same column
                tail = Pos { x: tail.x + sign_x, y: tail.y + sign_y };
                println!("TAIL (DIAG) -> ({},{})", tail.x, tail.y);
                visited.insert(tail, 1);

                // then follow along the same column, updating the row
                while abs(head.y - tail.y) > 1 {
                    tail = Pos { x: tail.x, y: tail.y + sign_y };
                    visited.insert(tail, 1);
                    println!("TAIL -> ({},{})", tail.x, tail.y);
                }
            }
            // changed columns
            // [T]    [T]
            // [H]          [H]
            //
            else if abs(head.x - tail.x) > 1 {
                // follow diagonally to the same row
                tail = Pos { x: tail.x + sign_x, y: tail.y + sign_y };
                println!("TAIL (DIAG) -> ({},{})", tail.x, tail.y);
                visited.insert(tail, 1);

                // then follow along the same row, updating the column
                while abs(head.x - tail.x) > 1 {
                    tail = Pos { x: tail.x + sign_x, y: tail.y };
                    visited.insert(tail, 1);
                    println!("TAIL -> ({},{})", tail.x, tail.y);
                }
            }
        }

        i += 1;
    };
    println!("Part 1: {}", count_positions_visited_once(&visited));
}

fn update_visited<'a>(tail: &Pos, visited: &mut std::collections::HashMap<Pos, usize>) {
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
        println!("({}, {}) => ", pos.x, pos.y);
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