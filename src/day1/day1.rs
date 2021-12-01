// 6:30

use std::fs;
use std::env;
use std::path::Path;
use itertools::Itertools;

static PATH : &str = "src/day1/input_day1.txt";

fn read_input_to_vector ( input_path: &str ) -> Vec<i32> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { input.lines().map(|s| s.to_string().parse::<i32>().unwrap()).collect()}
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn check_depth ( depths : Vec<i32> ) -> i32
{ 
    depths.windows(2).filter(|x| x[0] <  x[1] ).count() as i32
}

fn check_depth_sliding_window ( depths : Vec<i32> ) -> i32
{ 
    let counts: Vec<i32>= depths.windows(3).map(|x| x[0]+x[1]+x[2]).collect();
    counts.windows(2).filter(|x| x[0] <  x[1] ).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

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
