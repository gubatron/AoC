pub mod utils {
    use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
    use std::fmt::Display;
    use std::fs;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;
    use std::str::FromStr;
    use std::vec::Vec;

    pub fn split_and_parse<T>(input: &str, separator: &str) -> Result<Vec<T>, T::Err>
    where
        T: FromStr,
    {
        input
            .split(separator) // Split the input by the given separator
            .map(|s| s.trim().parse::<T>()) // Trim whitespace and parse into type T
            .collect() // Collect the parsed results into a Vec<T>
    }

    pub fn convert_comma_separated_number_list_to_vec_t<T>(input: &String) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        input
            .split(",")
            .map(|s| s.trim().parse::<T>().unwrap())
            .collect::<Vec<T>>()
    }

    pub fn load_input_break_by_empty_lines_as_vec_str(
        filename: impl AsRef<Path> + Display + Copy,
    ) -> Vec<String> {
        let input_str = fs::read_to_string(&filename)
            .expect(format!("Unable to read file {}", filename).as_str());
        let split = input_str.split("\n\n");
        let mut result = Vec::new();
        for s in split {
            result.push(s.to_string());
        }
        result
    }

    pub fn load_input_as_string(filename: impl AsRef<Path> + Display + Copy) -> String {
        fs::read_to_string(filename).expect(format!("Unable to read file {}", filename).as_str())
    }

    // Load a text file and return each line as a String in a Vec<string>
    pub fn load_input_lines_as_vec_str(filename: impl AsRef<Path> + Display) -> Vec<String> {
        let error_string = format!("file {} not found", filename);
        let file = File::open(filename).expect(&*error_string);
        let buf = BufReader::new(file);
        buf.lines()
            .map(|line| line.expect("error reading line"))
            .collect()
    }

    pub fn load_input_as_char_matrix(filename: impl AsRef<Path> + Display) -> Vec<Vec<char>> {
        let input = load_input_lines_as_vec_str(filename);
        input.iter().map(|line| line.chars().collect()).collect()
    }

    pub fn load_input_as_single_digit_matrix(filename: impl AsRef<Path> + Display) -> Vec<Vec<i32>> {
        let input = load_input_lines_as_vec_str(filename);
        input
            .iter()
            .map(|line| line.as_bytes().iter().map(|&c| (c - b'0') as i32).collect())
            .collect()
    }

    pub fn load_input_as_vec_char(filename: impl AsRef<Path> + Display) -> Vec<char> {
        let lines_vec = load_input_lines_as_vec_str(filename);
        let mut result = vec![];
        for line in lines_vec {
            for c in line.chars() {
                result.push(c);
            }
        }
        result
    }

    // Load each line of a file as an i32 and add it to a Vec<i32>
    pub fn load_input_as_vec_int(filename: impl AsRef<Path> + Display) -> Vec<i32> {
        let vec_str = load_input_lines_as_vec_str(filename);
        let vec_int: Vec<i32> = vec_str
            .iter()
            .map(|l| -> i32 { l.parse().unwrap() })
            .collect();
        vec_int
    }

    pub fn get_unique_substring_offset(s: &str, n: usize) -> usize {
        for i in 0..(s.len() - n + 1) {
            let substring = &s[i..i + n];
            let mut chars = std::collections::HashSet::new();
            let all_chars_different = substring.chars().all(|c| chars.insert(c));
            if all_chars_different {
                return i;
            }
        }
        s.len()
    }

    pub fn char_to_digit(c : char) -> Result<u32, String> {
        if c < '0' || c > '9' {
            return Err(format!("Invalid character {}", c))
        }
        Ok((c as u8 - b'0' as u8) as u32)
    }

    #[derive(Debug, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
    pub struct Coord {
        pub x: i32,
        pub y: i32,
    }

    impl Coord {
        pub fn new(x: i32, y: i32) -> Coord {
            Coord { x, y }
        }

        pub fn manhattan_distance(&self, other: &Coord) -> i32 {
            (self.x - other.x).abs() + (self.y - other.y).abs()
        }

        pub fn midpoint(&self, b: &Coord) -> Option<Coord> {
            let dx = (b.x - self.x) / 2;
            let dy = (b.y - self.y) / 2;

            // Check if the Manhattan distance is evenly divisible
            if (b.x - self.x).abs() % 2 == 0 && (b.y - self.y).abs() % 2 == 0 {
                Some(Coord::new(self.x + dx, self.y + dy))
            } else {
                None
            }
        }
    }

    impl PartialEq for Coord {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    pub fn neighbors(node: &Coord, rows: i32, cols: i32, consider_diagonals: bool) -> Vec<Coord> {
        let mut friends = vec![];
        let mut deltas = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        if consider_diagonals {
            deltas.extend(vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
        }
        for (dx, dy) in deltas {
            // test swapping these
            let y = node.y + dy;
            let x = node.x + dx;

            if x >= 0 && x < cols && y >= 0 && y < rows {
                friends.push(Coord::new(x, y));
            }
        }
        friends
    }

    pub fn dijkstra<T>(start: T, graph: &HashMap<T, Vec<T>>) -> HashMap<T, i32>
    where
        T: Ord + Hash + Copy + Eq + std::fmt::Debug,
    {
        let mut distances = HashMap::<T, i32>::new();
        let mut queue = BinaryHeap::<(i32, T)>::new();
        distances.insert(start, 0);
        queue.push((0, start));

        while !queue.is_empty() {
            let (dist, node) = queue.pop().unwrap();
            if let Some(neighbors) = graph.get(&node) {
                for n in neighbors {
                    let new_dist = dist + 1;
                    if !distances.contains_key(n) || new_dist < distances[n] {
                        distances.insert(*n, new_dist);
                        queue.push((new_dist, *n));
                    }
                }
            }
        }
        distances
    }

    pub fn dimensions_cols_rows<T>(matrix: &Vec<Vec<T>>) -> (usize, usize) {
        (matrix[0].len(), matrix.len())
    }

    pub fn bfs<T>(start: T, end: T, graph: &HashMap<T, Vec<T>>) -> (i32, HashSet<T>)
    where
        T: PartialEq + Eq + Hash + Clone + Copy + Ord + std::fmt::Debug,
    {
        // <(steps to get to this point as horizon opens, node)>
        let mut queue = VecDeque::<(i32, T)>::new();
        let mut seen = HashSet::<T>::new();
        queue.push_back((0, start));
        seen.insert(start);
        while !queue.is_empty() {
            let (dist, node) = queue.pop_front().unwrap();
            if node == end {
                return (dist, seen);
            }
            if let Some(neighbors) = graph.get(&node) {
                for n in neighbors {
                    if !seen.contains(n) {
                        seen.insert(*n);
                        queue.push_back((dist + 1, *n));
                    }
                }
            }
        }
        (i32::MAX, seen)
    }

    #[test]
    pub fn test_load_input_as_vec_int() {
        let vec_nums = load_input_as_vec_int("numbers.txt");
        assert_eq!(vec_nums.iter().sum::<i32>(), 0)
    }

    #[test]
    pub fn test_split_and_parse() {
        let input = "42, 15, 8, 23";
        let result: Result<Vec<u32>, _> = split_and_parse(input, ",");
        match result {
            Ok(parsed) => {
                println!("Parsed numbers: {:?}", parsed);
                assert_eq!(parsed.iter().sum::<u32>(), 88);
            }
            Err(e) => println!("Error parsing input: {:?}", e),
        }

        let bool_input = "true,false,true";
        let bool_result: Result<Vec<bool>, _> = split_and_parse(bool_input, ",");
        match bool_result {
            Ok(parsed) => println!("Parsed booleans: {:?}", parsed),
            Err(e) => println!("Error parsing booleans: {:?}", e),
        }
    }

    #[test]
    pub fn test_dimensions_cols_rows() {
        // (5,2)
        let a = vec![vec!['a', 'b', 'c', 'd', 'e'], vec!['1', '2', '3', '4', '5']];
        assert_eq!(dimensions_cols_rows(&a), (5, 2));
        let b = vec![vec!['a']];
        assert_eq!(dimensions_cols_rows(&b), (1, 1));
    }

    #[test]
    pub fn test_load_input_as_single_digit_matrix() {
        // 012345
        // 678901
        // 234567
        // 890123
        // 456789
        let matrix = load_input_as_single_digit_matrix("single_digit_matrix.txt");
        // matrix[y][x]
        assert_eq!(matrix[2][3], 5);
        let mut i = 0;
        for row in matrix {
            for cell in row {
                assert_eq!(cell, i % 10);
                i += 1;
            }
        }
    }
}
