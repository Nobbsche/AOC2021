// 7:17
use std::fs;
use std::env;
use std::path::Path;

static PATH : &str = "src/day3/input_day3.txt";

fn read_input_to_vector ( input_path: &str ) -> Vec<Vec<u32>> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { input.lines().map( |l| l.chars().map(|d| d.to_digit(10).unwrap() ).collect() ).collect()}
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

pub fn gamma_rate ( bits : Vec<Vec<u32>> ) -> Vec<u32> {
    let tbits = transpose_matrix(bits);
    let r = most_common_bit(&tbits, 0);
    r
}

pub fn most_common_bit ( bits : &Vec<Vec<u32>>, predict : u32) -> Vec<u32> {
    bits.iter().map(|c| { 
        let sum = c.iter().sum::<u32>();
        if sum >= c.len() as u32 - (c.len() as u32 /2) {
                1
        } else {
                0
        }
    }  ).collect()
}

pub fn filter_oxygen ( bits : Vec<Vec<u32>>) -> u32 {
    let tbits = transpose_matrix(bits.clone());
    let mut filter = bits.clone();
    for i in 0..bits[0].len() {
        let mc = most_common_bit(&transpose_matrix(filter.clone()), 1);
        filter = filter.into_iter().filter(|s| s[i] == mc[i] ).collect();
        if filter.len() == 1 {
            break;
        }
    }
    tranform_bin_to_num(filter[0].clone())
}

pub fn filter_co2scrubber ( bits : Vec<Vec<u32>>) -> u32 {
    let tbits = transpose_matrix(bits.clone());
    let mut filter = bits.clone();
    for i in 0..bits[0].len() {
        let mc = most_common_bit(&transpose_matrix(filter.clone()), 0);
        let inv_mc : Vec<u32> = mc.iter().map(|&bit| {if bit == 0 { 1} else { 0 }} ).collect();
        filter = filter.into_iter().filter(|s| s[i] == inv_mc[i] ).collect();
        if filter.len() == 1 {
            break;
        }
    }
    tranform_bin_to_num(filter[0].clone())
}

pub fn epsilon_rate ( num : Vec<u32> ) -> u32 {
   let x = num.iter().map(|&bit| {if bit == 0 { 1} else { 0 }} ).collect();
   tranform_bin_to_num(x)
}

fn tranform_bin_to_num ( bits : Vec<u32> ) -> u32 {
    let length = bits.len();
    let mut num = 0;
    for i in 0..length {
        num += bits[i]*(2_i32.pow(length as u32 -i as u32-1) as u32);
    }
    num
}

fn transpose_matrix ( bits : Vec<Vec<u32>> ) -> Vec<Vec<u32>> {
    let mut t = vec![Vec::with_capacity(bits.len()); bits[0].len()];
    for r in bits {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn epsilon_gama_rate ( bits : Vec<Vec<u32>>) -> u32 {
    let gamma = gamma_rate(bits);
    let epsilon = epsilon_rate(gamma.clone());
    tranform_bin_to_num(gamma) * epsilon
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
    fn scan_bits() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        println!("test: {:?}", bits);
        assert_eq!(epsilon_gama_rate(bits), 198);
    }

    #[test]
    fn gamma_rate_test() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        assert_eq!( tranform_bin_to_num(gamma_rate(bits)), 22);
    }

    #[test]
    fn epsilon_rate_test() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        let gamma = gamma_rate(bits);
        assert_eq!( epsilon_rate(gamma),9);
    }

    #[test]
    fn filter_oxy_test() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        assert_eq!( filter_oxygen(bits), 23);
    }

    #[test]
    fn filter_scrubber_test() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        assert_eq!( filter_co2scrubber(bits), 10);
    }

    #[test]
    fn life_support() {
        let bits = read_input_to_vector("src/day3/test_day3.txt");
        let oxy = filter_oxygen(bits.clone());
        let co2 = filter_co2scrubber(bits);
        assert_eq!( oxy*co2, 230);
    }

    #[test]
    fn riddle_1() {
        let bits = read_input_to_vector("src/day3/input_day3.txt");
        assert_eq!(epsilon_gama_rate(bits), 4006064);
    }

    #[test]
    fn riddle_2() {
        let bits = read_input_to_vector("src/day3/input_day3.txt");
        let oxy = filter_oxygen(bits.clone());
        let co2 = filter_co2scrubber(bits);
        assert_eq!( oxy*co2, 5941884);
    }
}