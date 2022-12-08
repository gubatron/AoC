use std::fs;

fn main() {
    // Day 8: Treetop Tree House
    let tree_map = convert_string_lines_to_int_two_dimensional_vector_of_vectors(&fs::read_to_string(&mut "8.txt".to_string()).unwrap());
    part1(&tree_map); // part1: 1688
    part2(tree_map); // part2: 410400
}

fn part1(tree_map: &Vec<Vec<i32>>) {
    println!("part1: {}", count_visible_trees(tree_map)); // part1: 1688
}

fn part2(tree_map: Vec<Vec<i32>>) {
    let visible_trees: Vec<(usize, usize)> = get_visible_trees(&tree_map);
    let mut max_scenic_score = 0;
    for (row, col) in visible_trees {
        tree_map[row][col];
        let tree_scenic_score = scenic_score(row, col, &tree_map);
        if tree_scenic_score > max_scenic_score {
            max_scenic_score = tree_scenic_score
        }
    }
    println!("part2: {}", max_scenic_score);
}

fn scenic_score(row: usize, col: usize, tree_map: &Vec<Vec<i32>>) -> usize {
    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_above = 0;
    let mut score_below = 0;
    let tree = tree_map[row][col];

    for t in elements_above(row, col, &tree_map).into_iter() {
        if t >= tree {
            score_above = score_above + 1;
            break;
        }
        if t < tree {
            score_above = score_above + 1;
        }
    }

    for t in elements_left(row, col, &tree_map).into_iter() {
        if t >= tree {
            score_left = score_left + 1;
            break;
        }
        if t < tree {
            score_left = score_left + 1;
        }
    }

    for t in elements_right(row, col, &tree_map).into_iter() {
        if t >= tree {
            score_right = score_right + 1;
            break;
        }
        if t < tree {
            score_right = score_right + 1;
        }
    }

    for t in elements_below(row, col, &tree_map).into_iter() {
        if t >= tree {
            score_below = score_below + 1;
            break;
        }
        if t < tree {
            score_below = score_below + 1;
        }
    }
    score_left * score_right * score_above * score_below
}

fn count_visible_trees(tree_map: &Vec<Vec<i32>>) -> usize {
    let visible_trees = get_visible_trees(&tree_map);
    let outer_trees = 2 * (tree_map[0].len() + tree_map.len() - 2);
    return outer_trees + visible_trees.len();
}

fn get_visible_trees(tree_map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut visible_trees = vec![];
    for row in 1..tree_map.len() - 1 {
        for column in 1..tree_map[row].len() - 1 {
            if is_visible(row, column, &tree_map) {
                visible_trees.push((row, column));
            }
        }
    }
    return visible_trees;
}

fn is_visible(row: usize, col: usize, tree_map: &Vec<Vec<i32>>) -> bool {
    let tree_height = tree_map[row][col];
    is_taller_than_all(tree_height, elements_left(row, col, tree_map)) ||
        is_taller_than_all(tree_height, elements_right(row, col, tree_map)) ||
        is_taller_than_all(tree_height, elements_above(row, col, tree_map)) ||
        is_taller_than_all(tree_height, elements_below(row, col, tree_map))
}

fn is_taller_than_all(height: i32, elements: Vec<i32>) -> bool {
    elements.iter().all(|&x| x < height)
}

fn elements_above(row: usize, col: usize, vec: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..row {
        result.push(vec[i][col]);
    }
    result.reverse();
    return result;
}

fn elements_below(row: usize, col: usize, vec: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    for i in row + 1..vec.len() {
        result.push(vec[i][col]);
    }
    return result;
}

fn elements_left(row: usize, col: usize, vec: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let row_elements = &vec[row];
    for i in 0..col {
        result.push(row_elements[i]);
    }
    result.reverse();
    return result;
}

fn elements_right(row: usize, col: usize, vec: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let row_elements = &vec[row];
    for i in col + 1..row_elements.len() {
        result.push(row_elements[i]);
    }
    return result;
}

fn convert_string_lines_to_int_two_dimensional_vector_of_vectors(map_string: &String) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();
    for line in map_string.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        map.push(row);
    }
    map
}

#[test]
fn test_part1() {
    let map_string = r###"30373
25512
65332
33549
35390"###.to_string();
    println!("test1: {}", count_visible_trees(&convert_string_lines_to_int_two_dimensional_vector_of_vectors(&map_string))); //test1: 21
}

#[test]
fn test_part2() {
    let map_string = r###"30373
25512
65332
33549
35390"###.to_string();
    let tree_map = convert_string_lines_to_int_two_dimensional_vector_of_vectors(&map_string);
    let visible_trees: Vec<(usize, usize)> = get_visible_trees(&tree_map);
    let mut max_scenic_score = 0;
    for (row, col) in visible_trees {
        let tree_scenic_score = scenic_score(row, col, &tree_map);
        if tree_scenic_score > max_scenic_score {
            max_scenic_score = tree_scenic_score
        }
    }
    println!("max_scenic_score: {}", max_scenic_score);
}