//6:55

use std::fs;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Command {
    command : String,
    motion : i32,
}

pub trait Calculate {
   fn calculate_motion (&mut self, command : &Command );
   fn get_motion_result (&self) -> i32;
}

pub struct Motion {
    depth : i32,
    forward : i32,
}

impl Motion {
    pub fn new () -> Self {
        Motion {
            depth: 0,
            forward: 0,
        }
    }
}

impl Calculate for Motion {
    fn calculate_motion (&mut self, command : &Command ) {
        match command.command.as_str() {
            "forward" => { self.forward += command.motion ;}
            "up" => {self.depth -= command.motion;}
            "down" => {self.depth += command.motion;}
            _ => {panic!("unknown command");}
        }
    }

    fn get_motion_result (&self) -> i32 {
        self.depth * self.forward
    }
}

pub struct AimMotion {
    depth : i32,
    forward : i32,
    aim: i32,
}

impl AimMotion {
    pub fn new () -> Self {
        AimMotion {
            depth: 0,
            forward: 0,
            aim: 0
        }
    }
}

impl Calculate for AimMotion {
    fn calculate_motion (&mut self, command : &Command ) {
        match command.command.as_str() {
            "forward" => { self.forward += command.motion;
                           self.depth += self.aim * command.motion; }
            "up" => {self.aim -= command.motion;}
            "down" => {self.aim += command.motion;}
            _ => {panic!("unknown command");}
        }
    }

    fn get_motion_result (&self) -> i32 {
        self.depth * self.forward
    }
}

static PATH : &str = "src/day2/input_day2.txt";

fn read_input_to_map ( input_path: &str ) -> Vec<Command> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
                input.lines().map(|l| { let t : Vec<&str> = l.split(' ').collect();
                                        Command{ command: t[0].to_owned(), motion : t[1].to_string().parse().unwrap()} } )
                             .collect()
        },
        Err (e) => {panic!("Could not parse input: {:?}", e);}
    }
}

fn calculate_motion <T: Calculate> ( input_map : Vec<Command> , mut calc : T ) -> i32 {
    let _v2: Vec<_> = input_map.iter().map(|c| calc.calculate_motion(c)).collect();
    calc.get_motion_result()
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

        assert_eq!(calculate_motion::<Motion>(in_map, Motion::new()), 150)
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

        assert_eq!(calculate_motion::<AimMotion>(in_map, AimMotion::new()), 900)
    }
    
    #[test]
    fn riddle_1() {
        let depths = read_input_to_map(PATH);
        assert_eq!(calculate_motion::<Motion>(depths, Motion::new()), 2036120);
    }
    
    #[test]
    fn riddle_2() {
        let depths = read_input_to_map(PATH);
        assert_eq!(calculate_motion::<AimMotion>(depths, AimMotion::new()), 2015547716);
    }
}