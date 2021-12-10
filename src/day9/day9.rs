// 19:26
use std::fs;
use std::env;
use std::path::Path;

use ndarray::prelude::*;
use ndarray::Array;

static PATH : &str = "src/day9/input_day9.txt";

fn read_input_to_vector ( input_path: &str, dim : (usize, usize) ) -> Array2::<i32,> {
    let mut a = Array2::<i32,>::zeros((dim.0, dim.1));
    let mut count = (0,0);
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            for l in input.lines() {
                for c in l.chars() {
                    a[[count.0, count.1]] = c.to_string().parse::<i32>().unwrap(); 
                    count.1 += 1;
                } 
                count.0 +=1;
                count.1 = 0;
            }
        }    
        Err (e) => panic!("Could not parse input: {:?}", e)
    } 
    a
}

fn find_lows( input : Array2::<i32,>) -> i32 {
    let shape = input.shape();
    let mut low_sum = 0;
    let mut up = 0;
    let mut down = 0;
    let mut right = 0;
    let mut left = 0;
    for ((r,c),value) in input.indexed_iter()  {
        if r == 0 {
            if c == 0 {
                right = input[[r,c+1]];
                down = input[[r+1,c]];
                if right > *value && down > *value {
                    low_sum += value + 1;
                }
            } else if c == shape[1] - 1  {
                left = input[[r,c-1]];
                down = input[[r+1,c]];
                if left > *value && down > *value {
                    low_sum += value + 1;
                }
            } else {
                right = input[[r,c+1]];
                left = input[[r,c-1]];
                down = input[[r+1,c]];
                if left > *value && down > *value && right > *value {
                    low_sum += value + 1;
                }    
            }
        } else if r == shape[0] - 1 {
            if c == 0 {
                right = input[[r,c+1]];
                up = input[[r-1,c]];
                if right > *value && up > *value {
                    low_sum += value + 1;
                }
            } else if c == shape[1] - 1 {
                left = input[[r,c-1]];
                up = input[[r-1,c]];
                if left > *value && up > *value {
                    low_sum += value + 1;
                }
            } else {
                right = input[[r,c+1]];
                left = input[[r,c-1]];
                up = input[[r-1,c]];
                if left > *value && up > *value && right > *value {
                    low_sum += value + 1;
                }    
            }
        } else {
            if c == 0 {
                right = input[[r,c+1]];
                down = input[[r+1,c]];
                up = input[[r-1,c]];
                if right > *value && down > *value && up > *value {
                    low_sum += value + 1;
                }
            } else if c == shape[1] - 1 {
                left = input[[r,c-1]];
                down = input[[r+1,c]];
                up = input[[r-1,c]];
                if left > *value && down > *value && up > *value {
                    low_sum += value + 1;
                }
            } else {
                right = input[[r,c+1]];
                left = input[[r,c-1]];
                down = input[[r+1,c]];
                up = input[[r-1,c]];
                if left > *value && down > *value && right > *value && up > *value {
                    low_sum += value + 1;
                }    
            }
        }
    }
    low_sum
}

fn find_low_vector( input : Array2::<i32,>) -> Vec<(usize,usize)> {
    let shape = input.shape();
    let mut low_vec = vec![];
    for ((r,c),value) in input.indexed_iter()  {
        if r == 0 {
            if c == 0 {
                if input[[r,c+1]] > *value && input[[r+1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else if c == shape[1] - 1  {
                if input[[r,c-1]] > *value && input[[r+1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else {
                if input[[r,c-1]] > *value && input[[r+1,c]] > *value && input[[r,c+1]] > *value {
                    low_vec.push((r,c));
                }    
            }
        } else if r == shape[0] - 1 {
            if c == 0 {
                if  input[[r,c+1]] > *value && input[[r-1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else if c == shape[1] - 1 {
                if input[[r,c-1]] > *value && input[[r-1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else {
                if input[[r,c-1]] > *value && input[[r-1,c]] > *value && input[[r,c+1]] > *value {
                    low_vec.push((r,c));
                }    
            }
        } else {
            if c == 0 {
                if input[[r,c+1]] > *value && input[[r+1,c]] > *value && input[[r-1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else if c == shape[1] - 1 {
                if input[[r,c-1]] > *value && input[[r+1,c]] > *value && input[[r-1,c]] > *value {
                    low_vec.push((r,c));
                }
            } else {
                if input[[r,c-1]] > *value && input[[r+1,c]] > *value && input[[r,c+1]] > *value && input[[r-1,c]] > *value {
                    low_vec.push((r,c));
                }    
            }
        }
    }
    low_vec
}

fn flood_recurse ( input : Array2::<i32,>, point : (i32,i32), preValue : i32, nextValue : i32, area : Vec<(usize,usize)> ) -> (Array2::<i32,>, Vec<(usize,usize)>) {
    let mut working = input;
    let mut area = area;
    let shape = working.shape();

    if point.0 < 0 || point.0 as usize >= shape[0] || point.1 < 0 || point.1 as usize >= shape[1]  {
        return (working, area);
    }
    let checkpoint = (point.0 as usize, point.1 as usize);
    if working[[checkpoint.0, checkpoint.1]] >= nextValue {
        return (working, area);
    }
    working[[checkpoint.0, checkpoint.1]] = nextValue;
    area.push((checkpoint.0,checkpoint.1));

    let (working, area) = flood_recurse( working, (point.0+1, point.1), preValue, nextValue, area);
    let (working, area) = flood_recurse( working, (point.0-1, point.1), preValue, nextValue, area);
    let (working, area) = flood_recurse( working, (point.0, point.1+1), preValue, nextValue, area);
    let (working, area) = flood_recurse( working, (point.0, point.1-1), preValue, nextValue, area);

    (working, area)
}
 
fn  floodFill( input : Array2::<i32,>, basin : Vec<(usize,usize)>, new_value : i32) -> usize
{
    let mut areas = vec![];
    for b in basin {
        let point = (b.0 as i32, b.1 as i32);
        let working = input.clone();
        let pre_value = working[[b.0,b.1]];
        let area = vec![];
        let (working, area) = flood_recurse(working, point, pre_value, new_value, area);
        areas.push(area.len());
    }
    areas.sort();
    areas.reverse();
    areas[2] * areas[1] * areas[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_path() {
        let inst = read_input_to_vector(PATH, (100, 100));
        assert_eq!(inst.is_empty(), false); 
    }

    #[test]
    fn read_path_test() {
        let inst = read_input_to_vector("src/day9/test_day9.txt", (5, 10));
        assert_eq!(inst.is_empty(), false); 
    }

    #[test]
    fn low_test() {
        let inst = read_input_to_vector("src/day9/test_day9.txt", (5, 10));
        assert_eq!(inst.is_empty(), false); 
        assert_eq!(find_lows(inst), 15);
    }

    #[test]
    fn low_vec_test() {
        let inst = read_input_to_vector("src/day9/test_day9.txt", (5, 10));
        assert_eq!(inst.is_empty(), false); 
        let basins = find_low_vector(inst.clone());
        assert_eq!(floodFill(inst, basins, 9), 1134 )
    }

    #[test]
    fn riddle_1() {
        let inst = read_input_to_vector(PATH, (100, 100));
        assert_eq!(inst.is_empty(), false); 
        assert_eq!(find_lows(inst), 558);
    }

    #[test]
    fn riddle_2() {
        let inst = read_input_to_vector(PATH, (100, 100));
        assert_eq!(inst.is_empty(), false); 
        let basins = find_low_vector(inst.clone());
        assert_eq!(floodFill(inst, basins, 9), 882942 )
    }
}