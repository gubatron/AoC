use aoc_2015::utils;

fn map_to_int_tuple(s: &String) -> (i32, i32, i32) {
    let mut number_list: Vec<i32> = s.split("x").map(|s| -> i32{ s.to_string().parse().unwrap() }).collect();
    number_list.sort();
    (*number_list.get(0).unwrap(), *number_list.get(1).unwrap(), *number_list.get(2).unwrap())
}

fn dimensions_to_rect_prism_surface_area_plus_smallest_side(tuple: (i32, i32, i32)) -> i32 {
    2 * (tuple.0 * tuple.1 + tuple.0 * tuple.2 + tuple.1 * tuple.2) + tuple.0 * tuple.1
}

fn dimensions_to_ribbons(t: (i32, i32, i32)) -> i32 {
    t.0 * 2 + t.1 * 2 + (t.0 * t.1 * t.2)
}

fn part1() -> i32 {
    utils::load_input_lines_as_vec_str("src/2.txt").
        iter().
        map(|line| map_to_int_tuple(line)).
        map(|dimensions| dimensions_to_rect_prism_surface_area_plus_smallest_side(dimensions)).
        sum()
}

fn part2() -> i32 {
    utils::load_input_lines_as_vec_str("src/2.txt").
        iter().
        map(|line| map_to_int_tuple(line)).
        map(|dimensions| dimensions_to_ribbons(dimensions)).
        sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}