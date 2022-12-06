pub mod utils {
    use std::fmt::Display;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;
    use std::vec::Vec;

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

    #[test]
    pub fn test_load_input_as_vec_int() {
        let vec_nums = load_input_as_vec_int("src/numbers.txt");
        assert_eq!(vec_nums.iter().sum::<i32>(), 0)
    }
}
