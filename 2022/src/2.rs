use aoc::utils;
use crate::Action::{DRAW, LOSE, WIN};
use crate::Shape::{PAPER, ROCK, SCISSORS, UNKNOWN};

fn main() {
    let moves = utils::load_input_lines_as_vec_str("2.txt");
    part1(&moves); //part1: 11666
    part2(&moves);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
    UNKNOWN
}

#[derive(PartialEq, Eq)]
enum Action {
    LOSE,
    DRAW,
    WIN,
    UNKNOWN
}

fn part1(moves: &Vec<String>) {
    let total_score = moves.iter().map(|line| {
        round_score(line)
    }).sum::<i32>();
    print!("part1: {}\n", total_score);
}

fn part2(moves: &Vec<String>) {
    let total_score = moves.iter().map(|line| {
        round_score_2(line)
    }).sum::<i32>();
    print!("part2: {}\n", total_score);

}

fn round_score(line: &String) -> i32 {
    let enemy_move = char_2_rps(line.chars().nth(0).unwrap());
    let my_move = char_2_rps(line.chars().nth(2).unwrap());
    return move_score(&enemy_move, &my_move) + shape_score(&my_move);
}

fn round_score_2(line : &String) -> i32 {
    let enemy_move = char_2_rps(line.chars().nth(0).unwrap());
    let my_action = char_2_action(line.chars().nth(2).unwrap());
    let my_move : Shape = decide_move(&enemy_move, &my_action);
    return move_score(&enemy_move, &my_move) + shape_score(&my_move);
}

fn char_2_rps(c: char) -> Shape {
    return match c {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => { UNKNOWN }
    };
}

fn char_2_action(c : char) -> Action {
    if c == 'X' {
        return LOSE;
    } else if c == 'Y' {
        return DRAW;
    } else if c == 'Z' {
        return WIN;
    }
    return Action::UNKNOWN;
}

fn shape_score(shape: &Shape) -> i32 {
    return match shape {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        UNKNOWN => 0
    }
}

fn move_score(enemy: &Shape, mine: &Shape) -> i32 {
    if enemy == mine {
        return 3;
    } else if *enemy == ROCK && *mine == PAPER {
        return 6;
    } else if *enemy == ROCK && *mine == SCISSORS {
        return 0;
    } else if *enemy == PAPER && *mine == ROCK {
        return 0;
    } else if *enemy == PAPER && *mine == SCISSORS {
        return 6;
    } else if *enemy == SCISSORS && *mine == ROCK {
        return 6;
    } else if *enemy == SCISSORS && *mine == PAPER {
        return 0;
    }
    return 0;
}

fn decide_move(enemy: &Shape, action: &Action) -> Shape {
    if *action == DRAW {
        return *enemy;
    }

    if *action == LOSE {
        if *enemy == ROCK {
            return SCISSORS;
        } else if *enemy == PAPER {
            return ROCK;
        } else if *enemy == SCISSORS {
            return PAPER;
        }
    }

    if *action == WIN {
        if *enemy == ROCK {
            return PAPER;
        } else if *enemy == PAPER {
            return SCISSORS;
        } else if *enemy == SCISSORS {
            return ROCK;
        }
    }
    return UNKNOWN;
}
