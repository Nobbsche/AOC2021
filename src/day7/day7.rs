// 6:18
use std::fs;
use std::env;
use std::path::Path;

use ndarray::prelude::*;
use ndarray::Array;

static PATH : &str = "src/day7/input_day7.txt";

fn read_input_to_vector ( input_path: &str ) -> Vec<i64> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { input.split(',').filter_map(|s| { s.parse::<i64>().ok()} ).collect() }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn fuel_calculation (input_subs : Vec<i64>) -> i64 {
    let calc_base = input_subs.clone();
    let mut fuel_vec = vec![0; calc_base.len()];
    for (i, sub) in input_subs.iter().enumerate() {
        let mut fuel = 0;
        for (j, e) in calc_base.iter().enumerate() {
            if i == j { continue; }
            fuel += (sub- e).abs();
        }
        fuel_vec[i] = fuel;
    }
    fuel_vec.iter().min().unwrap().clone()
}

fn fuel_calculation_increase (input_subs : Vec<i64>) -> i64 {
    let max : usize =  *input_subs.iter().max().unwrap() as usize;
    let mut fuel_vector : Vec<i64> = vec![0; max];
    for (i, sub) in input_subs.iter().enumerate() {
        fuel_vector = fuel_vector.into_iter()
            .enumerate()
            .map(|(e, mut k)| {
                let dist = (sub - e as i64).abs();
                let single = dist*(dist + 1) /2;
                k+= single;
                k} ).collect();
    }
    fuel_vector.iter().min().unwrap().clone()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_path() {
        let inst = read_input_to_vector(PATH);
        assert_eq!(inst.is_empty(), false); 
    }

    #[test]
    fn fuel_for_alignment() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(fuel_calculation(subs), 37); 
    }

    #[test]
    fn fuel_for_alignment_increase() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(fuel_calculation_increase(subs),168); 
    }

    #[test]
    fn riddle_1() {
        let subs = read_input_to_vector(PATH);
        assert_eq!(fuel_calculation(subs), 343605); 
    }

    #[test]
    fn riddle_2() {
        let subs = read_input_to_vector(PATH);
        assert_eq!(fuel_calculation_increase(subs), 343605); 
    }
}