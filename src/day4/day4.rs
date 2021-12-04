// 10:24
use std::fs;
use std::env;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct BingoNumber {
    number : u32,
    marked : u8,
}

impl BingoNumber {
    pub fn new ( i : u32)  -> Self {
        BingoNumber {
            number: i,
            marked : 0,
        }
    } 

    pub fn is_equal (&self, i : &u32 ) -> bool {
        self.number == *i 
    }

    pub fn is_marked(&self) -> u8 {
        self.marked.clone()
    }

    pub fn mark_number (&mut self, number : &u32) {
        if self.is_equal(number) {
            self.marked = 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    board : Vec<Vec<BingoNumber>>,
    has_won : bool,
}

impl Board {
    pub fn new (board : Vec<Vec<u32>>) -> Self {
        Board {
            board : board.iter().map(|s| s.iter().map(|n| BingoNumber::new(n.clone()) ).collect() ).collect(),
            has_won : false
        }
    }

    pub fn is_winner (&mut self) -> bool {
        let row_winner : Vec<u32> = self.board.iter().filter_map(|r| { if r.iter().map(|e| e.is_marked()).sum::<u8>() == 5 { Some(1) } else {None} } ).collect();
        let column = self.transpose_matrix(self.board.clone());
        let col_winner : Vec<u32> = column.iter().filter_map(|r| { if r.iter().map(|e| e.is_marked()).sum::<u8>() == 5 { Some(1) } else {None} }  ).collect();
        if row_winner.len() > 0 || col_winner.len() > 0 {
            self.has_won = true;
            return true;
        } 
        false
    }

    pub fn result (&self, number : u32) -> u32 {
        let r : u32 = self.board.iter().map( |r| { let rs : u32 = r.iter().map( |e| { if e.is_marked() == 0 { e.number } else {0} } ).sum();
                                                     //println!("rs: {:?}", rs);
                                                     rs }
                                                     ).sum();
        println!("r: {:?}", r);
        r * number
    }

    fn transpose_matrix (&self, bits : Vec<Vec<BingoNumber>> ) -> Vec<Vec<BingoNumber>> {
        let mut t = vec![Vec::with_capacity(bits.len()); bits[0].len()];
        for r in bits {
            for i in 0..r.len() {
                t[i].push(r[i]);
            }
        }
        t
    }
}


fn read_input_to_board ( input_path: &str ) -> Vec<Board> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    let mut playBoards = vec![];
    lazy_static! { static ref RE: Regex = Regex::new(r" *(\d*) *(\d*) *(\d*) *(\d*) *(\d*)").unwrap(); };
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            let boards : Vec<&str> = input.split( "\r\n\r\n").collect();
            for board in boards {
                let b : Vec<Vec<u32>> = board.lines().map( |l| { 
                    RE.captures(l).unwrap().iter().filter_map(|m| {
                        match m.map_or("", |m| m.as_str() ).parse::<u32>() {
                            Ok(x) => Some(x),
                            Err(_) => None,
                        } }).collect() 
                    } ).collect();
                playBoards.push(Board::new(b));
            }
        }
        Err (e) => {panic!("Could not parse input: {:?}", e);}
    }
    playBoards
}

fn read_input_to_draw ( input_path: &str ) -> Vec<u32> {
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => {input.split(',').map(|s| s.to_string().parse::<u32>().unwrap()).collect()}
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
}

fn play_bingo ( boards: Vec<Board>, draw : Vec<u32>) -> u32 {
    let mut boards = boards;
    for number in draw {
        for board in &mut boards {
            mark_number( board, &number);
            if board.is_winner () {
                return board.result(number);
            }
        }
    }
0
}

fn let_the_squid_win ( boards: Vec<Board>, draw : Vec<u32> ) -> Vec<u32> {
    let mut boards = boards;
    let mut board_results = vec![];
    for number in draw {
        for board in &mut boards {
            if board.has_won { continue;}
            mark_number( board, &number);
            if board.is_winner () {
                board_results.push(board.result(number));
            }
        }
    }
    board_results
}

pub fn mark_number (board: &mut Board, number : &u32 ) {
    //println!("mark: {:?}", board);
    let boardV = board.board.iter_mut().map(|r| r.iter_mut().map(|e| {e.mark_number(number); *e }).collect() ).collect();
    board.board = boardV;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_path() {
        let boards = read_input_to_board("src/day4/input_day4_boards.txt");
        assert_eq!(boards.is_empty(), false); 
    }

    #[test]
    fn parse_boards() {
        let boards = read_input_to_board("src/day4/test_day4_boards.txt");
        assert_eq!( boards.len(), 3); 
    }

    #[test]
    fn read_draw() {
        let draw = read_input_to_draw("src/day4/test_day4_draw.txt");
        assert_eq!( draw.len(), 27); 
    }

    #[test]
    fn play_bingo_test() {
        let boards = read_input_to_board("src/day4/test_day4_boards.txt");
        let draw = read_input_to_draw("src/day4/test_day4_draw.txt");
        assert_eq!( play_bingo(boards, draw ), 4512); 
    }

    #[test]
    fn let_squid_test() {
        let boards = read_input_to_board("src/day4/test_day4_boards.txt");
        let draw = read_input_to_draw("src/day4/test_day4_draw.txt");
        assert_eq!( let_the_squid_win(boards, draw ).last().unwrap().clone(), 1924); 
    }

    #[test]
    fn riddle_1() {
        let boards = read_input_to_board("src/day4/input_day4_boards.txt");
        let draw = read_input_to_draw("src/day4/input_day4_draw.txt");
        assert_eq!( play_bingo(boards, draw ), 67716); 
    }

    #[test]
    fn riddle_2() {
        let boards = read_input_to_board("src/day4/input_day4_boards.txt");
        let draw = read_input_to_draw("src/day4/input_day4_draw.txt");
        assert_eq!( let_the_squid_win(boards, draw ).last().unwrap().clone(), 1924); 
    }
}