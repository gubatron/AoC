use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// We'll represent the disk as a vector of isize:
// - file block: its ID as a positive number (or zero)
// - free space: -1
type Disk = Vec<isize>;

// Read the input file as a single line
fn load_input(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to read file");
    let line = io::BufReader::new(file)
        .lines()
        .next()
        .expect("No line in file")
        .unwrap();
    line
}

// Instead of returning a String of chars, we return a Vec<isize> representing the disk.
// Each file is assigned a unique integer ID starting at 0.
// Each free block is represented by -1.
fn map_diskmap_blockform(disk_map: &str) -> Disk {
    let disk_map_bytes: &[u8] = disk_map.as_bytes();
    let disk_map_len = disk_map_bytes.len();

    let mut disk: Disk = Vec::new();

    let mut id: isize = 0;
    let mut i = 0;
    while i < disk_map_len {
        let block_number = (disk_map_bytes[i] - b'0') as usize;
        let free_blocks = if i + 1 < disk_map_len {
            (disk_map_bytes[i + 1] - b'0') as usize
        } else {
            0
        };

        // Append 'block_number' times the current id
        for _ in 0..block_number {
            disk.push(id);
        }

        // Append 'free_blocks' times -1
        for _ in 0..free_blocks {
            disk.push(-1);
        }

        id += 1;
        i += 2;
    }

    disk
}

// Convert the disk (Vec<isize>) into a debug string like before, but only for small tests.
// For IDs, we take id % 10 to produce a single digit character. Free space is '.'.
fn disk_to_debug_string(disk: &Disk) -> String {
    let mut s = String::new();
    for &c in disk {
        if c == -1 {
            s.push('.');
        } else {
            let digit = (c % 10) as u32;
            s.push(std::char::from_digit(digit, 10).unwrap());
        }
    }
    s
}

// This function performs the defragmentation process exactly as described.
// Move one block at a time from the rightmost block to the leftmost gap until no moves remain.
fn defrag_disk(mut disk: Disk, debug: bool) -> Disk {
    loop {
        let leftmost_gap = match disk.iter().position(|&x| x == -1) {
            Some(pos) => pos,
            None => break, // no gaps
        };

        let rightmost_block = match disk.iter().rposition(|&x| x != -1) {
            Some(pos) => pos,
            None => break, // no blocks (unlikely)
        };

        // If no block is strictly to the right of the leftmost gap, done
        if rightmost_block <= leftmost_gap {
            break;
        }

        // Move rightmost block into the gap
        disk[leftmost_gap] = disk[rightmost_block];
        disk[rightmost_block] = -1;

        if debug {
            println!("{}", disk_to_debug_string(&disk));
        }
    }

    disk
}

// Calculate the checksum as specified.
// sum of (position * file_id) for each block (skip free spaces)
fn checksum(disk: &Disk) -> usize {
    let mut sum = 0;
    for (i, &id) in disk.iter().enumerate() {
        if id != -1 {
            sum += i * (id as usize);
        }
    }
    sum
}

// Entry point for part1
fn part1(disk_map: &str) -> usize {
    let disk = map_diskmap_blockform(disk_map);
    let defragged = defrag_disk(disk, false);
    println!("{}",disk_to_debug_string(&defragged));
    checksum(&defragged)
}

fn main() {
    let disk_map = load_input("inputs/9.txt");
    println!("Part 1: {}", part1(&disk_map)); // Part 1: 6385338159127
}

// Tests
#[test]
fn test_map_diskmap_blockform() {
    // For these small tests, IDs fit in a single digit anyway.
    let d = map_diskmap_blockform("12345");
    assert_eq!(disk_to_debug_string(&d), "0..111....22222");

    let d2 = map_diskmap_blockform("2333133121414131402");
    assert_eq!(disk_to_debug_string(&d2), "00...111...2...333.44.5555.6666.777.888899");
}

#[test]
fn test_defrag() {
    let d = map_diskmap_blockform("12345");
    let df = defrag_disk(d, true);
    // After defrag, using debug string:
    assert_eq!(disk_to_debug_string(&df), "022111222......");

    let d2 = map_diskmap_blockform("2333133121414131402");
    let df2 = defrag_disk(d2, true);
    assert_eq!(
        disk_to_debug_string(&df2),
        "0099811188827773336446555566.............."
    );
}
