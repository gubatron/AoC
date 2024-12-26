use std::collections::HashSet;
use std::io::Write;

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

fn get_robot(board: &Vec<String>) -> (usize, usize) {
    let mut robot = (0, 0);
    for (y, row) in board.iter().enumerate() {
        if let Some(x) = row.find('@') {
            robot = (x, y);
            break;
        }
    }
    robot
}

fn get_ahead(pos: (usize, usize), direction: char) -> (usize, usize) {
    match direction {
        '^' => (pos.0, pos.1 - 1),
        'v' => (pos.0, pos.1 + 1),
        '<' => (pos.0 - 1, pos.1),
        '>' => (pos.0 + 1, pos.1),
        _ => pos,
    }
}

fn push_box(
    direction: char,
    box_pos: (usize, usize),
    boxes: &mut HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) -> (bool, HashSet<(usize, usize)>) {
    let ahead = get_ahead(box_pos, direction);
    if walls.contains(&ahead) || boxes.contains(&ahead) {
        return (false, boxes.clone());
    }
    boxes.remove(&box_pos);
    boxes.insert(ahead);
    (true, boxes.clone())
}

fn find_consecutive_boxes(
    direction: char,
    first_box_pos: (usize, usize),
    boxes: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut consecutive_boxes = vec![first_box_pos];
    let mut ahead = get_ahead(first_box_pos, direction);
    while boxes.contains(&ahead) {
        consecutive_boxes.push(ahead);
        ahead = get_ahead(ahead, direction);
    }
    consecutive_boxes
}

fn push_boxes(
    direction: char,
    first_box_pos: (usize, usize),
    boxes: &mut HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) -> (bool, HashSet<(usize, usize)>) {
    let mut moved = false;
    let mut consecutive_boxes = find_consecutive_boxes(direction, first_box_pos, boxes);
    // we start from the last box
    while let Some(box_pos) = consecutive_boxes.pop() {
        let (box_moved, new_boxes) = push_box(direction, box_pos, boxes, walls);
        moved = moved || box_moved;
        *boxes = new_boxes;
        //(moved, *boxes) = push_box(direction, box_pos, boxes, walls);
    }
    (moved, boxes.clone())
}

fn move_robot(
    direction: char,
    robot: (usize, usize),
    boxes: &mut HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) -> ((usize, usize), HashSet<(usize, usize)>) {
    let ahead = get_ahead(robot, direction);
    if walls.contains(&ahead) {
        return (robot, boxes.clone());
    }
    if boxes.contains(&ahead) {
        let (moved, moved_boxes) = push_boxes(direction, ahead, boxes, walls);
        if moved {
            return (ahead, moved_boxes);
        }
        return (robot, boxes.clone());
    }

    (ahead, boxes.clone())
}

fn print_board(robot:(usize, usize), boxes: HashSet<(usize, usize)>, walls: HashSet<(usize,usize)>, delay: f32) {
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

fn part1(board: &Vec<String>, directions: &str, print_delay:f32) -> usize {
    let mut robot = get_robot(board);
    let mut boxes = get_boxes(board);
    let walls = get_walls(board);

    for direction in directions.chars() {
        let (new_robot, new_boxes) = move_robot(direction, robot, &mut boxes, &walls);
        robot = new_robot;
        print_board(robot, new_boxes, walls.clone(), print_delay);
    }
    // move the cursor down by the height of the board
    for _ in 0..walls.iter().map(|(_, y)| y).max().unwrap() + 1 {
        println!();
    }

    // sum the gps of all the boxes
    boxes.iter().map(|(x, y)| gps(*x, *y)).sum()
}

fn main() {
    let (board, directions) = load_board_directions("inputs/15.txt", false);
    println!("Part 1: {:?}", part1(&board, &directions, 0.0)); //1514333
}
