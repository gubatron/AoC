use linked_hash_set::LinkedHashSet;
use log::{debug, log_enabled};
use std::collections::{HashSet, VecDeque};
use std::io::Write;

type Pos = (usize, usize);
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct BigBox {
    left: Pos,
    right: Pos,
}

impl BigBox {
    fn new(left: Pos, right: Pos) -> Self {
        // Ensure that left is always to the left of right
        if left.0 < right.0 {
            BigBox { left, right }
        } else {
            BigBox {
                left: right,
                right: left,
            }
        }
    }
}

fn load_board_directions(file_path: &str, enhanced: bool) -> (Vec<String>, String) {
    let input = aoc::utils::load_input_break_by_empty_lines_as_vec_str(file_path);
    let board = input[0]
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let directions = input[1].replace('\n', "");

    if enhanced {
        let mut enhanced_board: Vec<String> = vec![];
        let replacements =
            std::collections::HashMap::from([('#', "##"), ('O', "[]"), ('@', "@."), ('.', "..")]);

        for (i, row) in board.iter().enumerate() {
            if enhanced_board.len() <= i {
                enhanced_board.push(String::new());
            }

            for char in row.chars() {
                if let Some(replacements) = replacements.get(&char) {
                    enhanced_board[i].push_str(replacements);
                }
            }
        }
        return (enhanced_board, directions);
    }

    (board, directions)
}

fn get_items(board: &Vec<String>, item: char) -> HashSet<(usize, usize)> {
    let mut items = HashSet::new();
    for (y, row) in board.iter().enumerate() {
        // find the position of the item in the current row String
        let mut x = 0;
        while let Some(pos) = row[x..].find(item) {
            x += pos;
            items.insert((x, y));
            x += 1;
        }
    }
    items
}

fn get_walls(board: &Vec<String>) -> HashSet<(usize, usize)> {
    get_items(board, '#')
}

fn get_boxes(board: &Vec<String>) -> HashSet<(usize, usize)> {
    get_items(board, 'O')
}

fn gps(x: usize, y: usize) -> usize {
    y * 100 + x
}

fn get_robot(board: &Vec<String>) -> Pos {
    for (y, row) in board.iter().enumerate() {
        if let Some(x) = row.find('@') {
            return (x, y);
        }
    }
    (0, 0)
}

/**
* Returns the position ahead of the given position.
* No logic about borders, or boxes, just the position ahead.
*/
fn get_ahead(pos: Pos, direction: char) -> Option<Pos> {
    match direction {
        '^' => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        'v' => Some((pos.0, pos.1 + 1)),
        '<' => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        '>' => Some((pos.0 + 1, pos.1)),
        _ => Some(pos),
    }
}

