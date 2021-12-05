// 9:45
use std::fs;
use std::env;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use ndarray::prelude::*;
use ndarray::Array;

static PATH : &str = "src/day5/input_day5.txt";


#[derive(Debug)]
pub struct Instruction {
    x1 : usize,
    y1 : usize,
    x2 : usize,
    y2 : usize,
}

impl Instruction {
    pub fn new ( x1 : usize, y1: usize, x2 :usize, y2 : usize ) -> Self {
        Instruction {
            x1 : x1,
            y1 : y1,
            x2 : x2,
            y2 : y2
        }
    }

    pub fn is_straigth_line (&self) -> bool {
        (self.x1 == self.x2) || (self.y1 == self.y2)
    }

    pub fn is_row (&self) -> bool {
        self.y1 == self.y2
    }

    pub fn is_column(&self) -> bool {
        self.x1 == self.x2
    }
}

fn read_input_to_vector ( input_path: &str ) -> Vec<Instruction> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    lazy_static! { static ref RE: Regex = Regex::new(r"(\d*),(\d*).*-> *(\d*),(\d*)").unwrap(); };
    match fs::read_to_string(cpath) {
        Ok (input) => { input.lines().map( |l| {
                                                    let caps = RE.captures(l).unwrap();
                                                    Instruction::new(  caps.get(1).map_or("", |m| m.as_str()).parse().unwrap()
                                                                     , caps.get(2).map_or("", |m| m.as_str()).parse().unwrap()
                                                                     , caps.get(3).map_or("", |m| m.as_str()).parse().unwrap()
                                                                     , caps.get(4).map_or("", |m| m.as_str()).parse().unwrap() )

                                                }).collect()
                                            }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn count_vents_intersection ( inst : Vec<Instruction>, size : (usize, usize) ) -> i32 {
    let mut array = Array2::<i32,>::zeros((size.0, size.1));
    println!{"{:?}", array};
    let mut counter = 0;
    for i in inst {
        println!{"{:?}", i};
        if i.is_row() {
            let x1;
            let x2;
            if i.x1 > i.x2 { x1 = i.x2; x2 = i.x1 } else { x1 = i.x1; x2 = i.x2};
            array.slice_mut(s![i.y1, x1..x2+1 ]).map_inplace(|e| *e += 1);
            println!("inst row: {} - {} , {} - {}", x1, i.y1, x2+1, i.y2);
        }
        else if i.is_column()
        {
            let y1;
            let y2;
            if i.y1 > i.y2 { y1 = i.y2; y2 = i.y1 } else { y1 = i.y1; y2 = i.y2};
            array.slice_mut(s![y1..y2+1, i.x1 ]).map_inplace(|e| *e += 1);
            println!("inst col: {} - {} , {} - {}", i.x1, i.x2, y1, y2+1);
        }
    }
    for ((x,y),value) in array.indexed_iter_mut()  {
        if value > &mut 1 {
            counter += 1;
        }
    }
    counter
}

fn count_vents_intersection_diagonal ( inst : Vec<Instruction>, size : (usize, usize) ) -> i32 {
    let mut array = Array2::<i32,>::zeros((size.0, size.1));
    //println!{"{:?}", array};
    let mut counter = 0;
    for i in inst {
        println!{"{:?}", i};
        if i.is_row() {
            let x1;
            let x2;
            if i.x1 > i.x2 { x1 = i.x2; x2 = i.x1 } else { x1 = i.x1; x2 = i.x2};
            array.slice_mut(s![i.y1, x1..x2+1 ]).map_inplace(|e| *e += 1);
            println!("inst row: {} - {} , {} - {}", x1, i.y1, x2+1, i.y2);
        }
        else if i.is_column()
        {
            let y1;
            let y2;
            if i.y1 > i.y2 { y1 = i.y2; y2 = i.y1 } else { y1 = i.y1; y2 = i.y2};
            array.slice_mut(s![y1..y2+1, i.x1 ]).map_inplace(|e| *e += 1);
            println!("inst col: {} - {} , {} - {}", i.x1, i.x2, y1, y2+1);
        }
        else{
            let m = (i.x2 as i32 - i.x1 as i32) / (i.y1 as i32 - i.y2 as i32);
            //println!("m: {:?}", m);
            let x1;
            let x2;
            if i.x1 > i.x2 { x1 = i.x2; x2 = i.x1 } else { x1 = i.x1; x2 = i.x2};
            let y1;
            let y2;
            if i.y1 > i.y2 { y1 = i.y2; y2 = i.y1 } else { y1 = i.y1; y2 = i.y2};
            let ys = (x2 as i32 - x1 as i32).abs() as i32;
            if m > 0 {
                //println!("m+ inst: {:?}", i);
                //println!("x1: {}, x2 : {} - y1: {} - y2: {}", x1, x2+1, y1, y2+1);
                let mut slice = array.slice_mut(s![y1..y2+1, x1..x2+1]);
                //println!("slice: {:?}", slice);
                for ((x,y),value) in slice.indexed_iter_mut()  {
                    //println!("con: {} - {} ", y as i32- ys , x);
                    if (y as i32 - ys).abs() == x as i32 {
                        *value += 1;
                        //println!("mark inst: {:?} - x: {} - y: {}", i, x, y);
                    } 
                }
            } else {
                //println!("m+ inst: {:?}", i);
                //println!("x1: {}, x2 : {} - y1: {} - y2: {}", x1, x2, y1, y2);
                let mut slice = array.slice_mut(s![y1..y2+1, x1..x2+1 ]);
                //println!("slice: {:?}", slice);
                for ((x,y),value) in slice.indexed_iter_mut()  {
                    //println!("con: {} - {} ", y as i32- ys , x);
                    if (y as i32).abs() == x as i32 {
                        *value += 1;
                        //println!("mark inst: {:?} - x: {} - y: {}", i, x, y);
                    } 
                }
            } 
            //println!("inst: {} - {} , {} - {} -> \n {:?}", i.x1, i.y1, i.x2, i.y2, slice);
        }
        println!{"\n{:?}\n", array};
        //println!("inst: {} - {} , {} - {} ", i.x1, i.y1, i.x2, i.y2  );
        
    }
    println!("\n{:?}\n",array);
    for ((x,y),value) in array.indexed_iter_mut()  {
        if value > &mut 1 {
            counter += 1;
        }
    }
    counter
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
    fn read_test_path() {
        let inst = read_input_to_vector("src/day5/test_day5.txt");
        assert_eq!(inst.is_empty(), false); 
        assert_eq!(inst.len(), 10);
    }

    #[test]
    fn count_intersections_test () {
        let inst = read_input_to_vector("src/day5/test_day5.txt");
        assert_eq!(count_vents_intersection(inst, (10,10)), 5)
    }

    #[test]
    fn count_intersections_diagonal_test () {
        let inst = read_input_to_vector("src/day5/test_day5.txt");
        assert_eq!(count_vents_intersection_diagonal(inst, (10,10)), 12)
    }

    #[test]
    fn riddle_1 () {
        let inst = read_input_to_vector("src/day5/input_day5.txt");
        assert_eq!(count_vents_intersection(inst, (999,999)), 5774)
    }

    #[test]
    fn riddle_2 () {
        let inst = read_input_to_vector("src/day5/input_day5.txt");
        assert_eq!(count_vents_intersection_diagonal(inst, (999,999)), 18423)
    }
}