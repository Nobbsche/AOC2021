// 6:30

use std::fs;
use itertools::Itertools;

static PATH : &str = "D:/AOC/AOC2022/day1/src/input_day1.txt";

fn read_input_to_vector ( input_path: &str ) -> Vec<i32> {
    match fs::read_to_string(input_path) {
        Ok (input) => { input.lines().map(|s| s.to_string().parse::<i32>().unwrap()).collect()}
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn check_depth ( depths : Vec<i32> ) -> i32
{ 
    let mut current_value = depths[0];
    let mut inc_counter = 0;

    for i in 1..depths.len() {
        if current_value < depths[i] {
            inc_counter += 1;
        }
        current_value = depths[i];
    }
    inc_counter
}

fn check_depth_sliding_window ( depths : Vec<i32> ) -> i32
{ 

    let mut inc_counter = 0;

    let counts: Vec<i32>= depths.windows(3).map(|x| x[0]+x[1]+x[2]).collect();
    let mut current_value = counts[0];

    for i in 1..counts.len() {
        if current_value < counts[i] {
            inc_counter += 1;
        }
        current_value = counts[i];
    }
    inc_counter
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PATH;
    
    #[test]
    fn read_path() {
        let depths = read_input_to_vector(PATH);
        assert_eq!(depths.is_empty(), false);
    }

    #[test]
    fn increase_counter() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(check_depth(depths), 7);
    }

    #[test]
    fn riddle_1() {
        let depths = read_input_to_vector(PATH);
        assert_eq!(check_depth(depths), 1121);  
    }

    #[test]
    fn sliding_window() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(check_depth_sliding_window(depths), 5);
    }

    #[test]
    fn riddle_2() {
        let depths = read_input_to_vector(PATH);
        assert_eq!(check_depth_sliding_window(depths), 1065);
    }
}