fn get_big_boxes_ahead(big_box: &BigBox, direction: char, boxes: &HashSet<BigBox>) -> Vec<BigBox> {
    let mut result = Vec::new();

    // Coordinates for the current big box
    let (left_x, left_y) = big_box.left;
    let (right_x, right_y) = big_box.right;

    match direction {
        '<' => {
            // if left_x == 0 {
            //     // Cannot move left if already at the leftmost position
            //     return result;
            // }
            let target_pos = (left_x - 1, left_y);
            // Check if any big box has its right part at target_pos
            for box_candidate in boxes {
                if box_candidate.right == target_pos {
                    result.push(box_candidate.clone());
                }
            }
        }
        '>' => {
            let target_pos = (right_x + 1, right_y);
            // Check if any big box has its left part at target_pos
            for box_candidate in boxes {
                if box_candidate.left == target_pos {
                    result.push(box_candidate.clone());
                }
            }
        }
        '^' => {
            if left_y == 0 {
                // Cannot move up if already at the top
                return result;
            }
            // Positions to check: directly above left and directly above right
            let possible_big_box_above = BigBox::new((left_x, left_y - 1), (right_x, right_y - 1));
            let possible_big_box_left_diagonal =
                BigBox::new((left_x - 1, left_y - 1), (right_x - 1, right_y - 1));
            let possible_big_box_right_diagonal =
                BigBox::new((left_x + 1, left_y - 1), (right_x + 1, right_y - 1));
            if boxes.contains(&possible_big_box_above) {
                result.push(possible_big_box_above);
            }
            if boxes.contains(&possible_big_box_left_diagonal) {
                result.push(possible_big_box_left_diagonal);
            }
            if boxes.contains(&possible_big_box_right_diagonal) {
                result.push(possible_big_box_right_diagonal);
            }
        }
        'v' => {
            // Positions to check: directly below left and directly below right
            let possible_big_box_below = BigBox::new((left_x, left_y + 1), (right_x, right_y + 1));
            let possible_big_box_left_diagonal =
                BigBox::new((left_x - 1, left_y + 1), (right_x - 1, right_y + 1));
            let possible_big_box_right_diagonal =
                BigBox::new((left_x + 1, left_y + 1), (right_x + 1, right_y + 1));
            if boxes.contains(&possible_big_box_below) {
                result.push(possible_big_box_below);
            }
            if boxes.contains(&possible_big_box_left_diagonal) {
                result.push(possible_big_box_left_diagonal);
            }
            if boxes.contains(&possible_big_box_right_diagonal) {
                result.push(possible_big_box_right_diagonal);
            }
        }
        _ => {
            // Invalid direction; do nothing
        }
    }

    result
}

fn push_box(direction: char, box_pos: Pos, boxes: &mut HashSet<Pos>, walls: &HashSet<Pos>) -> bool {
    // Attempt to get the position ahead; handle cases where it's out of bounds
    let ahead_opt = get_ahead(box_pos, direction);

    // If moving ahead is out of bounds, the push is invalid
    let ahead = match ahead_opt {
        Some(pos) => pos,
        None => return false,
    };

    // Check if the position ahead is blocked by a wall or another box
    if walls.contains(&ahead) || boxes.contains(&ahead) {
        return false;
    }

    // Perform the push: remove the current box position and insert the new position
    boxes.remove(&box_pos);
    boxes.insert(ahead);

    debug!(
        "Pushed box from {:?} to {:?} in direction '{}'",
        box_pos, ahead, direction
    );

    true
}

fn find_consecutive_boxes(direction: char, first_box_pos: Pos, boxes: &HashSet<Pos>) -> Vec<Pos> {
    let mut consecutive_boxes = vec![first_box_pos];
    let mut current_pos = first_box_pos;

    loop {
        // Attempt to get the position ahead; handle out-of-bounds moves
        let ahead_opt = get_ahead(current_pos, direction);

        // If moving ahead is out of bounds, stop searching
        let ahead = match ahead_opt {
            Some(pos) => pos,
            None => break,
        };

        // If the position ahead contains a box, add it to the list
        if boxes.contains(&ahead) {
            consecutive_boxes.push(ahead);
            current_pos = ahead; // Move to the next box in the sequence
        } else {
            break; // No more consecutive boxes in this direction
        }
    }

    consecutive_boxes
}

fn push_boxes(
    direction: char,
    first_box_pos: Pos,
    boxes: &mut HashSet<Pos>,
    walls: &HashSet<Pos>,
) -> bool {
    let mut moved = false;
    let mut boxes_to_push = find_consecutive_boxes(direction, first_box_pos, boxes);

    while let Some(box_pos) = boxes_to_push.pop() {
        let box_moved = push_box(direction, box_pos, boxes, walls);
        moved = moved || box_moved;
    }

    moved
}

