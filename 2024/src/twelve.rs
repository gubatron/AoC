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

fn main() {
    let grid = aoc::utils::load_input_as_char_matrix("inputs/12.txt");
    println!("Part 1: {}", part1(&grid)); // Part 1: 1456082
}
