pub mod utils {
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::fs;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::ops::Deref;
    use std::path::Path;
    use std::vec::Vec;

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

    pub fn load_input_break_by_empty_lines_as_vec_str(filename: impl AsRef<Path> + Display + Copy) -> Vec<String> {
        let input_str = fs::read_to_string(&filename).expect(format!("Unable to read file {}", filename).as_str());
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

    pub trait GraphNode {
        fn equals(&self, other: &Self) -> bool;
    }

    pub fn bfs<T: GraphNode + PartialEq + Eq + Hash + Clone + Copy>(source: T, target: T, graph: &HashMap<T, Vec<T>>) -> Vec<T> {
        let mut queue = vec![source];
        let mut visited :Vec<T> = vec![];

        while !queue.is_empty() {
            let node = queue.remove(0);
            if node.equals(&target) {
                return vec![node];
            }
            if visited.contains(&node) {
                continue;
            }
            visited.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for nref in neighbors.iter() {
                    let neighbor : T = nref.deref().clone();
                    queue.push(neighbor);
                }
            }
        }
        visited
    }

    #[test]
    pub fn test_load_input_as_vec_int() {
        let vec_nums = load_input_as_vec_int("src/numbers.txt");
        assert_eq!(vec_nums.iter().sum::<i32>(), 0)
    }
}