fn move_robot(direction: char, robot: Pos, boxes: &mut HashSet<Pos>, walls: &HashSet<Pos>) -> Pos {
    // Attempt to get the position ahead; handle out-of-bounds
    let ahead_opt = get_ahead(robot, direction);

    // Match on the Option<Pos> to handle None (out-of-bounds) safely
    let ahead = match ahead_opt {
        Some(pos) => pos,
        None => {
            // Cannot move out of bounds; stay in place
            return robot;
        }
    };

    // If there's a wall in the position ahead, do not move
    if walls.contains(&ahead) {
        return robot;
    }

    // If there's a box in the position ahead, attempt to push it
    if boxes.contains(&ahead) {
        let moved = push_boxes(direction, ahead, boxes, walls);
        if moved {
            // Successfully pushed the box; update robot's position
            return ahead;
        }
        // Box could not be moved; robot stays in place
        return robot;
    }

    // No obstacles; move the robot to the position ahead
    ahead
}

fn print_board(robot: Pos, boxes: &HashSet<Pos>, walls: HashSet<Pos>, delay: f32) {
    // get the width and height of the board by looking at the max x and y values of the walls
    let width = walls.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = walls.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut board = vec![vec!['.'; width]; height];
    board[robot.1][robot.0] = '@';
    for (x, y) in boxes.iter() {
        board[*y][*x] = 'O';
    }
    for (x, y) in walls.iter() {
        board[*y][*x] = '#';
    }
    for row in board.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    move_console_cursor_up_by(height);
    if delay > 0.0 {
        std::thread::sleep(std::time::Duration::from_secs_f32(delay));
    }
}

fn move_console_cursor_up_by(times: usize) {
    for _ in 0..times {
        // print the escape sequence to move the cursor up by one line
        print!("\x1B[1A");
    }
    //flush stdout
    std::io::stdout().flush().unwrap();
}

fn part1(board: &Vec<String>, directions: &str, print_delay: f32) -> usize {
    let mut robot = get_robot(board);
    let mut boxes = get_boxes(board);
    let walls = get_walls(board);

    for direction in directions.chars() {
        let new_robot = move_robot(direction, robot, &mut boxes, &walls);
        robot = new_robot;
        print_board(robot, &boxes, walls.clone(), print_delay);
    }
    // move the cursor down by the height of the board
    for _ in 0..walls.iter().map(|(_, y)| y).max().unwrap() + 1 {
        println!();
    }

    // sum the gps of all the boxes
    boxes.iter().map(|(x, y)| gps(*x, *y)).sum()
}

fn print_big_board(
    robot: Pos,
    boxes: HashSet<BigBox>,
    walls: HashSet<Pos>,
    delay: f32,
    move_cursor_up: bool,
) {
    // get the width and height of the board by looking at the max x and y values of the walls
    let width = walls.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = walls.iter().map(|(_, y)| y).max().unwrap() + 1;
    let mut board = vec![vec!['.'; width]; height];
    board[robot.1][robot.0] = '@';

    for bb in boxes.iter() {
        let ((x1, y1), (x2, y2)) = (bb.left, bb.right);
        board[y1][x1] = '[';
        board[y2][x2] = ']';
    }

    for (x, y) in walls.iter() {
        board[*y][*x] = '#';
    }
    for row in board.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    if move_cursor_up {
        move_console_cursor_up_by(height);
    }
    if delay > 0.0001 {
        std::thread::sleep(std::time::Duration::from_secs_f32(delay));
    }
}

fn get_big_boxes(board: &Vec<String>) -> HashSet<BigBox> {
    let left_halves = get_items(&board, '[');
    left_halves
        .iter()
        .map(|(x, y)| {
            let right_half = (*x + 1, *y);
            BigBox::new((*x, *y), right_half)
        })
        .collect()
}

