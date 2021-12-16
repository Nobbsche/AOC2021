//14.12.2021 18:58
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::HashMap;

use std::fs;
use std::env;
use std::path::Path;

use itertools::Itertools;

use lazy_static::lazy_static;
use regex::Regex;

fn read_input_rules ( input_path: &str ) -> HashMap<(char,char), char> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    let mut polymer_map = HashMap::new();
    lazy_static! { static ref RE: Regex = Regex::new(r"(\S)(\S).*(\S)").unwrap(); };
    match fs::read_to_string(cpath) {
        Ok (input) => { input.lines().foreach( |l| {    let caps = RE.captures(l).unwrap();
                                                    polymer_map.insert(
                                                        ( caps.get(1).map_or('0', |m| m.as_str().chars().collect::<Vec<char>>()[0] )
                                                        , caps.get(2).map_or('0', |m| m.as_str().chars().collect::<Vec<char>>()[0] ) )
                                                        , caps.get(3).map_or('0', |m| m.as_str().chars().collect::<Vec<char>>()[0] )
                                                    );
                                                });
                                            }
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
    polymer_map
}

fn polymer_next_generation( polymer : &str, rules : HashMap<(char,char),char>, steps : u32) -> u64 {

    let mut statistics = init_statistics(polymer);
    let mut producer = init_producer(polymer, &rules);
    let mut step = 1;
    while step <= steps {
      do_produce(&mut producer, &mut statistics, &rules);
      step += 1;
    }
    
    *statistics.values().max().unwrap() as u64 - *statistics.values().min().unwrap() as u64
}

fn do_produce( producer: &mut HashMap<(char, char), u64>, statistics: &mut HashMap<char, u64>, rules: &HashMap<(char,char), char> ) {
    let mut new_pairs = HashMap::<(char, char), u64>::new();
    for (pair, pair_count) in producer.iter_mut() {
      let produced_char = rules.get(pair).unwrap();
      update_statistics(statistics, produced_char, pair_count);
      let (pair_left, pair_right) = produce_pairs(pair, produced_char);
      if rules.contains_key(&pair_left) {
        add_pair_to_map(&mut new_pairs, &pair_left, pair_count);
      }
      if rules.contains_key(&pair_right) {
        add_pair_to_map(&mut new_pairs, &pair_right, pair_count);
      }
      *pair_count = 0;
    }
    for (pair, pair_count) in new_pairs.iter() {
      add_pair_to_map(producer, pair, pair_count);
    }
  }
  
  fn add_pair_to_map(pairs: &mut HashMap<(char, char), u64>, pair: &(char, char), count: &u64) {
    let new_count = match pairs.get(pair) {
      Some(i) => i + count,
      None => *count,
    };
    pairs.insert(*pair, new_count);
  }
  
  fn produce_pairs(pair: &(char, char), produced_char: &char) -> ((char, char), (char, char)) {
    ( (pair.0, *produced_char), (*produced_char, pair.1))
  }
  
  fn update_statistics(statistics: &mut HashMap<char, u64>, produced_char: &char, count: &u64) {
    let counter = statistics.entry(*produced_char).or_insert(*count);
    *counter += count;
  }
  
  fn init_statistics(initial: &str) -> HashMap<char, u64> {
    let mut res = HashMap::new();
    let chars = initial.chars().foreach(|c| { 
            let counter = res.entry(c).or_insert(1); 
            *counter += 1; 
        });
    res
  }
  
  fn init_producer(initial: &str, rules: &HashMap<(char,char), char>) -> HashMap<(char,char), u64> {
    let mut res = HashMap::new();
    let chars = initial.chars().collect::<Vec<char>>();
    chars.windows(2).foreach(|c| {
        let pair = ( c[0] ,c[1]);
        let counter = res.entry(pair).or_insert(0);
        *counter += 1;
    });
    res
  }


#[cfg(test)]
mod tests {
    use super::*;

    static TEMPLATE : &str = "CNBPHFBOPCSPKOFNHVKV";
    static TTEMPLATE : &str = "NNCB";

    static RULES : &str = "src/day14/input_day14.txt";
    static TRULES : &str = "src/day14/test_day14.txt";

    #[test]
    fn read_path() {
        let rules = read_input_rules(RULES);
        assert_eq!(rules.is_empty(), false); 
    }

    #[test]
    fn test_simple_polymer() {
        let rules = read_input_rules(TRULES);
        assert_eq!(polymer_next_generation( TTEMPLATE, rules, 10), 1588);
    }

    #[test]
    fn riddle_1() {
        let rules = read_input_rules(RULES);
        assert_eq!(polymer_next_generation( TEMPLATE, rules, 10), 2170);
    }

    #[test]
    fn riddle_2() {
        let rules = read_input_rules(RULES);
        assert_eq!(polymer_next_generation(TEMPLATE, rules, 40), 2422444761283);
    }
} 
