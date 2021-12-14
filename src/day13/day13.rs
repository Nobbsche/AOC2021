// 14.12.2021 08:02
use std::io::BufWriter;
use std::fs::File;
use std::io::prelude::*;

use std::fs;
use std::env;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use ndarray::prelude::*;
use ndarray::Array;

fn read_input_image ( input_path: &str ) -> Array2::<i32,> {

    let mut coords : Vec<(usize, usize)> = vec![];
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            for l in input.lines() { 
                    let c : Vec<usize> = l.split(',').map(|e| e.to_string().parse::<usize>().unwrap() ).collect();
                    coords.push((c[0],c[1]));
                }  
        },    
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
    
    let x_max = coords.clone().into_iter().map(|(v,_)|v).fold(0, std::cmp::max);
    let y_max = coords.clone().into_iter().map(|(_,v)|v).fold(0, std::cmp::max);
    println!("x: {} - y: {}", x_max, y_max);
    let mut a = Array2::<i32,>::zeros((y_max+1, x_max+1));
    for coord in coords.iter() {
        a[[coord.1, coord.0]] = 1;
    } 
    a
}

fn read_fold_instructions ( input_path: &str ) -> Vec<(char, usize)> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    lazy_static! { static ref RE: Regex = Regex::new(r"fold along ([xy])=(\d*)").unwrap(); };
    match fs::read_to_string(cpath) {
        Ok (input) => { input.lines().map( |l| { let caps = RE.captures(l).unwrap();
                                                ( caps.get(1).map_or("", |m| m.as_str()).chars().collect::<Vec<char>>()[0], caps.get(2).map_or("", |m| m.as_str()).parse().unwrap())
                                                }).collect()
                                            }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn fold_image ( mut array : Array2::<i32,>, fold : (char, usize)) -> Array2::<i32,> {
    let mut axis = 0;
    if fold.0 == 'y' {
        axis = 0;
    }
    if fold.0 == 'x' {
        axis = 1;
    }

    let (mut splitArray1, mut splitArray2) = array.view_mut().split_at(Axis(axis), fold.1);
    let (_, mut splitArray2) = splitArray2.view_mut().split_at(Axis(axis),1);

    splitArray2.invert_axis(Axis(axis));
    println!("start: \n \n{:?}\n - \n{:?}\n", splitArray1, splitArray2);

    let shape = splitArray2.shape();
    let flat = Array::from_iter(splitArray2.iter());
    let mut counter_x = 0;
    let mut counter_y = 0;
    println!("shape: {:?}", shape );
    for e in flat.iter() {
        if counter_x == shape[1] {
            counter_x = 0;
            counter_y += 1;
        }
        splitArray1[[counter_y, counter_x]] += **e;
        if splitArray1[[counter_y, counter_x]] > 1 {
            splitArray1[[counter_y, counter_x]] = 1;
        }
        counter_x += 1;
    }
    println!("flat: {:?}", flat);
    println!("result: \n \n{:?}\n", splitArray1);
    let result = (&splitArray1.to_owned()).clone(); //+ &splitArray2;
    println!("whites: {:?}", result.sum());
    result
}

fn fold_all_and_write_result ( mut array : Array2::<i32,>, folds : Vec<(char, usize)> ) -> u32 {
    let mut imag= array;
    for fold in folds {
        imag = fold_image(imag, fold);
    }

    write_to_file (imag.clone());

    imag.sum().try_into().unwrap()
}

pub fn write_to_file ( array : Array2::<i32,> ) {
    let cpath = env::current_dir().unwrap().join(Path::new("src/day13/output_image_day13.txt"));
    let mut output = File::create(cpath).unwrap();
    for row in array.rows() {
        let s : String = row.iter().map(|e| {
            match e {
                0 => '.',
                1 => '#',
                _ => ' '
            }
        }).collect::<Vec<char>>().into_iter().collect();
        writeln!(output, "{:?}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static IPATH : &str = "src/day13/input_image_day13.txt";
    static FPATH : &str = "src/day13/input_fold_day13.txt";

    static ITESTPATH : &str = "src/day13/test_image_day13.txt";
    static FTESTPATH : &str = "src/day13/test_fold_day13.txt";

    #[test]
    fn read_path() {
        let inst = read_input_image(IPATH);
        assert_eq!(inst.is_empty(), false); 
    }

    #[test]
    fn read_path_fold() {
        let inst = read_fold_instructions(FPATH);
        assert_eq!(inst.is_empty(), false); 
        assert_eq!(inst.len(), 12);
    }

    #[test]
    fn fold_one_time() {
        let imag = read_input_image(ITESTPATH);
        let fold = read_fold_instructions(FTESTPATH);
        assert_eq!(fold_image(imag, fold[0]).sum(), 17);    
    }

    #[test]
    fn fold_all_and_write() {
        let imag = read_input_image(ITESTPATH);
        let fold = read_fold_instructions(FTESTPATH);
        assert_eq!( fold_all_and_write_result(imag, fold), 16);  
    }

    #[test]
    fn riddle_1() {
        let imag = read_input_image(IPATH);
        let fold = read_fold_instructions(FPATH);
        assert_eq!(fold_image(imag, fold[0]).sum(), 818);
    }

    #[test]
    fn riddle_2() {
        let imag = read_input_image(IPATH);
        let fold = read_fold_instructions(FPATH);
        assert_eq!( fold_all_and_write_result(imag, fold), 101);  
    }
}