// BFS
fn find_consecutive_big_boxes(
    direction: char,
    first_big_box: BigBox,
    boxes: &HashSet<BigBox>,
) -> LinkedHashSet<BigBox> {
    let mut consecutive_boxes = LinkedHashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(first_big_box.clone());
    visited.insert(first_big_box.clone());

    debug!(
        "find_consecutive_boxes: Starting BFS to find consecutive boxes in direction '{}'",
        direction
    );

    while let Some(current_box) = queue.pop_front() {
        debug!(
            "find_consecutive_boxes: Visiting big box: {:?}",
            current_box
        );

        consecutive_boxes.insert(current_box.clone());

        let boxes_ahead = get_big_boxes_ahead(&current_box, direction, boxes);

        for box_ahead in boxes_ahead {
            if !visited.contains(&box_ahead) {
                debug!(
                    "find_consecutive_boxes: Found big box ahead: {:?}",
                    box_ahead
                );

                visited.insert(box_ahead.clone());
                queue.push_back(box_ahead.clone());
            }
        }
    }

    debug!(
        "find_consecutive_boxes: Consecutive big boxes found: {:?}",
        consecutive_boxes
    );

    consecutive_boxes
}

fn push_big_box(
    direction: char,
    big_box: BigBox,
    boxes: &mut HashSet<BigBox>,
    walls: &HashSet<Pos>,
) -> (bool, HashSet<BigBox>) {
    match direction {
        '<' => {
            // Attempt to move the box left
            let ahead = match get_ahead(big_box.left, direction) {
                Some(pos) => pos,
                None => return (false, boxes.clone()), // Move invalid if out of bounds
            };

            // Prevent moving left beyond the grid
            if ahead.0 >= big_box.left.0 {
                return (false, boxes.clone());
            }

            // Check for walls at the new position
            if walls.contains(&ahead) {
                return (false, boxes.clone());
            }

            // Define the new big box positions
            let new_big_box = BigBox::new(ahead, (ahead.0 + 1, ahead.1));

            if boxes.contains(&new_big_box) {
                return (false, boxes.clone());
            }

            // Move the box
            boxes.remove(&big_box);
            boxes.insert(new_big_box.clone());
            debug!(
                "Moved big box from {:?} to {:?} in direction '<'",
                big_box, new_big_box
            );

            (true, boxes.clone())
        }
        '>' => {
            // Attempt to move the box right
            let ahead = match get_ahead(big_box.right, direction) {
                Some(pos) => pos,
                None => return (false, boxes.clone()), // Move invalid if out of bounds
            };

            // Check for walls at the new position
            if walls.contains(&ahead) {
                return (false, boxes.clone());
            }

            // Define the new big box positions
            let new_big_box = BigBox::new((ahead.0 - 1, ahead.1), ahead);

            if boxes.contains(&new_big_box) {
                return (false, boxes.clone());
            }

            // Move the box
            boxes.remove(&big_box);
            boxes.insert(new_big_box.clone());

            debug!(
                "Moved big box from {:?} to {:?} in direction '>'",
                big_box, new_big_box
            );
            (true, boxes.clone())
        }
        '^' | 'v' => {
            // Attempt to move the box vertically
            let new_left_half = match get_ahead(big_box.left, direction) {
                Some(pos) => pos,
                None => return (false, boxes.clone()), // Move invalid if out of bounds
            };
            let new_right_half = (new_left_half.0 + 1, new_left_half.1);

            let new_big_box_above = BigBox::new(new_left_half, new_right_half);

            // Check for walls at the new positions
            if walls.contains(&new_left_half) || walls.contains(&new_right_half) {
                return (false, boxes.clone());
            }

            if boxes.contains(&new_big_box_above) {
                return (false, boxes.clone());
            }

            // Move the box
            boxes.remove(&big_box);
            boxes.insert(new_big_box_above.clone());
            debug!(
                "Moved big box from {:?} to {:?} in direction '{}'",
                big_box, new_big_box_above, direction
            );
            (true, boxes.clone())
        }
        _ => {
            // Invalid direction
            (false, boxes.clone())
        }
    }
}

