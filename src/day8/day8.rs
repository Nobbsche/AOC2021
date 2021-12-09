// 6:27
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;
use std::path::Path;

use ndarray::prelude::*;
use ndarray::Array;

use lazy_static::lazy_static;
use regex::Regex;

static PATH : &str = "src/day8/input_day8.txt";

#[derive(Debug, Clone)]
pub struct Segment {
    input : Vec<String>,
    output : Vec<String>
}

fn read_input_to_vector ( input_path: &str ) -> Vec<Segment> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    lazy_static! { static ref RE: Regex = Regex::new(r"(\b\S*)").unwrap(); };
    match fs::read_to_string(cpath) {
        Ok (input) => { input
                .lines()
                .map( |l| { let vl : Vec<Vec<String>> = l
                    .split('|')
                    .map(|io| io.split(' ').filter_map(|i| if !i.is_empty() { Some (String::from(i)) } else { None } )
                        .collect() )
                    .collect();
                    Segment{ input : vl[0].clone(), output : vl[1].clone()} }
                ).collect() 
        },                
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn count_numbers ( input :  Vec<Segment> ) -> u32 {
    let counter : usize = input.iter().map( |seg| seg.output.iter()
        .filter_map ( |e| match  e.chars().count() {
                    2 | 3 | 4 | 7 => Some (e),
                    _ => None }
        ).count()
        ).sum();
    counter as u32
}

pub fn find_segements( input : &Segment) -> Vec<char> {
    let mut result : Vec<char>= vec![' ';7];
    let put = input.input.clone();
    let one = put.iter().filter(|x| x.len() == 2 ).collect::<Vec<&String>>()[0].chars();
    let four = put.iter().filter(|x| x.len() == 4 ).collect::<Vec<&String>>()[0].chars();
    let seven = put.iter().filter(|x| x.len() == 3 ).collect::<Vec<&String>>()[0].chars();
    let eight = put.iter().filter(|x| x.len() == 7 ).collect::<Vec<&String>>()[0].chars();

    let s : HashSet<char> = seven.clone().into_iter().collect();
    let u : HashSet<char> = four.clone().into_iter().collect();

    result[0] = *s.difference(&u).collect::<Vec<&char>>()[0];

    let zero  = put.iter().filter(|e| e.len() == 6)
                         .filter(|e| { four.clone().filter(|c| { 
                                        e.chars().filter(|e| {
                                            e == c || (e == &one.clone().nth(0).unwrap() || e == &one.clone().nth(1).unwrap()) } ).count() <= 2 } )
                                        .count() == 3 })
                         .collect::<Vec<&String>>()[0].chars();
    
    result[3] = eight.clone().filter (|e| { let k : Vec<char> = zero.clone().filter(|z| z == e ).collect(); k.is_empty() } ).collect::<Vec<char>>()[0];
    for p in put.iter() {
        if p.len() != 6 { continue; } 
        let k = p.chars();
        let t : Vec<char> = one.clone().filter( |e| { let k = k.clone().filter(|z| z == e ).count(); k == 0 }  ).collect();
        if !t.is_empty() {
            result[2] = t[0];
        }        
    } 
    result[5] = one.clone().filter(|e| e != &result[2] ).collect::<Vec<char>>()[0];

    for p in put.iter() { 
        if p.len() != 5 { continue; } 
        let f : HashSet<char> = result.clone().into_iter().filter(|&s| s != ' ').collect(); 
        let e : HashSet<char> = p.chars().into_iter().collect();
        let t = e.difference(&f).collect::<Vec<&char>>();
        if t.len() == 1 {
            result[6] = *t[0];
            break;
        }
        println!("t: {:?}", t);
    } 
    for p in put.iter() { 
        if p.len() != 5 { continue; } 
        let f : HashSet<char> = result.clone().into_iter().filter(|&s| s != ' ').collect(); 
        let v : HashSet<char> = four.clone().into_iter().collect();
        let e : HashSet<char> = p.chars().into_iter().collect();
        let u : HashSet<char> = f.union(&v).collect::<Vec<&char>>().into_iter().map(|m| *m).collect();
        let two = e.difference(&u).collect::<Vec<&char>>();
        if two.len() == 1 {
            result[4] = *two[0];
            break;
        }
    }

    let f : HashSet<char> = result.clone().into_iter().filter(|&s| s != ' ').collect();
    let e : HashSet<char> = eight.clone().into_iter().collect();
    let two = e.difference(&f).collect::<Vec<&char>>();
    result[1] = *two[0];
    //println!("-> result: {:?}", result);
    result
}

pub fn generate_output (input : &Segment, filter : Vec<char> ) -> u32 {
    let mut m : HashMap<u32, Vec<char>> = HashMap::new();
    m.insert(0, vec![filter[0],filter[1],filter[2],filter[4],filter[5],filter[6]]);
    m.insert(1, vec![filter[2],filter[5]]);
    m.insert(2, vec![filter[0],filter[2],filter[3],filter[4],filter[6]]);
    m.insert(3, vec![filter[0],filter[2],filter[3],filter[5],filter[6],]);
    m.insert(4, vec![filter[1],filter[2],filter[3],filter[5]]);
    m.insert(5, vec![filter[0],filter[1],filter[3],filter[5],filter[6]]);
    m.insert(6, vec![filter[0],filter[1],filter[3],filter[4],filter[5],filter[6]]);
    m.insert(7, vec![filter[0],filter[2],filter[5]]);
    m.insert(8, vec![filter[0],filter[1],filter[2],filter[3],filter[4],filter[5],filter[6]]);
    m.insert(9, vec![filter[0],filter[1],filter[2],filter[3],filter[5],filter[6]]);

    let mut numbers = vec![];
    
    for o in input.output.iter() {
        let os : HashSet<char> = o.chars().into_iter().collect();
        for (k, v) in m.iter() {
            let e : HashSet<char> = v.clone().into_iter().collect();
            let diff = e.symmetric_difference(&os).collect::<Vec<&char>>();
            //println!("{:?} - f : {:?} - diff: {:?}", k, e ,diff);
            if diff.len() == 0 {
                numbers.push(k);
                break;
            }
        }
        //println!("input: {:?} - numbers: {:?}", o, numbers);
    }
    let n = numbers[0] *1000 + numbers[1] *100 + numbers[2] * 10 + numbers[3];
    //println!("number: {:?}", n);
    n
}

pub fn run_all_segments ( input: Vec<Segment> ) -> u32 {
    let mut sum = 0;
    for segment in input {
        let sol = find_segements(&segment);
        println!("segment: {:?} - sol: {:?} - sum: {:?}", segment, sol, sum);
        sum += generate_output(&segment, sol);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test_path() {
        let input = read_input_to_vector("src/day8/test_day8.txt");
        assert_eq!(input.is_empty(), false);
        assert_eq!(input.len(), 10); 
    }

    #[test]
    fn read_path() {
        let input = read_input_to_vector(PATH);
        assert_eq!(input.is_empty(), false);
        assert_eq!(input.len(), 200); 
    }

    #[test]
    fn count_number_test() {
        let input = read_input_to_vector("src/day8/test_day8.txt");
        assert_eq!(input.is_empty(), false);
        assert_eq!(input.len(), 10); 
        assert_eq!(count_numbers(input), 26);
    }

    #[test]
    fn find_segments_test() {
        let input = Segment{input : vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa")
                                        , String::from("fbcad"), String::from("dab"), String::from("cefabd")
                                        , String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"),
                                        String::from("ab")],
                            output : vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
             };
        assert_eq!(find_segements(&input), vec!['d','e','a','f','g','b','c']);
    }

    #[test]
    fn find_segment2() {
        let input = read_input_to_vector("src/day8/test_day8.txt");
        assert_eq!(find_segements(&input[0]), vec!['d','g','b','c','a','e','f']);
    }

    #[test]
    fn generate_number_from_result_test() {
        let input = Segment{input : vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa")
                                        , String::from("fbcad"), String::from("dab"), String::from("cefabd")
                                        , String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"),
                                        String::from("ab")],
                            output : vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
                        };
        let sol = find_segements(&input);
        assert_eq!(generate_output(&input, sol) ,5353);
    }

    #[test]
    fn run_all_segments_test() {
        let input = read_input_to_vector("src/day8/test_day8.txt");
        assert_eq!(run_all_segments(input),61229);
    }

    #[test]
    fn riddle_1() {
        let input = read_input_to_vector(PATH);
        assert_eq!(input.is_empty(), false);
        assert_eq!(input.len(), 200); 
        assert_eq!(count_numbers(input), 534);
    }

    #[test]
    fn riddle_2() {
        let input = read_input_to_vector(PATH);
        assert_eq!(run_all_segments(input),1070188);
    }
}