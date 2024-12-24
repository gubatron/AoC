use aoc::utils::{euclidean_modulo, Coord};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Debug for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Robot: pos: {:?}, vel: {:?}, quadrant: {}\n",
            self.pos,
            self.vel,
            map_coord_to_quadrant(self.pos, 101, 103)
        )
    }
}

impl Robot {
    fn move_robot(&mut self, secs: usize, tides_wide: usize, tides_tall: usize) {
        self.pos.x += self.vel.x * secs as i32;
        self.pos.x = euclidean_modulo(self.pos.x, tides_wide as i32);
        self.pos.y += self.vel.y * secs as i32;
        self.pos.y = euclidean_modulo(self.pos.y, tides_tall as i32);
    }
}

fn main() {
    let input: Vec<String> = aoc::utils::load_input_lines_as_vec_str("inputs/14.txt");
    let robots: Vec<Robot> = input.iter().map(|line| parse_robot(line)).collect();

    // real tiles are 101x103
    // test tiles are 11x7
    //println!("Part 1: {} ", part1(robots.clone(), 11, 7, 100));
    println!("Part 1: {} ", part1(robots.clone(), 101, 103, 100));
    //println!("Part 2: {} ", part2(&robots));
}

fn parse_robot(input: &str) -> Robot {
    //p=0,4 v=3,-3
    let r = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let caps = r.captures(input).unwrap();
    let pos_x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let pos_y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let vel_x = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let vel_y = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    Robot {
        pos: Coord::new(pos_x, pos_y),
        vel: Coord::new(vel_x, vel_y),
    }
}

fn map_coord_to_quadrant(coord: Coord, tiles_wide: usize, tiles_tall: usize) -> usize {
    let middle_x: usize = tiles_wide / 2;
    let middle_y: usize = tiles_tall / 2;
    if (coord.x as usize) < middle_x && (coord.y as usize) < middle_y {
        1
    } else if (coord.x as usize) > middle_x && (coord.y as usize) < middle_y {
        2
    } else if (coord.x as usize) < middle_x && (coord.y as usize) > middle_y {
        3
    } else if (coord.x as usize) > middle_x && (coord.y as usize) > middle_y {
        4
    } else {
        0
    }
}

fn part1(robots: Vec<Robot>, tiles_wide: usize, tiles_tall: usize, secs: usize) -> i32 {
    let mut quadrant_bots: [usize; 5] = [0, 0, 0, 0, 0];
    let mut mut_robots = robots.clone();
    for mut robot in mut_robots.iter_mut() {
        robot.move_robot(secs, tiles_wide, tiles_tall);
        let quadrant = map_coord_to_quadrant(robot.pos, tiles_wide, tiles_tall);
        quadrant_bots[quadrant] += 1;
    }
    safety_factor(quadrant_bots)
}

fn safety_factor(quadrant_bots: [usize; 5]) -> i32 {
    println!("{:?}", quadrant_bots);
    let mut result = 0;
    for i in 1..5 {
        if quadrant_bots[i] > 0 {
            if result == 0 {
                result = quadrant_bots[i];
            } else {
                result *= quadrant_bots[i];
            }
        }
    }
    result as i32
}

fn part2(robot: &Vec<Robot>) -> i32 {
    0
}