fn wall_ahead_of_big_box(direction: char, big_box: BigBox, walls: &HashSet<Pos>) -> bool {
    if direction == '<' {
        let ahead = match get_ahead(big_box.left, direction) {
            Some(pos) => pos,
            None => return true, // Treat out-of-bounds as wall
        };
        return walls.contains(&ahead);
    }

    if direction == '>' {
        let ahead = match get_ahead(big_box.right, direction) {
            Some(pos) => pos,
            None => return true, // Treat out-of-bounds as wall
        };
        return walls.contains(&ahead);
    }

    if direction == '^' || direction == 'v' {
        let ahead_of_left_half = match get_ahead(big_box.left, direction) {
            Some(pos) => pos,
            None => return true, // Treat out-of-bounds as wall
        };
        let ahead_of_right_half = match get_ahead(big_box.right, direction) {
            Some(pos) => pos,
            None => return true, // Treat out-of-bounds as wall
        };
        return walls.contains(&ahead_of_left_half) || walls.contains(&ahead_of_right_half);
    }
    false
}

fn all_consecutive_big_boxes_clear_to_move(
    direction: char,
    consecutive_big_boxes: LinkedHashSet<BigBox>,
    walls: &HashSet<Pos>,
) -> bool {
    consecutive_big_boxes
        .iter()
        .all(|big_box| !wall_ahead_of_big_box(direction, big_box.clone(), walls))
}

fn push_big_boxes(
    direction: char,
    first_big_box: BigBox,
    boxes: &mut HashSet<BigBox>,
    walls: &HashSet<Pos>,
) -> (bool, HashSet<BigBox>) {
    let mut moved = false;
    let consecutive_boxes = find_consecutive_big_boxes(direction, first_big_box, boxes);

    // Check if all consecutive big boxes can be moved
    if !all_consecutive_big_boxes_clear_to_move(direction, consecutive_boxes.clone(), walls) {
        debug!("Cannot push all boxes in direction '{}'", direction);
        return (false, boxes.clone());
    }

    // Convert LinkedHashSet to Vec for ordered processing
    let mut boxes_to_push: Vec<BigBox> = consecutive_boxes.into_iter().collect();

    // // Sort boxes based on direction to push from farthest to nearest
    boxes_to_push.sort_by(|a, b| {
        match direction {
            '<' => b.left.0.cmp(&a.left.0), // Push leftmost first
            '>' => a.left.0.cmp(&b.left.0), // Push rightmost first
            '^' => b.left.1.cmp(&a.left.1), // Push topmost first
            'v' => a.left.1.cmp(&b.left.1), // Push farthest down first
            _ => std::cmp::Ordering::Equal,
        }
    });

    // Debug: Print sorted boxes to push
    debug!(
        "Boxes to push in direction '{}': {:?}",
        direction, boxes_to_push
    );

    while !boxes_to_push.is_empty() {
        let box_pos = boxes_to_push.pop().unwrap();
        let (box_moved, new_boxes) = push_big_box(direction, box_pos, boxes, walls);
        moved = moved || box_moved;
        *boxes = new_boxes;
    }

    (moved, boxes.clone())
}

