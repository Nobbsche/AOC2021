// 19:10
use std::fs;
use std::env;
use std::path::Path;

use ndarray::prelude::*;
use ndarray::Array;


fn read_input_to_array ( input_path: &str, dim : (usize, usize) ) -> Array2::<i32,> {
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

pub struct Octopussy {
    octopus : Array2::<i32,>,
    flash_counter : i64,
    shape : Vec<usize>,
    flashed : Array2::<u32,>,
}

impl Octopussy {
    
    pub fn new ( octos : Array2::<i32,> ) -> Self {
        let shape = octos.shape().to_vec();
        let x = shape[0].clone();
        let y = shape[1].clone();
        Octopussy{ octopus : octos , flash_counter : 0, shape : shape, flashed : Array2::<u32,>::zeros((x,y)) } 
    }

    pub fn get_flashers (&self) -> u32 {
        let mut flash = 0;
        for ( (r,c), value) in self.octopus.indexed_iter() {
            if *value != 0 { continue; }
            flash += 1;
        }
        flash
    }

    fn change_value(&mut self, pos : (usize,usize) ) {
        self.octopus[[pos.0, pos.1]] += 1;
    } 

    fn reset_flashed (&mut self) {
        let x = self.shape[0].clone();
        let y = self.shape[0].clone();
        self.flashed = Array2::<u32,>::zeros((x,y));
    }
}

pub fn flash_octo (pos : (i32, i32), mut octos : Octopussy,  ) -> Octopussy {
    if pos.0 < 0 || pos.0 as usize >= octos.shape[0] || pos.1 < 0 || pos.1 as usize >= octos.shape[1]  {
       return octos;
    }
    let checkpoint = (pos.0 as usize, pos.1 as usize);
    //println!("pos: {:?} - value: {:?} - flashed: {:?}", pos, octos.octopus[[checkpoint.0, checkpoint.1]], octos.flashed[[checkpoint.0, checkpoint.1]]);
    if octos.flashed[[checkpoint.0, checkpoint.1]] == 1 {
        return octos;
    }
    if octos.octopus[[checkpoint.0, checkpoint.1]] < 9 {
        octos.octopus[[pos.0 as usize, pos.1 as usize]] += 1;
        return octos;
    }

    octos.octopus[[pos.0 as usize, pos.1 as usize]] = 0;
    octos.flashed[[pos.0 as usize, pos.1 as usize]] = 1;
    octos.flash_counter += 1;

    //println!("new => pos: {:?} - value: {:?} - flashed: {:?}", pos, octos.octopus[[checkpoint.0, checkpoint.1]], octos.flashed[[checkpoint.0, checkpoint.1]]);
    octos = flash_octo( (pos.0+1, pos.1-1), octos);
    octos = flash_octo( (pos.0+1, pos.1), octos);
    octos = flash_octo( (pos.0+1, pos.1+1), octos);
    octos = flash_octo( (pos.0, pos.1-1), octos);
    octos = flash_octo( (pos.0, pos.1+1), octos);
    octos = flash_octo( (pos.0-1, pos.1-1), octos);
    octos = flash_octo( (pos.0-1, pos.1), octos);
    octos = flash_octo( (pos.0-1, pos.1+1), octos);

    //println!("\n{:?}\n",octos.octopus);
    octos
}

pub fn do_step ( mut o : Octopussy ) -> Octopussy {
    for ((r,c), val) in o.octopus.clone().indexed_iter() {
        //println!("pos: {:?}-{:?} - value: {:?}", r,c, o.octopus[[r,c]]);
        if o.octopus[[r,c]] < 9 && o.flashed[[r,c]] == 0 {
            o.change_value((r,c));
        } else {
            o = flash_octo((r as i32,c as i32), o);  
        } 
    }
    println!("step:");
    println!("\n{:?}\n",o.octopus);
    o
}

fn do_steps( mut o : Octopussy, steps : u32 ) -> i64 {
    for i in 0..steps {
        o = do_step(o);
        o.reset_flashed();
    }
    o.flash_counter
}

fn run_sim( mut o : Octopussy, max_steps : i64 ) -> i64 {
    let mut step_counter = 0;
    while step_counter < max_steps {
        step_counter += 1;
        o = do_step(o);
        o.reset_flashed();
        if o.get_flashers() == 100 {
            break;
        }
    }
    step_counter
}


#[cfg(test)]
mod tests {
    static PATH : &str = "src/day11/input_day11.txt";
    static TESTPATH : &str = "src/day11/test_day11.txt";
    static TESTPATH2 : &str = "src/day11/test2_day11.txt";

    use super::*;

    #[test]
    fn read_path() {
        let inst = read_input_to_array(PATH, (10,10));
        assert_eq!(inst.is_empty(), false); 
    }

    #[test]
    fn read_test_path() {
        let inst = read_input_to_array(TESTPATH2, (5,5));
        assert_eq!(inst.is_empty(), false);
        assert_eq!(inst.shape(), vec![5,5]);
    }

    #[test]
    fn find_number9_test() {
        let inst = read_input_to_array(TESTPATH2, (5,5));
        assert_eq!(inst.is_empty(), false);
        let mut octos = Octopussy::new(inst);
        octos = do_step(octos);
        octos.reset_flashed();
        octos = do_step(octos);
        assert_eq!(octos.octopus[[2,2]], 1);
    }

    #[test]
    fn do_steps_test() {
        let inst = read_input_to_array(TESTPATH, (10,10));
        assert_eq!(inst.is_empty(), false);
        let mut octos = Octopussy::new(inst.clone());
        assert_eq!(do_steps(octos, 10), 204);

        let mut octos = Octopussy::new(inst);
        assert_eq!(do_steps(octos, 100), 1656);
    }

    #[test]
    fn step_counting() {
        let inst = read_input_to_array(TESTPATH, (10,10));
        assert_eq!(inst.is_empty(), false);

        let mut octos = Octopussy::new(inst);
        assert_eq!(run_sim(octos, 200), 195);
    }
    
    #[test]
    fn riddle_1() {
        let inst = read_input_to_array(PATH, (10,10));
        assert_eq!(inst.is_empty(), false);

        let mut octos = Octopussy::new(inst);
        assert_eq!(do_steps(octos, 100), 1637);
    }

    #[test]
    fn riddle_2() {
        let inst = read_input_to_array(PATH, (10,10));
        assert_eq!(inst.is_empty(), false);

        let mut octos = Octopussy::new(inst);
        assert_eq!(run_sim(octos, 1000), 242);
    }

}