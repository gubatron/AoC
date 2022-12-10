fn main() {
    // Day 9: ROpe Bridge
    let command_log = aoc_2022::utils::load_input_lines_as_vec_str("9.test.txt");
    let moves = command_log.iter().map(|s| parse_command(s)).collect();
    part1(&moves);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn part1(moves: &Vec<(i32, i32)>) {
    let start = Pos { x: 0, y: 0 };
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: std::collections::HashMap<&Pos, usize> = std::collections::HashMap::new();

    //moves.iter().for_each(move |(dx, dy)| {
    let mut i = 0;
    let moves_len = moves.len();
    while i < moves_len {
        let (dx, dy) = moves[i];
        // first move is always head
        if head == start {
            head.x = head.x + dx;
            head.y = head.y + dy;
        } else {
            // if head is in the same row as tail, and not right next to it, then we can move the tail
            if head.y == tail.y && (head.x - tail.x).abs() > 1 {
                // we'll move the tail towards the new location of the head - 1
                // one by one we mark the tail positions as visited
                // move to the right
                if head.x > tail.x {
                    while (head.x - tail.x).abs() > 1 {
                        let new_tail = Pos { x: tail.x + 1, y: tail.y };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                    }
                } else if head.x < tail.x {
                    while (head.x - tail.x).abs() > 1 {
                        let new_tail = Pos { x: tail.x - 1, y: tail.y };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                    }
                }
            }
            // head is in the same column as tail, and not right next to it, then we can move the tail
            else if head.x == tail.x && (head.y - tail.y).abs() > 1 {
                // we'll move the tail towards the new location of the head - 1
                // one by one we mark the tail positions as visited
                // move up
                if head.y > tail.y {
                    while (head.y - tail.y).abs() > 1 {
                        let new_tail = Pos { x: tail.x, y: tail.y + 1 };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                    }
                } else if head.y < tail.y {
                    while (head.y - tail.y).abs() > 1 {
                        let new_tail = Pos { x: tail.x, y: tail.y - 1 };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                    }
                }
            }
            // head ends up in different row(y) and column(x)
            else if head.x != tail.x && head.y != tail.y {
                // they were horizontally side by side, head moved up or down
                if (head.x - tail.x).abs() == 1 {
                    // head moved up more than one step
                    if head.y > tail.y && (head.y - tail.y).abs() > 1 {
                        //tail moves diagonally up and to the same .x of the head
                        let new_tail = Pos { x: head.x, y: tail.y + 1 };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                        // then tail moves up until it's one step away from the head
                        while (head.y - tail.y).abs() > 1 {
                            let new_tail = Pos { x: tail.x, y: tail.y + 1 };
                            tail = new_tail.clone();
                            update_visited(&tail, &mut visited)
                        }
                    } else if head.y < tail.y && (head.y - tail.y).abs() > 1 {
                        //tail moves diagonally down and to the same .x of the head
                        let new_tail = Pos { x: head.x, y: tail.y - 1 };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                        while (head.y - tail.y).abs() > 1 {
                            let new_tail = Pos { x: tail.x, y: tail.y - 1 };
                            tail = new_tail.clone();
                            update_visited(&tail, &mut visited)
                        }
                    }
                }
                // they were vertically side by side, head moved left or right
                else if (head.y - tail.y).abs() == 1 {
                    // head moved right more than one step
                    if head.x > tail.x && (head.x - tail.x).abs() > 1 {
                        //tail moves diagonally right and to the same .y of the head
                        let new_tail = Pos { x: tail.x + 1, y: head.y };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                        // then tail moves right until it's one step away from the head
                        while (head.x - tail.x).abs() > 1 {
                            let new_tail = Pos { x: tail.x + 1, y: tail.y };
                            tail = new_tail.clone();
                            update_visited(&tail, &mut visited);
                        }
                    } else if head.x < tail.x && (head.x - tail.x).abs() > 1 {
                        //tail moves diagonally left and to the same .y of the head
                        let new_tail = Pos { x: tail.x - 1, y: head.y };
                        tail = new_tail.clone();
                        update_visited(&tail, &mut visited);
                        while (head.x - tail.x).abs() > 1 {
                            let new_tail = Pos { x: tail.x - 1, y: tail.y };
                            tail = new_tail.clone();
                            update_visited(&tail, &mut visited);
                        }
                    }
                }
            }
        }
        i += 1;
    };
    println!("Part 1: {}", count_positions_visited_once(&visited));
}

fn update_visited<'a>(tail: &'a Pos, visited: &mut std::collections::HashMap<&'a Pos, usize>) {
    if visited.contains_key(tail) {
        let visits = visited.get(tail).unwrap() + 1;
        visited.insert(tail, visits);
    } else {
        visited.insert(tail, 1);
    }
}

fn count_positions_visited_once(visited: &std::collections::HashMap<&Pos, usize>) -> usize {
    visited.iter().filter(|(_, count)| **count == 1).count()
}


// returns a tuple with (delta x, delta y) to move the head of the rope
fn parse_command(command: &String) -> (i32, i32) {
    let mut command_parts = command.split(" ");
    let direction = command_parts.next().unwrap().to_string();
    let steps = command_parts.next().unwrap().parse::<i32>().unwrap();

    match direction.as_str() {
        "U" => (0, -steps),
        "R" => (steps, 0),
        "D" => (0, steps),
        "L" => (-steps, 0),
        _ => panic!("Unknown direction: {}", direction)
    }
}