// 6:41
use std::fs;
use std::env;
use std::path::Path;

static PATH : &str = "src/day6/input_day6.txt";

fn read_input_to_vector ( input_path: &str ) -> Vec<u64> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { input.split(',').filter_map(|s| { s.parse::<u64>().ok()} ).collect() }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn fish_counter (mut fish : Vec<u64>, days: u64) -> usize {

    for day in 0..days {
        let mut counter = 0;
        for f in fish.iter_mut() {
            match f {
                0 => { counter += 1; *f = 6  },
                _ => {  *f -= 1}
            }
        }
        for i in 0..counter {
            fish.push(8);
        }
    }
    fish.len()
}

fn fish_growth_model (mut fish : Vec<u64>, days: u64 ) -> u64 {
    let mut counter = 0;
    let mut spawns = vec![0;days as usize];
    for f in fish {
        counter += 1 + calculate_new_fishs(f,days as i64, &mut spawns);
    }
    counter
}

fn calculate_new_fishs ( fish_timer: u64, remaining_days: i64, spawn_check : &mut Vec<u64> ) -> u64 {

    println!("fish: {:?} - days: {:?}", fish_timer, remaining_days);
    let mut spawn = remaining_days as i64 - fish_timer as i64;
    let mut family = 0;
    while spawn >= 1 {
        let sp : usize = spawn as usize;
        if spawn_check[sp] != 0 {
             family += spawn_check[sp]
        } else {
            spawn_check[sp] = 1 + calculate_new_fishs(8, spawn - 1, spawn_check);
            family += spawn_check[sp];
        }
        spawn -= 7;
    }
    family
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
    fn count_fish() {
        let fish = vec![3,4,3,1,2];
        assert_eq!(fish_counter(fish.clone(), 18), 26);
        assert_eq!(fish_counter(fish, 80), 5934);
    }

    #[test]
    fn riddle_1() {
        let fish = read_input_to_vector(PATH);
        assert_eq!(fish_counter(fish, 80), 379114);
    }

    
    #[test]
    fn count_fish_long() {
        let fish = vec![3,4,3,1,2];
        assert_eq!(fish_growth_model(fish, 256), 26984457539);
    }

    #[test]
    fn riddle_2() {
        let fish = read_input_to_vector(PATH);
        assert_eq!(fish_growth_model(fish, 256), 26984457539);
    }

}