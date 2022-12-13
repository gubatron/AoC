use std::collections::HashMap;
use aoc_2022::utils::{bfs, GraphNode};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl GraphNode for Coord {
    fn equals(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl GraphNode for &Coord {
    fn equals(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn neighbors(&self, height_matrix: &Vec<Vec<u32>>, consider_diagonals: bool) -> Vec<Coord> {
        let mut surrounding = vec![];
        let mut deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        if consider_diagonals {
            deltas.extend(vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
        }
        let rows = height_matrix.len() as i32;
        let cols = height_matrix[0].len() as i32;

        let my_height = height_matrix[self.y as usize][self.x as usize];

        for (dx, dy) in deltas {
            let x = self.x + dx;
            let y = self.y + dy;

            if x >= 0 && x < rows && y >= 0 && y < cols {
                //we do this inside to avoid index out of bounds panic
                let current_height = height_matrix[x as usize][y as usize];
                let height_diff = my_height.abs_diff(current_height);
                if height_diff >= 0 && height_diff <= 2 {
                    surrounding.push(Coord::new(x, y));
                }
            }
        }

        surrounding
    }
}

impl From<(i32, i32)> for Coord {
    fn from(xy: (i32, i32)) -> Self {
        Coord {
            x: xy.0,
            y: xy.1,
        }
    }
}

fn main() {
    let input = aoc_2022::utils::load_input_lines_as_vec_str("12.test.txt");

    // convert input into a Vec<Vec<char>> and extract the coordinates of 'S' and 'E' into 2 variables made of (u32,u32)
    let mut map: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let (mut s_x, mut s_y) = (0, 0);
    let (mut e_x, mut e_y) = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                s_x = x as u32;
                s_y = y as u32;
            } else if *c == 'E' {
                e_x = x as u32;
                e_y = y as u32;
            }
        }
    }

    // print s_x, s_y, e_x, e_y
    println!("s_x: {}, s_y: {}, e_x: {}, e_y: {}", s_x, s_y, e_x, e_y);


    // convert input into a Vec<Vec<u32> where a=1, b=2, c=3, .. z=26
    // special cases are S=1 and E=26
    let mut height_matrix: Vec<Vec<u32>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'a'..='z' => c as u32 - 96,
                    'S' => 1,
                    'E' => 26,
                    _ => panic!("unexpected char"),
                })
                .collect()
        })
        .collect();

    // print the Vec<Vec<u32>> as an NxN matrix
    for row in &height_matrix {
        for col in row {
            print!("{:02} ", col);
        }
        println!();
    }

    let graph = create_graph(&height_matrix, false);
    let visited = bfs::<Coord>(Coord::new(s_x as i32, s_y as i32), Coord::new(e_x as i32, e_y as i32), &graph);

    println!("total visited: {:?}", visited.len());


    // create an adjacency matrix of the graph where the nodes are the coordinates of the map
    // and are only adjacent if their height difference is only 1
    //let mut graph = HashMap::new();
    //println!("path: {}", path.len());
}


fn create_graph(height_matrix: &Vec<Vec<u32>>, consider_diagonals: bool) -> HashMap<Coord, Vec<Coord>> {
    let mut graph = HashMap::new();

    let rows = height_matrix.len() as i32;
    let cols = height_matrix[0].len() as i32;

    for y in 0..rows {
        for x in 0..cols {
            let coord = Coord::new(x, y);
            let neighbors = coord.neighbors(height_matrix, consider_diagonals);
            graph.insert(coord, neighbors);
        }
    }
    graph
}

// returns a vector with the neighboring points of a given point (x, y)
fn get_neighbors(point: (u32, u32), width: usize, height: usize) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::new();
    let (x, y) = point;

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < (width - 1) as u32 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < (height - 1) as u32 {
        neighbors.push((x, y + 1));
    }

    neighbors
}