use aoc::utils::{euclidean_modulo, Coord};
use image::{ImageBuffer, Rgb};
use minifb::{Window, WindowOptions};
use std::fmt::{Debug, Formatter};
use std::path::Path;


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

fn reset_map(map: &mut [[i32; 101]; 103]) {
    for i in 0..103 {
        for j in 0..101 {
            map[i][j] = 0;
        }
    }
}

fn project_bots_onto_map(robots: &Vec<Robot>, map: &mut [[i32; 101]; 103]) {
    reset_map(map);
    for robot in robots {
        map[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }
}

/// Converts the robot map into a pixel buffer.
/// Each robot is represented by a specific color.
/// Multiple robots in the same cell can be represented differently.
fn update_window_buffer(map: &[[i32; 101]; 103], buffer: &mut Vec<u32>, scale: u32) {
    // Define colors in ARGB format (0xAARRGGBB)
    let empty_color = 0xFF000000;        // Black
    let single_robot_color = 0xFFFFFFFF; // White
    let multiple_robot_color = 0xFFFF0000; // Red

    for y in 0..103 {
        for x in 0..101 {
            let robot_count = map[y][x];
            let color = if robot_count == 1 {
                single_robot_color
            } else if robot_count > 1 {
                multiple_robot_color
            } else {
                empty_color
            };

            for dy in 0..scale {
                for dx in 0..scale {
                    let pixel_x = x * scale as usize + dx as usize;
                    let pixel_y = y * scale as usize + dy as usize;
                    if pixel_x < 101 * scale as usize && pixel_y < 103 * scale as usize {
                        let index = pixel_y * (101 * scale as usize) + pixel_x;
                        buffer[index] = color;
                    }
                }
            }
        }
    }
}

/// Renders the robot map to a PNG image with the current time in the filename.
///
/// # Arguments
///
/// * `map` - A 2D array representing the robot positions.
/// * `time` - The current time in seconds.
/// * `scale` - The scaling factor to enlarge the image for better visibility.
///
/// # Example
///
/// ```rust
/// let map = [[0; 101]; 103];
/// render_map_to_png(&map, 7548, 5).unwrap();
/// ```
fn render_map_to_png(
    map: &[[i32; 101]; 103],
    time: usize,
    scale: u32,
    grid: bool
) -> Result<(), image::ImageError> {
    // Define colors
    let empty_color = Rgb([0u8, 0u8, 0u8]); // Black for empty cells
    let single_robot_color = Rgb([255u8, 255u8, 255u8]); // White for single robots
    let multiple_robot_color = Rgb([255u8, 0u8, 0u8]); // Red for multiple robots

    // Define image dimensions with scaling
    let img_width = 101 * scale;
    let img_height = 103 * scale;

    // Create a new image buffer with explicit pixel and container types
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(img_width, img_height, empty_color);

    // Iterate over each cell in the map
    for y in 0..103 {
        for x in 0..101 {
            let robot_count = map[y][x];
            if robot_count > 0 {
                // Determine color based on robot count
                let color = if robot_count == 1 {
                    single_robot_color
                } else {
                    multiple_robot_color
                };

                // Calculate the top-left pixel position for this cell
                let pixel_x = (x as u32) * scale;
                let pixel_y = (y as u32) * scale;

                // Draw a scaled block for better visibility
                for dy in 0..scale {
                    for dx in 0..scale {
                        if pixel_x + dx < img_width && pixel_y + dy < img_height {
                            img.put_pixel(pixel_x + dx, pixel_y + dy, color);
                        }
                    }
                }
            }

            // Optional: Draw grid lines (right and bottom edges of each cell)
            // Uncomment the following block if you want grid lines

            if grid {
                let grid_color = Rgb([50u8, 50u8, 50u8]); // Dark gray for grid lines
                if x < 101 - 1 {
                    for dy in 0..scale {
                        if (y as u32) * scale + dy < img_height {
                            img.put_pixel((x as u32 + 1) * scale, (y as u32) * scale + dy, grid_color);
                        }
                    }
                }
                if y < 103 - 1 {
                    for dx in 0..scale {
                        if (x as u32) * scale + dx < img_width {
                            img.put_pixel((x as u32) * scale + dx, (y as u32 + 1) * scale, grid_color);
                        }
                    }
                }
            }

        }
    }

    // Format the filename with leading zeros (e.g., robots_at_0007548.png)
    let filename = format!("outputs/robots_at_{:07}.bmp", time);

    // Save the image as PNG
    img.save(Path::new(&filename))?;
    Ok(())
}

// Given a map, find the first straight line of at least 10 robots in consecutive cells
// can be horizontal or vertical
fn find_straight_line_of_10(map: &[[i32; 101]; 103]) -> bool {
    // Check horizontal lines
    for y in 0..103 {
        let mut count = 0;
        for x in 0..101 {
            if map[y][x] > 0 {
                count += 1;
                if count >= 10 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    // Check vertical lines
    for x in 0..101 {
        let mut count = 0;
        for y in 0..103 {
            if map[y][x] > 0 {
                count += 1;
                if count >= 10 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }

    false
}

fn main() {
    let input: Vec<String> = aoc::utils::load_input_lines_as_vec_str("inputs/14.txt");
    let robots: Vec<Robot> = input.iter().map(|line| parse_robot(line)).collect();
    println!("Part 1: {} ", part1(robots.clone(), 101, 103, 100)); // Part 1: 230436441
    println!("Part 2: {} ", part2(&robots, 101, 103)); // Part 2: 8270
}

fn part1(robots: Vec<Robot>, tiles_wide: usize, tiles_tall: usize, secs: usize) -> i32 {
    let mut quadrant_bots: [usize; 5] = [0, 0, 0, 0, 0];
    let mut mut_robots = robots.clone();
    for robot in mut_robots.iter_mut() {
        robot.move_robot(secs, tiles_wide, tiles_tall);
        let quadrant = map_coord_to_quadrant(robot.pos, tiles_wide, tiles_tall);
        quadrant_bots[quadrant] += 1;
    }
    safety_factor(quadrant_bots)
}

fn part2(robots: &Vec<Robot>, tiles_wide: usize, tiles_tall: usize) -> usize {
    let mut mut_robots = robots.clone();
    let mut map: [[i32; 101]; 103] = [[0; 101]; 103];
    let mut time = 1;

    // Initialize window parameters
    let scale = 5; // Adjust for better visibility
    let window_width = (101 * scale) as usize;
    let window_height = (103 * scale) as usize;

    // Create a window
    let mut window = Window::new(
        "Robot Simulation",
        window_width,
        window_height,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });

    // Initialize the pixel buffer
    let mut buffer: Vec<u32> = vec![0; window_width * window_height];

    // Limit to ~60 FPS
    window.set_target_fps(60);

    loop {
        for robot in mut_robots.iter_mut() {
            robot.move_robot(1, tiles_wide, tiles_tall);
        }
        project_bots_onto_map(&mut_robots, &mut map);

        if find_straight_line_of_10(&map) {
            // Render the map to PNG (optional)
            let filename = format!("day_24_robots_at_{:07}.png", time);
            if let Err(e) = render_map_to_png(&map, time, scale, false) {
                eprintln!("Failed to render image at time {}: {}", time, e);
            } else {
                println!("Rendered image: {}", filename);
            }
            // Update the window buffer
            update_window_buffer(&map, &mut buffer, scale);
            window.update_with_buffer(&buffer, window_width, window_height).unwrap();
            break;
        }
        time += 1;
    }
    time
}