fn move_robot_bb(
    direction: char,
    robot: (usize, usize),
    boxes: &mut HashSet<BigBox>,
    walls: &HashSet<(usize, usize)>,
) -> ((usize, usize), HashSet<BigBox>) {
    let ahead = match get_ahead(robot, direction) {
        Some(pos) => pos,
        None => return (robot, boxes.clone()), // Move invalid if out of bounds
    };

    if walls.contains(&ahead) {
        return (robot, boxes.clone());
    }

    let mut first_box: Option<BigBox> = None;

    // Identify the big box ahead based on direction
    if direction == '^' || direction == 'v' {
        // Define potential big box positions ahead
        let potential_boxes = match direction {
            '^' => vec![
                BigBox::new((robot.0 - 1, robot.1 - 1), (robot.0, robot.1 - 1)),
                BigBox::new((robot.0, robot.1 - 1), (robot.0 + 1, robot.1 - 1)),
            ],
            'v' => vec![
                BigBox::new((robot.0 - 1, robot.1 + 1), (robot.0, robot.1 + 1)),
                BigBox::new((robot.0, robot.1 + 1), (robot.0 + 1, robot.1 + 1)),
            ],
            _ => vec![],
        };

        // Find the first big box ahead
        for potential_box in potential_boxes {
            if boxes.contains(&potential_box) {
                first_box = Some(potential_box);
                break;
            }
        }

        // If no big box is ahead, move the robot
        if first_box.is_none() {
            return (ahead, boxes.clone());
        }
    }

    if direction == '<' {
        let ahead_right = match get_ahead(robot, direction) {
            Some(pos) => pos,
            None => return (robot, boxes.clone()), // Move invalid if out of bounds
        };
        let ahead_left = (ahead_right.0 - 1, ahead_right.1);
        first_box = Some(BigBox::new(ahead_left, ahead_right));
    } else if direction == '>' {
        let ahead_left = match get_ahead(robot, direction) {
            Some(pos) => pos,
            None => return (robot, boxes.clone()), // Move invalid if out of bounds
        };
        let ahead_right = (ahead_left.0 + 1, ahead_left.1);
        first_box = Some(BigBox::new(ahead_left, ahead_right));
    }

    if let Some(box_to_push) = first_box {
        if boxes.contains(&box_to_push) {
            let (moved, moved_boxes) = push_big_boxes(direction, box_to_push, boxes, walls);
            if moved {
                return (ahead, moved_boxes);
            }
            return (robot, boxes.clone());
        }
    }

    (ahead, boxes.clone())
}

fn part2(board: &Vec<String>, directions: &String, print_delay: f32) -> usize {
    let mut robot = get_robot(board);
    let walls = get_walls(board);
    let mut boxes = get_big_boxes(board);

    if log_enabled!(log::Level::Info) {
        print_big_board(robot, boxes.clone(), walls.clone(), print_delay, false);
    }

    for direction in directions.chars() {
        let (new_robot, new_boxes) = move_robot_bb(direction, robot, &mut boxes, &walls);
        robot = new_robot;
        if log_enabled!(log::Level::Info) {
            debug!("Move {:?}", direction);
            print_big_board(robot, new_boxes, walls.clone(), print_delay, false);
            println!();
        } else {
            print_big_board(robot, new_boxes, walls.clone(), print_delay, true);
        }
    }
    // move the cursor down by the height of the board
    if log_enabled!(log::Level::Info) {
        for _ in 0..walls.iter().map(|(_, y)| y).max().unwrap() + 1 {
            println!();
        }
    }

    // sum the gps of all the boxes
    boxes
        .iter()
        .map(|big_box| gps(big_box.left.0, big_box.left.1))
        .sum()
}

fn main() {
    env_logger::init();
    let (board, directions) = load_board_directions("inputs/15.txt", false);
    println!("Part 1: {:?}", part1(&board, &directions, 0f32)); //1514333

    let (board, directions) = load_board_directions("inputs/15.txt", true);
    println!("Part 2: {:?}", part2(&board, &directions, 0.0f32)); // 1528453

    // tests

    // let (board, directions) = load_board_directions("inputs/15.side.test.txt", false);
    // println!("Part 2: {:?}", part2(&board, &directions, 0.205f32)); //

    // let (board, directions) = load_board_directions("inputs/15.up.test.txt", false);
    // println!("Part 2: {:?}", part2(&board, &directions, 0.50f32)); //

    // let (board, directions) = load_board_directions("inputs/15.down.test.txt", false);
    // println!("Part 2: {:?}", part2(&board, &directions, 0.50f32)); //

    // let (board, directions) = load_board_directions("inputs/15.2.test.txt", true);
    // println!("Part 2: {:?}", part2(&board, &directions, 0.02f32)); //

    //crate::test::test_large_example_part_two();
}