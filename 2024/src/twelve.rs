use aoc::utils::Coord;
use std::collections::{HashMap, HashSet, VecDeque};

// Function to find contiguous areas in the map (bfs)
fn find_areas(grid: &[Vec<char>]) -> HashMap<char, Vec<Vec<Coord>>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut areas: HashMap<char, Vec<Vec<Coord>>> = HashMap::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..rows {
        for j in 0..cols {
            if visited[i][j] {
                continue;
            }
            let char_type = grid[i][j];
            let mut queue = VecDeque::new();
            let mut area = Vec::new();

            queue.push_back(Coord {
                x: i as i32,
                y: j as i32,
            });
            visited[i][j] = true;

            while let Some(Coord { x, y }) = queue.pop_front() {
                area.push(Coord { x, y });

                for &(dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0
                        && ny >= 0
                        && (nx as usize) < rows
                        && (ny as usize) < cols
                        && !visited[nx as usize][ny as usize]
                        && grid[nx as usize][ny as usize] == char_type
                    {
                        visited[nx as usize][ny as usize] = true;
                        queue.push_back(Coord {
                            x: nx as i32,
                            y: ny as i32,
                        });
                    }
                }
            }

            areas.entry(char_type).or_default().push(area);
        }
    }

    areas
}

// Function to calculate the perimeter of a contiguous area
fn calculate_perimeter(area: &[Coord], grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut perimeter = 0;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let cell_set: HashSet<_> = area.iter().cloned().collect();

    for &Coord { x, y } in area {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0
                || ny < 0
                || (nx as usize) >= rows
                || (ny as usize) >= cols
                || !cell_set.contains(&Coord {
                    x: nx as i32,
                    y: ny as i32,
                })
            {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_sides(area: &[Coord]) -> usize {
    let cell_set: HashSet<_> = area.iter().cloned().collect();
    area.iter().map(|&c| get_corners_for_coord(c, &cell_set)).sum()
}

fn get_corners_for_coord(coord: Coord, cell_set: &HashSet<Coord>) -> usize {
    let Coord { x, y } = coord;

    // Orthogonal neighbors
    let top = (x as isize - 1, y as isize);
    let right = (x as isize, y as isize + 1);
    let down = (x as isize + 1, y as isize);
    let left = (x as isize, y as isize - 1);

    // We'll define a small helper to check membership
    let in_area = |(r, c): (isize, isize)| {
        if r < 0 || c < 0 {
            return false;
        }
        cell_set.contains(&Coord { x: r as i32, y: c as i32 })
    };

    let mut count = 0;

    // 1) Four orthogonal corners
    // If top & right are out => corner
    if !in_area(top) && !in_area(right) { count += 1; }
    // right & down
    if !in_area(right) && !in_area(down) { count += 1; }
    // down & left
    if !in_area(down) && !in_area(left) { count += 1; }
    // left & top
    if !in_area(left) && !in_area(top) { count += 1; }

    // 2) Four diagonal corners
    // top & right in area, but diagonal top-right out
    if in_area(top) && in_area(right) {
        let top_right = (top.0, right.1);
        if !in_area(top_right) { count += 1; }
    }
    // right & down
    if in_area(right) && in_area(down) {
        let right_down = (down.0, right.1);
        if !in_area(right_down) { count += 1; }
    }
    // down & left
    if in_area(down) && in_area(left) {
        let down_left = (down.0, left.1);
        if !in_area(down_left) { count += 1; }
    }
    // left & top
    if in_area(left) && in_area(top) {
        let left_top = (top.0, left.1);
        if !in_area(left_top) { count += 1; }
    }

    count
}
fn part1(grid: &[Vec<char>]) -> i32 {
    let areas = find_areas(grid);

    let mut sum = 0;
    for (_char_type, char_areas) in &areas {
        for area in char_areas {
            let perimeter = calculate_perimeter(area, grid);
            sum += area.len() as i32 * perimeter as i32;
        }
    }

    sum
}

fn part2(grid: &[Vec<char>]) -> i32 {
    let areas = find_areas(grid);

    let mut sum = 0;
    for (_char_type, char_areas) in &areas {
        for area in char_areas {
            let sides = calculate_sides(area);
            sum += area.len() as i32 * sides as i32;
        }
    }

    sum
}

fn main() {
    let grid = aoc::utils::load_input_as_char_matrix("inputs/12.txt");
    println!("Part 1: {}", part1(&grid)); // Part 1: 1456082
    println!("Part 2: {}", part2(&grid)); // Part 2: 872382
}
