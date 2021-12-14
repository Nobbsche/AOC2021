// 10:13
use std::fs;
use std::env;
use std::path::Path;

use itertools::Itertools;

fn read_input_to_graph ( input_path: &str, visit_mode : bool ) -> CaveGrid {
    let mut grid = CaveGrid::new(visit_mode);
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            for l in input.lines() {
                let s : Vec<&str> = l.split('-').collect();
                let start = String::from(s[0]);
                let end = String::from(s[1]);
                let a = grid.add_node(start);
                let b = grid.add_node(end);
                grid.create_edges(&[(a, b, 1)]);
            }
        }    
        Err (e) => panic!("Could not parse input: {:?}", e)
    }
    println!("Grid: \n {:?}", grid);
    grid
}

pub type Edge = (usize, usize);

#[derive(Debug, Clone)]
pub struct Cave {
    id : usize,
    name : String,
    edges : Vec<Edge>,
    visited : bool,
    is_start : bool,
    is_end : bool,
    is_small_cave: bool,
    visited_counter: u32,    
}

impl Cave {
    pub fn set_visted (&mut self) {
        if self.name.chars().collect::<Vec<char>>()[0].is_ascii_lowercase() {
            self.visited = true;
        }
    }

    pub fn is_small_cave (&self) -> bool {
        self.is_small_cave
    }

    pub fn is_start (&self) -> bool {
        self.is_start
    }

    pub fn is_end (&self) -> bool {
        self.is_end
    }

    pub fn reset_visited (&mut self) {
        self.visited = false;
        self.visited_counter = 0;
    }
}

#[derive(Debug, Clone)]
struct CaveGrid {
    nodes: Vec<Cave>,
    pathList : Vec<Vec<usize>>,
    visit_small_caves : bool,
    start_node : usize,
    end_node : usize,
}

type WeightedEdge = (usize, usize, usize);

impl CaveGrid {
    fn new( visit_mode : bool ) -> Self {
        CaveGrid { nodes: Vec::new(), pathList: vec![], visit_small_caves: visit_mode,  start_node : 0, end_node : 0 } 
    }

    fn add_node(&mut self, data: String) -> usize {
        if let Some (pos) = self.nodes.iter().position(|c| c.name == data ) {
            return pos;
        } 

        let is_start = data == String::from("start"); 
        let is_end = data == String::from("end");
        let is_small_cave = data.chars().collect::<Vec<char>>()[0].is_ascii_lowercase();

        let mut node = Cave {
            id : 0,
            edges: Vec::new(),
            name: data,
            visited : false,
            is_start : is_start,
            is_end : is_end,
            is_small_cave : is_small_cave,
            visited_counter : 0,
        };

        self.nodes.push(node.clone());
        self.nodes.last_mut().unwrap().id = self.nodes.len() - 1;
        
        if is_start {
            self.start_node = self.nodes.len() - 1;
        }
        if is_end {
            self.end_node = self.nodes.len() - 1;
        }

        self.nodes.len() - 1
    }
 
    fn create_edges<'a, I>(&mut self, iterator: I) where I: IntoIterator<Item=&'a WeightedEdge> {
        for &(start,end,weight) in iterator.into_iter() {
            self.nodes[start].edges.push((end,weight));
            self.nodes[end].edges.push((start,weight));
        }
    }

    fn find_all_paths ( &mut self ) -> usize {
        let start_node = self.nodes.iter().position(|c| c.name == String::from("start") ).unwrap();
        let end_node = self.nodes.iter().position(|c| c.name == String::from("end") ).unwrap();
        let mut path = vec![];
        path.push(start_node);
    
        self.find_path(&start_node, &end_node, path);
        println!("path list: {:?}", self.pathList);   
        self.pathList.len()
    }
    
    fn find_path(&mut self, start_node : &usize , end_node: &usize, mut path : Vec<usize> ) -> Vec<usize> {

        println!("entry => node: {:?} - path: {:?}", self.nodes[*start_node], path);
        if start_node == end_node {
            self.pathList.push(path.clone());
            return path;
        } 
    
        self.nodes[*start_node].set_visted();

        let node_edges = self.nodes[*start_node].edges.clone();
        for edge in node_edges.iter() {
            if !self.nodes[edge.0].visited {
                path.push(edge.0);
                path = self.find_path(&edge.0, end_node ,path );
                let remove_node = path.iter().position(|c| *c == edge.0 ).unwrap();
                path.remove(remove_node); 
            }
        }
        println!("exit => node: {:?} - path: {:?}", self.nodes[*start_node], path);
        self.nodes[*start_node].reset_visited();
        path
    }

