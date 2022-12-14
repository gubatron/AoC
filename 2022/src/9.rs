fn main() {
    // Day 9: Rope Bridge
    let command_log = aoc_2022::utils::load_input_lines_as_vec_str("9.txt");
    let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part1(&moves); // not 4082 too low
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
    let mut visited = std::collections::HashSet::<Pos>::new();
    visited.insert(head);
    let mut i = 0;

    let moves_len = moves.len();
    while i < moves_len {
        let (dx, dy) = moves[i];
        // one of them is always 0
        let mut deltas = abs(dx) + abs(dy);
        while deltas > 0 {
            deltas -= 1;
            println!("head: ({},{}) deltas={}", head.x, head.y, deltas);
            println!("i={}: ({},{})", i, dx, dy);
            head.x += dx.signum();
            head.y += dy.signum();
            println!("head': ({},{}) deltas={}", head.x, head.y, deltas);
            println!();
            // Start from same row, either direction
            //
            // [T][H]
            // [T] -> [H]
            // Start from same row, either direction
            // [T]  [T]
            // [H]  ⬇
            //      [H]
            // changed columns
            // [T]    [T]
            // [H]          [H]
            //
            // follow diagonally to the same row
            // changed rows
            // [T][H]       [T]
            //
            //                 [H]
            // follow diagonally to the same column
            // In all cases, there's a 2 places difference between head and tail.
            if abs(head.y - tail.y) == 2 || abs(head.x - tail.x) == 2 {
                follow(&head, &mut tail);
                visited.insert(tail);
            }
        }
        i += 1;
        println!();
        //print_visited_in_ascii_grid(&head, &visited);
        println!("===================");
    };
    println!("Part 1: {}", visited.len());
}

fn follow(head: &Pos, tail: &mut Pos) {
    let sign_x = (head.x - tail.x).signum();
    let sign_y = (head.y - tail.y).signum();
    tail.x += sign_x;
    tail.y += sign_y;
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