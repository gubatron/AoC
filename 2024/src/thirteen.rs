use aoc::utils::{dijkstra,load_input_break_by_empty_lines_as_vec_str,Coord};
use regex::Regex;

// A - 3 tokens
// B - 1 token
#[derive(Debug)]
struct ButtonConfig {
    movement:Coord,
    cost:i32
}

#[derive(Debug)]
struct PrizeConfig {
    button_a:ButtonConfig,
    button_b:ButtonConfig,
    prize:Coord
}

fn parse_button(input:&str, input_cost:i32) -> Option<ButtonConfig> {
    let button_regex = "X\\+(\\d+), Y\\+(\\d+)";
    let re = Regex::new(button_regex).unwrap();
    if let Some(captures) = re.captures(input) {
        let x_group:i32 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group:i32 = captures.get(2).unwrap().as_str().parse().expect("NaN");
        
        println!("Button X coordinate: {}", x_group);
        println!("Button Y coordinate: {}", y_group);
        Some(ButtonConfig{ movement: Coord{ x:x_group, y:y_group}, cost:input_cost})
    } else {
        println!("Button '{}' didnt match the regex", input);
        None
    }
}

fn parse_prize_coord(input:&str) -> Option<Coord> {
     //Prize: X=8400, Y=5400
    let prize_regex = "X=(\\d+), Y=(\\d+)";
    let re = Regex::new(prize_regex).unwrap();   
    if let Some(captures) = re.captures(input) {
        let x_group:i32 = captures.get(1).unwrap().as_str().parse().expect("NaN");
        let y_group:i32 = captures.get(2).unwrap().as_str().parse().expect("NaN");
        
        println!("Prize X coordinate: {}", x_group);
        println!("Prize Y coordinate: {}", y_group);
        Some(Coord{ x:x_group, y:y_group})
    } else {
        println!("Prize '{}' didnt match the regex", input);
        None
    }

}

// Right is along the X axis8400/04
// Forward is along the Y axis
fn parse_config_prize(input:String) -> PrizeConfig {
    let input_vec : Vec<&str> = input.split("\n").collect();
    let button_a_str = input_vec[0];
    let button_b_str = input_vec[1];
    let prize_str = input_vec[2];

    // Button A
    let parsed_button_a : ButtonConfig = parse_button(button_a_str, 3).unwrap();
    
    // Button B
    let parsed_button_b : ButtonConfig = parse_button(button_b_str, 1).unwrap();

    // Prize
    let parsed_prize : Coord = parse_prize_coord(prize_str).unwrap();
    
    PrizeConfig{
        button_a: parsed_button_a,
        button_b: parsed_button_b,
        prize: parsed_prize
    }
}


fn part1(configs:Vec<PrizeConfig>) -> i32 {
  0
}

fn main() {
  println!("Hello 13");
  let inputs : Vec<String> = load_input_break_by_empty_lines_as_vec_str("inputs/13.test.txt");

  for input in inputs {
      println!("{:?}", parse_config_prize(input));
  }


}
