use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// We'll represent the disk as a vector of isize:
// - file block: its ID as a positive number (or zero)
// - free space: -1
type Disk = Vec<isize>;

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

// Find the start and length of a contiguous run of the given file_id.
// Returns (start, length) or None if file not found.
fn find_file_run(disk: &Disk, file_id: isize) -> Option<(usize, usize)> {
    let mut start = None;
    let mut length = 0;
    for (i, &b) in disk.iter().enumerate() {
        if b == file_id {
            if start.is_none() {
                start = Some(i);
                length = 1;
            } else {
                length += 1;
            }
        }
    }

    if let Some(st) = start {
        Some((st, length))
    } else {
        None
    }
}

// Find a contiguous run of free space (-1) to the left of `file_start` that can fit `file_length` blocks.
// Return (start_of_free_run) of where to place the file.
fn find_suitable_free_run_to_left(disk: &Disk, file_start: usize, file_length: usize) -> Option<usize> {
    // We want a contiguous run of -1 somewhere before file_start
    // We'll scan from left to right for free runs that end before file_start
    let mut free_start = None;
    let mut free_len = 0;
    let mut best_start = None;

    for i in 0..file_start {
        if disk[i] == -1 {
            if free_start.is_none() {
                free_start = Some(i);
                free_len = 1;
            } else {
                free_len += 1;
            }
        } else {
            // we hit a block, check if we had a free run going
            if let Some(st) = free_start {
                if free_len >= file_length {
                    // Found a suitable run. Since we want the leftmost suitable run, we can return immediately
                    return Some(st);
                }
            }
            free_start = None;
            free_len = 0;
        }
    }

    // End of the loop, check if the last run was suitable
    if let Some(st) = free_start {
        if free_len >= file_length {
            return Some(st);
        }
    }

    best_start
}

// Move the file (given by start and length) to the given free_start index
fn move_file(disk: &mut Disk, file_start: usize, file_length: usize, free_start: usize) {
    let file_id = disk[file_start];
    // Clear the old location
    for i in file_start..file_start+file_length {
        disk[i] = -1;
    }
    // Place file at the free run start
    for i in 0..file_length {
        disk[free_start + i] = file_id;
    }
}






// Entry point for part1
fn part1(disk_map: &str) -> usize {
    let disk = map_diskmap_blockform(disk_map);
    let defragged = defrag_disk(disk, false);
    println!("{}",disk_to_debug_string(&defragged));
    checksum(&defragged)
}

fn part2(disk_map: &str) -> usize {
    let mut disk = map_diskmap_blockform(disk_map);

    // Determine the max file ID
    let max_id = *disk.iter().max().unwrap();

    // For each file from max_id down to 0, try to move it
    for fid in (0..=max_id).rev() {
        // Find the file run
        if let Some((start, length)) = find_file_run(&disk, fid) {
            // Find suitable free run to the left
            if let Some(free_start) = find_suitable_free_run_to_left(&disk, start, length) {
                // Move the file
                move_file(&mut disk, start, length, free_start);
            }
        }
    }

    checksum(&disk)
}


fn main() {
    let disk_map = aoc::utils::load_input_as_string("inputs/9.txt");
    println!("Part 1: {}", part1(&disk_map)); // Part 1: 6385338159127
    println!("Part 2: {}", part2(&disk_map)); // Part 2: 6415163624282
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
