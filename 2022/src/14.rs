use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::thread;
use aoc_2022::utils::Coord;
use crate::Element::{Air, Rock, Sand, Source};

fn main() {
    // Day 14: Regolith Reservoir
    let input = aoc_2022::utils::load_input_lines_as_vec_str("inputs/14.txt");
    let now = std::time::Instant::now();

    let mut map: HashMap<Coord, Element> = HashMap::new();
    map.insert(Coord::new(500, 0), Source);

    for line in input {
        let coords = input_line_to_coords(line.as_str());
        draw_rock_segments_on_map(&mut map, coords);
    }

    // Part 1
    // test:24, real: 728
    let sand_units_units_until_overflow = units_until_overflow(&mut map);
    draw_map(&map, false);
    println!("Part 1: {} sand units before sand overflow", sand_units_units_until_overflow);

    // Part 2
    // test: 93, real: 27623
    draw_rock_bed_on_map(&mut map);
    let sand_units_until_full = units_until_full(&mut map);
    draw_map(&map, false);
    println!("Part 2: {} sand units until full", sand_units_units_until_overflow + sand_units_until_full);

    println!("Time: {:?}", now.elapsed());
}

fn units_until_overflow(map: &mut HashMap<Coord, Element>) -> i32 {
    let mut sand_units = 0;
    let mut current_sand = Coord::new(500, 1);
    let (_, _, _, max_y) = get_map_bounds(&map);

    while current_sand.y <= max_y {
        let mut sand_settled = false;
        while !sand_settled {
            let down_coord = Coord::new(current_sand.x, current_sand.y + 1);
            let down_left_coord = Coord::new(current_sand.x - 1, current_sand.y + 1);
            let down_right_coord = Coord::new(current_sand.x + 1, current_sand.y + 1);

            let down_tile = *map.get(&down_coord).unwrap_or(&Air);
            let down_left_tile = *map.get(&down_left_coord).unwrap_or(&Air);
            let down_right_tile = *map.get(&down_right_coord).unwrap_or(&Air);

            if down_tile == Air {
                current_sand.y += 1;
            } else if down_left_tile == Air {
                current_sand.x -= 1;
                current_sand.y += 1;
            } else if down_right_tile == Air {
                current_sand.x += 1;
                current_sand.y += 1;
            } else {
                map.insert(current_sand, Sand);
                sand_units += 1;
                current_sand = Coord::new(500, 1);
                sand_settled = true;
                //draw_map(&map, true);
            }
            if current_sand.y > max_y {
                break;
            }
            //thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    sand_units
}

fn units_until_full(map: &mut HashMap<Coord, Element>) -> i32 {
    let mut sand_units = 0;
    let origin = Coord::new(500, 0);
    let mut current_sand = Coord::new(500, 0);

    while *map.get(&origin).unwrap() == Source {
        let mut sand_settled = false;
        while !sand_settled {
            let down_coord = Coord::new(current_sand.x, current_sand.y + 1);
            let down_left_coord = Coord::new(current_sand.x - 1, current_sand.y + 1);
            let down_right_coord = Coord::new(current_sand.x + 1, current_sand.y + 1);

            let down_tile = *map.get(&down_coord).unwrap_or(&Air);
            let down_left_tile = *map.get(&down_left_coord).unwrap_or(&Air);
            let down_right_tile = *map.get(&down_right_coord).unwrap_or(&Air);

            if down_tile == Air {
                current_sand.y += 1;
            } else if down_left_tile == Air {
                current_sand.x -= 1;
                current_sand.y += 1;
            } else if down_right_tile == Air {
                current_sand.x += 1;
                current_sand.y += 1;
            } else {
                map.insert(current_sand, Sand);
                sand_units += 1;
                current_sand = Coord::new(500, 0);
                sand_settled = true;
            }
        }
    }
    sand_units
}

fn draw_rock_bed_on_map(map: &mut HashMap<Coord, Element>) {
    let (min_x, max_x, _, max_y) = get_map_bounds(&map);
    let diff_twice = (max_x - min_x) * 4;
    for x in min_x - diff_twice..max_x + diff_twice {
        map.insert(Coord::new(x, max_y + 2), Rock);
    }
}

fn get_map_bounds(map: &HashMap<Coord, Element>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for coord in map.keys() {
        if coord.x < min_x {
            min_x = coord.x;
        }
        if coord.x > max_x {
            max_x = coord.x;
        }
        if coord.y < min_y {
            min_y = coord.y;
        }
        if coord.y > max_y {
            max_y = coord.y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

// returns (max_y, min_x, max_x)
fn draw_map(map: &HashMap<Coord, Element>, clear_screen: bool) {
    let (min_x, max_x, min_y, max_y) = get_map_bounds(&map);
    // header coordinate drawing
    let min_x_str = min_x.to_string();
    let origin_x_str = 500.to_string();
    let max_x_str = max_x.to_string();

    let n_spaces_origin = 500 - min_x - 1;
    let n_spaces_max = max_x - 500 - 1;

    let mut spaces_origin = String::new();
    for _ in 0..n_spaces_origin {
        spaces_origin.push(' ');
    }
    let mut spaces_max = String::new();
    for _ in 0..n_spaces_max {
        spaces_max.push(' ');
    }

    let mut initial_padding = String::from("   ");
    if max_y > 99 {
        initial_padding.push(' ');
    }

    if clear_screen {
        print!("\x1B[2J\x1B[1;1H");
    }

    // print coordinates in the header
    for i in 0..3 {
        let c_min = min_x_str.chars().nth(i).unwrap_or(' ');
        let c_origin = origin_x_str.chars().nth(i).unwrap_or(' ');
        let c_max = max_x_str.chars().nth(i).unwrap_or(' ');

        print!("{}", initial_padding);
        print!("{}", c_min);
        print!("{}", spaces_origin);
        print!("{}", c_origin);
        print!("{}", spaces_max);
        println!("{}", c_max);
    }


    // print row number with 0 padding
    for y in min_y..max_y + 1 {
        if max_y >= 100 {
            if y < 10 {
                print!("00");
            } else if y < 100 {
                print!("0");
            }
        } else if max_y < 100 {
            if y < 10 {
                print!("0");
            }
        }
        print!("{} ", y);

        // print map elements
        for x in min_x..=max_x {
            let coord = Coord::new(x, y);
            let element = map.get(&coord).unwrap_or(&Air);
            print!("{}", element);
        }
        println!();
    }
}

fn draw_rock_segments_on_map(map: &mut HashMap<Coord, Element>, coords: Vec<Coord>) {
    for i in 1..coords.len() {
        let prev = coords[i - 1];
        let curr = coords[i];

        if prev.y == curr.y {
            let start = std::cmp::min(prev.x, curr.x);
            let end = std::cmp::max(prev.x, curr.x) + 1;
            for x in start..end {
                map.insert(Coord::new(x, prev.y), Rock);
            }
        } else {
            let start = std::cmp::min(prev.y, curr.y);
            let end = std::cmp::max(prev.y, curr.y) + 1;
            for y in start..end {
                map.insert(Coord::new(prev.x, y), Rock);
            }
        }
    }
}

fn input_line_to_coords(line: &str) -> Vec<Coord> {
    let coords_str: Vec<&str> = line.split(" -> ").collect();
    coords_str.iter().map(|s|
        {
            let mut split = s.split(",");
            Coord {
                x: split.next().unwrap().parse::<i32>().unwrap(),
                y: split.next().unwrap().parse::<i32>().unwrap(),
            }
        }
    ).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Element {
    Air,
    Rock,
    Source,
    Sand,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Air => write!(f, "."),
            Rock => write!(f, "#"),
            Source => write!(f, "+"),
            Sand => write!(f, "o"),
        }
    }
}
