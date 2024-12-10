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

        let id_char = std::char::from_digit(id, 10).unwrap();

        // Append the block number as `id_char`
        blockform.extend(std::iter::repeat(id_char).take(block_number));

        // Append free blocks as '.'
        blockform.extend(std::iter::repeat('.').take(free_blocks));

        id += 1;
        i += 2;
    }

    blockform
}

fn part1(disk_map: &str) -> i32 {
    // map diskmap to
    let blockform = map_diskmap_blockform(disk_map);
    println!("{:?}", blockform);
    0
}

fn main() {
    let disk_map = aoc::utils::load_input_as_string("inputs/9.test.txt");
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
