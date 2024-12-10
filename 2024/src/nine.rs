// 0  1  2
// 12 34 5
// 0..111....22222
fn map_diskmap_blockform(disk_map: &str) -> String {
    // we read the diskmap from left to right, 2 digits at the time
    // if there's a lone digit at the end, it means there's no free space after it.

    // the first number is the block number, the second how many blocks of free space.
    // the first block number to the left has ID 0, the second 1 and so on.
    // So for each pair of input numbers we have (ID, BLOCK_NUMBER, FREE_BLOCKS)
    // 12345 -> (ID=0,BLOCK_NUMBER=1,FREE_BLOCKS=2), (ID=1,BLOCK_NUMBER=3,FREE_BLOCKS=4), (ID=2,BLOCK_NUMBER=5,FREE_BLOCKS=0)

    // we need to map this to a block form, where each block is logic-wise a tuple of (ID, BLOCK_NUMBER, FREE_BLOCKS)

    let mut blockform = String::new();
    let mut id: u32 = 0;
    let disk_map_bytes: &[u8] = disk_map.as_bytes();
    let disk_map_len = disk_map_bytes.len();

    let mut i = 0;
    while i < disk_map_len {
        let block_number = (disk_map_bytes[i] - b'0') as usize;
        let free_blocks = if i + 1 < disk_map_len {
            (disk_map_bytes[i + 1] - b'0') as usize
        } else {
            0
        };

        //println!("disk_map_bytes[{}] = {}", i, disk_map_bytes[i]);
        let id_char = std::char::from_digit(id % 10, 10).unwrap();

        // Append the block number as `id_char`
        blockform.extend(std::iter::repeat(id_char).take(block_number));

        // Append free blocks as '.'
        blockform.extend(std::iter::repeat('.').take(free_blocks));

        id += 1;
        i += 2;
    }

    blockform
}

fn defrag_blockform(blockform: String, debug: bool) -> String {
    // We grab the rightmost block, and move it to the leftmost free space
    // We place a '.' in its place
    // We repeat this until there are no more blocks to move
    let mut defragged: Vec<char> = blockform.chars().collect();

    loop {
        // Find the leftmost free space
        let leftmost_free_space = defragged.iter().position(|&c| c == '.').unwrap();

        // Find the rightmost block
        if let Some(rightmost_block) = defragged.iter().rposition(|&c| c != '.') {
            // If it's to the left of the leftmost free space, we're done
            if rightmost_block < leftmost_free_space {
                break;
            }
            // Move the rightmost block to the leftmost free space
            defragged[leftmost_free_space] = defragged[rightmost_block];
            defragged[rightmost_block] = '.';
        } else {
            break;
        }

        // convert Vec<char> to String
        if debug {
            let defragged_str: String = defragged.iter().collect();
            println!("{}", defragged_str);
        }
    }

    // Convert the vector of characters back into a string
    defragged.into_iter().collect()
}

// We need to calculate the checksum of the defragged disk
// We multiply each digit by its position on the string, we can skip the 0th since it's * 0 = 0
// then sum
fn checksum(defragged: &String) -> usize {
    let mut sum = 0;
    let mut index = 0;

    for c in defragged.chars() {
        if c != '.' {
            if let Some(digit) = c.to_digit(10) {
                sum += index * digit as usize;
            }
        }
        index += 1;
    }

    sum
}

fn part1(disk_map: &str) -> usize {
    // map diskmap to
    let blockform = map_diskmap_blockform(disk_map);
    //println!("{:?}", blockform);
    let defragged = defrag_blockform(blockform, false);
    //println!("{:?}", defragged);
    checksum(&defragged)
}

fn main() {
    let disk_map = aoc::utils::load_input_as_string("inputs/9.txt");
    println!("Part 1: {}", part1(&disk_map));
}

#[test]
fn test_map_diskmap_blockform() {
    assert_eq!(map_diskmap_blockform("12345"), "0..111....22222");
    assert_eq!(
        map_diskmap_blockform("2333133121414131402"),
        "00...111...2...333.44.5555.6666.777.888899"
    )
}

#[test]
fn test_defrag() {
    assert_eq!(
        defrag_blockform("0..111....22222".to_string(), true),
        "022111222......"
    );
    assert_eq!(
        defrag_blockform(
            "00...111...2...333.44.5555.6666.777.888899".to_string(),
            true
        ),
        "0099811188827773336446555566.............."
    )
}
