// 16:36
use std::fs;
use std::env;
use std::path::Path;

fn read_input ( input_path: &str ) -> Vec<String> {
    let mut st = vec![];
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            for l in input.lines() {
                st.push(l.to_string());
            }
        }    
        Err (e) => panic!("Could not parse input: {:?}", e)
    } 
    st
}

fn find_closing_index (pos : usize, s : String ) -> usize {
    let cv : Vec<char> = s.chars().collect();
    let mut close_pos = 0;
    let mut stack = vec![];
    if cv[pos] != '(' && cv[pos] != '[' && cv[pos] != '{' && cv[pos] != '<' {
        return 0;
    } 
    for i in pos..cv.len() {
        if cv[i] == '(' || cv[i] == '[' || cv[i] == '{' || cv[i] == '<' {
            stack.push(cv[i]);
        } else if cv[i] == ')' || cv[i] == ']' || cv[i] == '}' || cv[i] == '>' {
            stack.pop();
            if stack.is_empty() {
                return i;
            }
        }
    }
    0
}   

fn is_corrupt_line ( open : usize, closing : usize , s : String ) -> bool {
    let cv = s.chars().collect::<Vec<char>>();
    match cv[open] {
        '(' => cv[closing] != ')',
        '{' => cv[closing] != '}',
        '[' => cv[closing] != ']',
        '<' => cv[closing] != '>',
        _ => false,

    }
}

fn calculate_penality ( line : String ) -> u64 {
    let cv = line.chars().collect::<Vec<char>>();
    let mut penality = 0;
    for (i,c) in cv.iter().enumerate() {
        let closing_index = find_closing_index( i, line.clone() );
        //rintln!("closing_index: {:?}", closing_index); 
        if closing_index == 0 {
            continue;
        }
        if is_corrupt_line ( i, closing_index, line.clone() ) {
            penality = match cv[closing_index] {
                ')' => 3,
                '}' => 1197,
                ']' => 57,
                '>' => 25137,
                _ => 0,
            };
            return penality;
        }
    }
    penality
} 

fn calcuculate_all ( input : Vec<String> ) -> u64 {
    input.iter().map(|l| calculate_penality(l.to_string()) ).sum()
}

fn complete_line ( s : String ) -> Vec<char> {
    let mut comp = vec![];
    let cv : Vec<char> = s.chars().collect();
    let mut close_pos = 0;
    let mut stack = vec![];

    for i in 0..cv.len() {
        if cv[i] == '(' || cv[i] == '[' || cv[i] == '{' || cv[i] == '<' {
            stack.push(cv[i]);
        } else if cv[i] == ')' || cv[i] == ']' || cv[i] == '}' || cv[i] == '>' {
            stack.pop();
            if stack.is_empty() {
                break;
            }
        }
    }

    stack.reverse();
    for s in stack.clone() {   
        let return_char = match s {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            _  => panic!("not valid character: {:?}", cv.last().unwrap() )
        };
        comp.push(return_char);
    };
    println!("stack: {:?} - comp: {:?}", stack, comp);
    comp
} 

fn calculate_score( s : Vec<char> ) -> u64 {
    let mut sum = 0;
    s.iter().for_each( |x| { let p = match x {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("non valid character"),
            };
            sum *= 5;
            sum += p; 
        } ); 
    sum
}

fn corrupt ( l : String ) -> bool {
    let cv : Vec<char> = l.chars().collect();
    for (i,c) in cv.iter().enumerate() {
        let closing_index = find_closing_index( i, l.clone() );
        //println!("closing_index: {:?}", closing_index); 
        if closing_index == 0 {
            continue;
        }
        if is_corrupt_line ( i, closing_index, l.clone() ) {
            return true;
        }
    }
    false
}

fn is_uncomplete_line ( l : String ) -> bool {
    let cv : Vec<char> = l.chars().collect();
    for (i,_c) in cv.iter().enumerate() {
        let closing_index = find_closing_index( i, l.clone() );
        println!("closing_index: {:?}", closing_index); 
        if closing_index == 0 {
            return true;
        }
    }
    false
}

fn get_complete_score ( input : Vec<String> ) -> Vec<u64> {
    let mut scores = vec![];
    for line in input {
        if corrupt(line.clone()){
            continue;
        }
        let comp = complete_line(line);
        scores.push(calculate_score(comp));
    }
    scores.sort();
    let index = (scores.len()/2)+2;
    println!("length: {:?} - index: {:?}", scores.len(), index);
    scores
}

#[cfg(test)]
mod tests {
    static PATH : &str = "src/day10/input_day10.txt";
    static TESTPATH : &str = "src/day10/test_day10.txt";

    use super::*;

    #[test]
    fn read_path() {
        let inst = read_input(PATH);
        assert_eq!(inst.is_empty(), false); 
        assert_eq!(inst.len(), 94);
    }

    #[test]
    fn is_legal_line_test() {
        let s = String::from("{()()()}");
        assert_eq!(find_closing_index(0, s.clone()), 7 );
        assert_eq!(is_corrupt_line(0, 7, s.clone()), false);
        assert_eq!(find_closing_index(1, s), 2 );
        let corupt = String::from("{()()()>");
        assert_eq!(find_closing_index(0, corupt.clone()), 7);
        assert_eq!(is_corrupt_line(0, 7, corupt), true);
    }

    #[test]
    fn calculate_penenality_test() {
        let inst = read_input(TESTPATH);
        assert_eq!(inst.len(), 10);
        assert_eq!(calculate_penality(inst[2].clone()), 1197 );
        assert_eq!(calculate_penality(inst[4].clone()), 3 );
        assert_eq!(calculate_penality(inst[7].clone()), 3 );
        assert_eq!(calculate_penality(inst[8].clone()), 25137 );

        assert_eq!(calculate_penality(inst[0].clone()), 0 );
        assert_eq!(calculate_penality(inst[1].clone()), 0 );
    }

    #[test]
    fn calculate_all() {
        let inst = read_input(TESTPATH);
        assert_eq!(inst.len(), 10);
        assert_eq!(calcuculate_all(inst), 26397);
    }

    #[test]
    fn complete_line_test () {
        let s = String::from("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!( complete_line(s), vec![ '}','}',']',']',')','}',')',']' ] );
    }

    #[test]
    fn calculate_score_test() {
        let comp = vec![ ']',')','}','>' ];
        assert_eq!(calculate_score(comp), 294);
    }

    #[test]
    fn get_complete_score_test() {
        let inst = read_input(TESTPATH);
        assert_eq!(inst.len(), 10);
        assert_eq!(get_complete_score(inst)[3], 288957);
    }

    #[test]
    fn riddle_1() {
        let inst = read_input(PATH);
        assert_eq!(inst.len(), 94);
        assert_eq!(calcuculate_all(inst), 364389);
    }

    #[test]
    fn riddle_2() {
        let inst = read_input(PATH);
        assert_eq!(inst.len(), 94);
        let result = get_complete_score(inst);
        let index = result.len()/2;
        assert_eq!(result[index], 2870201088);
    }
}