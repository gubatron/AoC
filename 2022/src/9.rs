fn main() {
    // Day 9: Rope Bridge
    let command_log = aoc::utils::load_input_lines_as_vec_str("inputs/9.txt");
    let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part1(&moves); // 6011

    //let command_log = aoc::utils::load_input_lines_as_vec_str("9.test.2.txt");
    //let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part2(&moves); // 2419
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
    // Part 1: 6002, (Test 13
    println!("Part 1: {}", count_unique_tail_knot_positions(moves, 2));
}

fn part2(moves: &Vec<(i32, i32)>) {
    // Part 2: ???, (Test 36)
    println!("Part 2: {}", count_unique_tail_knot_positions(moves, 10));
}

fn count_unique_tail_knot_positions(moves: &Vec<(i32, i32)>, total_knots: usize) -> usize {
    let mut rope = vec![Pos { x: 0, y: 0 }; total_knots];
    let mut visited = std::collections::HashSet::<Pos>::new();
    visited.insert(rope[0]);
    let mut i = 0;

    let moves_len = moves.len();
    while i < moves_len {
        let (dx, dy) = moves[i];
        // one of them is always 0
        let mut deltas = abs(dx) + abs(dy);
        while deltas > 0 {
            deltas -= 1;

            // lead with the first node in the rope [0]
            let head = &mut rope[0];
            head.x += dx.signum();
            head.y += dy.signum();

            // now we have more than one tail knot
            for i in 1..total_knots {
                let curr_head = rope[i - 1];
                let curr_tail = rope[i];
                let y_head = curr_head.y;
                let x_head = curr_head.x;

                if abs(y_head - curr_tail.y) == 2 || abs(x_head - curr_tail.x) == 2 {
                    follow(rope[i - 1], &mut rope[i]);
                }
                visited.insert(*rope.last().unwrap());
            }
        }
        i += 1;
    };
    visited.len()
}

fn follow(head: Pos, tail: &mut Pos) {
    tail.x += (head.x - tail.x).signum(); //-1, 0, 1 depending value of the integer
    tail.y += (head.y - tail.y).signum();
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