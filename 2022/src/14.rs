use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::thread;
use aoc_2022::utils::Coord;
use crate::Element::{Air, Rock, Sand, Source};

fn main() {
    // Day 14: Regolith Reservoir
    let input = aoc_2022::utils::load_input_lines_as_vec_str("inputs/14.txt");

    let mut map: HashMap<Coord, Element> = HashMap::new();
    map.insert(Coord::new(500, 0), Source);

    for line in input {
        let coords = input_line_to_coords(line.as_str());
        draw_rock_segments_on_map(&mut map, coords);
        //thread::sleep(std::time::Duration::from_millis(1000))
    }

    draw_map(&map, true);
}

fn draw_map(map: &HashMap<Coord, Element>, clear_scren: bool) {
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

    if clear_scren {
        print! ("\x1B[2J\x1B[1;1H");
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
    for y in min_y..=max_y {
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
