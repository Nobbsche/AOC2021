//6:55

use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::Path;
use itertools::Itertools;

#[derive(Debug)]
pub struct Command {
    command : String,
    motion : i32,
}

static PATH : &str = "src/day2/input_day2.txt";

fn read_input_to_map ( input_path: &str ) -> Vec<Command> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    let mut input_vec = vec![];
    match fs::read_to_string(cpath) {
        Ok (input) => { 
                println!("input: {:?}", input);
                for line in input.lines() {
                    let t : Vec<&str> = line.split(' ').collect();
                    input_vec.push( Command{ command: t[0].to_owned(), motion : t[1].to_string().parse().unwrap()} );
                }   
        },
        Err (e) => {panic!("Could not parse input: {:?}", e);}
    }
    println!("map: {:?}", input_vec);
    input_vec
}

fn calculate_motion ( input_map : Vec<Command> ) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    for c in input_map.iter() {
        println!("k: {:?} - v : {:?}", c.command, c.motion);
        match c.command.as_str() {
            "forward" => { forward += c.motion ;}
            "up" => {depth -= c.motion;}
            "down" => {depth += c.motion;}
            _ => {panic!("unknown command");}
        }
        println!("d: {:?} - f : {:?}", depth, forward);
    }
    println!("d: {:?} - f : {:?}", depth, forward);
    depth*forward
}

fn calculate_motion_with_aim ( input_map : Vec<Command> ) -> i32 {
    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for c in input_map.iter() {
        println!("k: {:?} - v : {:?}", c.command, c.motion);
        match c.command.as_str() {
            "forward" => { forward += c.motion;
                           depth += aim * c.motion; }
            "up" => {aim -= c.motion;}
            "down" => {aim += c.motion;}
            _ => {panic!("unknown command");}
        }
        println!("d: {:?} - f : {:?}", depth, forward);
    }
    println!("d: {:?} - f : {:?}", depth, forward);
    depth*forward
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_path() {
        let depths = read_input_to_map(PATH);
        assert_eq!(depths.is_empty(), false);
    }

    #[test]
    fn motion_calculation() {
        let mut in_map = vec![];
        in_map.push(Command{ command: String::from("forward"), motion :5 });
        in_map.push(Command{ command: String::from("down"), motion: 5 });
        in_map.push(Command{ command: String::from("forward"), motion: 8 });
        in_map.push(Command{ command: String::from("up"), motion : 3 });
        in_map.push(Command{ command: String::from("down"), motion : 8 });
        in_map.push(Command{ command: String::from("forward"), motion: 2});

        assert_eq!(calculate_motion(in_map), 150)
    }

    #[test]
    fn motion_calculation_with_aim() {
        let mut in_map = vec![];
        in_map.push(Command{ command: String::from("forward"), motion :5 });
        in_map.push(Command{ command: String::from("down"), motion: 5 });
        in_map.push(Command{ command: String::from("forward"), motion: 8 });
        in_map.push(Command{ command: String::from("up"), motion : 3 });
        in_map.push(Command{ command: String::from("down"), motion : 8 });
        in_map.push(Command{ command: String::from("forward"), motion: 2});

        assert_eq!(calculate_motion_with_aim(in_map), 900)
    }
    
    #[test]
    fn riddle_1() {
        let depths = read_input_to_map(PATH);
        assert_eq!(calculate_motion(depths), 2036120);
    }

    #[test]
    fn riddle_2() {
        let depths = read_input_to_map(PATH);
        assert_eq!(calculate_motion_with_aim(depths), 2015547716);
    }
}