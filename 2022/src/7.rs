use std::borrow::BorrowMut;
use std::rc::Rc;

fn main() {
    let command_log = aoc_2022::utils::load_input_lines_as_vec_str("inputs/7.test.txt");
    part1(command_log);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dir<'a> {
    name: String,
    sub_dirs: Vec<Dir<'a>>,
    files: Vec<File>,
    parent: Option<&'a Dir<'a>>,
}

impl Dir<'_> {
    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    // true if added, false if already exists
    pub fn try_add_dir_by_name(&mut self, dir_name: &String) -> bool {
        match self.get_dir(dir_name) {
            None => {
                let new_dir = Dir {
                    name: String::from(dir_name),
                    sub_dirs: Vec::new(),
                    files: Vec::new(),
                    parent: None,
                };
                self.sub_dirs.push(new_dir);
                true
            }
            Some(_) => {
                false
            }
        }
    }


    pub fn get_size(&self) -> i32 {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }
        for folder in self.sub_dirs.iter() {
            size += folder.get_size();
        }
        size
    }

    pub fn get_dir(&self, folder_name: &String) -> Option<&mut Dir> {
        let len_sub_dirs = self.sub_dirs.len();
        let mut i = 0;
        while i < len_sub_dirs {
            let folder = &mut self.sub_dirs[i];
            if folder.name.eq(folder_name) {
                return Some(folder);
            }
            i = i + 1;
        }
        None
    }

    pub fn print(&self, indent: i32) {
        for _ in 0..indent {
            print!(" ");
        }
        println!("- {} (dir)", self.name);
        for folder in &self.sub_dirs {
            folder.print(indent + 1);
        }

        for file in &self.files {
            for _ in 0..indent {
                print!(" ");
            }
            println!("- {} (file, size={})", file.name, file.size);
        }
    }

    pub fn depth(&self) -> i32 {
        if self.parent.is_none() {
            return 0;
        }
        return self.parent.as_ref().unwrap().depth() + 1;
    }
}

fn set_parent<'a>(dir_child: &mut Dir<'a>, dir_parent: &'a mut Dir<'a>) {
    dir_child.parent = Some(dir_parent);
}

fn starts_with_positive_integer(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_digit(10) && c != '0')
}

struct FileSystem<'a> {
    root: Dir<'a>,
    current_dir: Option<&'a Dir<'a>>,
}

fn part1(command_log: Vec<String>) {
    let mut fs = FileSystem {
        root: Dir {
            name: String::from("/"),
            sub_dirs: Vec::new(),
            files: Vec::new(),
            parent: None,
        },
        current_dir: None,
    };

    fs.current_dir = Some(&fs.root);

    for line in command_log.iter() {
        println!("line: {}", line);

        if line.starts_with("$ cd ") && !line.ends_with("/") {
            let dir_name = &line.split("$ cd ").last().unwrap().to_string();
            // CD ..
            if dir_name == ".." && fs.current_dir.unwrap().parent.is_some() {
                let parent = fs.current_dir.unwrap().parent;
                fs.current_dir = parent;
            }
            // CD <something>
            else {
                let a = fs.current_dir;
                let mut current_dir = a.unwrap().clone();

                if current_dir.try_add_dir_by_name(dir_name) {
                    let new_dir_opt = current_dir.get_dir(dir_name);
                    let mut new_dir = new_dir_opt.unwrap();
                    let mut new_dir = new_dir.borrow_mut();
                    set_parent(&mut new_dir, &mut current_dir);
                }
                let x = fs.current_dir.unwrap().get_dir(dir_name).unwrap();
                fs.current_dir = x;
            }
            println!("-> current directory: {}", fs.current_dir.unwrap().name);
        } else if line.starts_with("$ ls") {
            return;
        } else if line.starts_with("dir") {
            // LSing a dir
            let dir_name = &line.split("dir ").last().unwrap().to_string();
            fs.current_dir.unwrap().try_add_dir_by_name(dir_name);
        }
        // check if line starts with a number
        else if starts_with_positive_integer(line) {
            let mut parts = line.split(" ");
            let size = parts.next().unwrap().parse::<i32>().unwrap();
            let name = parts.next().unwrap();
            fs.current_dir.unwrap().add_file(File { name: String::from(name), size });
            println!("added file: {} {} (size={})", fs.current_dir.unwrap().name, name, size);
        }
    };

    fs.root.print(0);
}