    fn is_visited(&self, x: usize, path : &Vec<usize>) -> bool {

        if !self.nodes[x].is_small_cave() {
            return false;
        };

        let filter : Vec<&usize> = path.into_iter().filter(|&&e| e == self.start_node ).collect();
        if filter.len() == 1 && x == self.start_node {
            return true;
        }

        let filter : Vec<&usize> = path.into_iter().filter(|&&e| e == self.end_node ).collect();
        if filter.len() == 1 && x == self.end_node {
            return true;
        }

        if path.iter().find(|&&e| e == x ).is_some() {
            let filter : Vec<&usize> = path.iter().filter(|&&e| self.nodes[e].is_small_cave() ).collect();
            let f : Vec<&usize> = filter.clone().into_iter().unique().collect();

            if filter.len() == f.len() {
                return false;
            } else {
                return true;
            }

        }
        false
    }

    fn find_all_paths_BFS ( &mut self ) -> usize {
        let mut current_path_list = vec![];
        let mut current_path = vec![];
        current_path.push(self.start_node);
        current_path_list.push(current_path);
        let mut counter = 0;
        while !current_path_list.is_empty() && counter < 1000000 {
            let path = current_path_list.remove(0);
            let last = path.last().unwrap();
            
            if *last == self.end_node {
                //println!("path: {:?}", path); 
                self.pathList.push(path.clone());
                continue;
            }

            let node_edges = self.nodes[*last].edges.clone();
            for edge in node_edges.iter() {
                if !self.is_visited(edge.0, &path) {
                    let mut newpath = path.clone();
                    newpath.push(edge.0);
                    current_path_list.push(newpath);
                }
            }
            counter += 1;
        }
        //println!("nodes: {:?}", self.nodes );
        //println!("path: {:?}", self.pathList); 
        //println!("counter: {:?}", counter);
        self.pathList.len()
    }
}

#[cfg(test)]
mod tests {
    static PATH : &str = "src/day12/input_day12.txt";
    static TESTPATH : &str = "src/day12/test_day12.txt";
    static TESTPATH2: &str = "src/day12/test2_day12.txt";
    static TESTPATH3: &str = "src/day12/test3_day12.txt";

    use super::*;

    #[test]
    fn read_path() {
        let grid = read_input_to_graph(PATH, false);
        assert_eq!(grid.nodes.is_empty(), false); 
    }

    #[test]
    fn test_grahp_test() {
        let mut grid = read_input_to_graph(TESTPATH, false);
        assert_eq!(grid.find_all_paths(), 10);
    }

    #[test]
    fn test_grahp_test_sample_2() {
        let mut grid = read_input_to_graph(TESTPATH2, false);
        assert_eq!(grid.find_all_paths(), 19);
    }

    #[test]
    fn test_grahp_test_sample_3() {
        let mut grid = read_input_to_graph(TESTPATH3, false);
        assert_eq!(grid.find_all_paths(), 226);
    }

    #[test]
    fn test_grahp_test_advanced() {
        let mut grid = read_input_to_graph(TESTPATH, true);
        assert_eq!(grid.find_all_paths_BFS(), 36);
    }

    
    #[test]
    fn test_grahp_test_advanced_2() {
        let mut grid = read_input_to_graph(TESTPATH2, false);
        assert_eq!(grid.find_all_paths_BFS(), 103);
    }

    #[test]
    fn test_grahp_test_advanced_3() {
        let mut grid = read_input_to_graph(TESTPATH3, false);
        assert_eq!(grid.find_all_paths_BFS(), 3509);
    }

    #[test]
    fn riddle_1() {
        let mut grid = read_input_to_graph(PATH, false);
        assert_eq!(grid.find_all_paths(), 3738);
    }

    #[test]
    #[ignore]
    fn riddle_2() {
        let mut grid = read_input_to_graph(PATH, false);
        assert_eq!(grid.find_all_paths_BFS(), 120506);
    }
}