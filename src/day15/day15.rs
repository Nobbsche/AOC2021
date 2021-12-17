// 16.12.2021 19:13
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::Path;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

use ndarray::prelude::*;
use ndarray::Array;

fn read_input_to_array ( input_path: &str, dim : (usize, usize) ) -> Array2::<u32,> {
    let mut a = Array2::<u32,>::zeros((dim.0, dim.1));
    let mut count = (0,0);
    let cpath = env::current_dir().unwrap().join(Path::new(input_path));
    match fs::read_to_string(cpath) {
        Ok (input) => { 
            for l in input.lines() {
                for c in l.chars() {
                    a[[count.0, count.1]] = c.to_string().parse::<u32>().unwrap(); 
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
 
struct Grid {
    nodes: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    data: (usize, usize),
    edges: Vec<(usize,u32)>,
}
 
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: u32,
}
 
// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
 
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
 
type WeightedEdge = (usize, usize, u32);
 
impl Grid {
    fn new() -> Self {
        Grid { nodes: Vec::new() }
    }
 
    fn add_node(&mut self, data: (usize, usize) ) -> usize {
        let node = Node {
            edges: Vec::new(),
            data: data,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }
 
    fn create_edges<'a, I>(&mut self, iterator: I) where I: IntoIterator<Item=&'a WeightedEdge> {
        for &(start,end,weight) in iterator.into_iter() {
            if !self.nodes[start].edges.contains( &(end, weight) ) {
                self.nodes[start].edges.push((end,weight));
            }
        }
    }
 
    fn find_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, u32)> {
        let mut dist = vec![(usize::MAX, None); self.nodes.len()];
 
        let mut heap = BinaryHeap::new();
        dist[start] = (0, None);
        heap.push(State {
            node: start,
            cost: 0,
        });
 
        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                let mut path = Vec::with_capacity(dist.len() / 2);
                let mut current_dist = dist[end];
                path.push(end);
                while let Some(prev) = current_dist.1 {
                    path.push(prev);
                    current_dist = dist[prev];
                }
                path.reverse();
                return Some((path, cost));
            }
 
            if cost as usize > dist[node].0 {
                continue;
            }
            for edge in &self.nodes[node].edges {
                let next = State {
                    node: edge.0,
                    cost: cost + edge.1,
                };
                if (next.cost as usize) < dist[next.node].0 {
                    dist[next.node] = (next.cost as usize, Some(node));
                    heap.push(next);
                }
            }
            //println!("node: {:?} - cost: {:?}", node, cost);
        }
        None
    }
}

fn generate_edge ( array : & Array2::<u32,>, point : (i32, i32), map : &HashMap<(usize,usize), usize> ,grid_id : (usize, usize) ) -> Option<(usize, usize, u32)> {
    let shape = array.shape();
    if point.0 < 0 || point.0 as usize >= shape[0] || point.1 < 0 || point.1 as usize >= shape[1]  {
        return None;
    }

    let cost = array[[point.0 as usize, point.1 as usize]];
    let destination = map.get(&(point.0 as usize, point.1 as usize)).unwrap();
    let source = map.get(&(grid_id.0 as usize, grid_id.1 as usize)).unwrap();
    Some((*source,*destination,cost))
}
 
fn create_path ( array : Array2::<u32,>) -> u32 {

    let mut grid = Grid::new();
    let mut node_map : HashMap<(usize,usize), usize>= HashMap::new();
    
    for ((y, x), _value) in array.indexed_iter() {
        let id = grid.add_node((y,x));
        node_map.insert((y,x), id);
    }

    
    for (key, value) in &node_map {
        let ikey = (key.0 as i32, key.1 as i32);
        let mut edge_vec = vec![];
        if let Some (edge) = generate_edge( &array, (ikey.0+1, ikey.1), &node_map, *key ) { edge_vec.push(edge)};
        if let Some (edge) = generate_edge( &array, (ikey.0-1, ikey.1), &node_map, *key ) { edge_vec.push(edge)};
        if let Some (edge) = generate_edge( &array, (ikey.0, ikey.1+1), &node_map, *key ) { edge_vec.push(edge)};
        if let Some (edge) = generate_edge( &array, (ikey.0, ikey.1-1), &node_map, *key ) { edge_vec.push(edge)};
        println!("edges: {:?}", edge_vec);
        grid.create_edges(&edge_vec);
    }

    println!("start path");
    let shape = array.shape();
    let start = node_map.get(&(0,0)).unwrap();
    let end   = node_map.get(&(shape[0]-1, shape[1]-1)).unwrap();
    let (path, cost) = grid.find_path(*start, *end).unwrap();
 
    let mut sum = 0;
    print!("{:?}", grid.nodes[path[0]].data);
    for i in path.iter().skip(1) {
        let point = grid.nodes[*i].clone();
        let value = array[[point.data.0, point.data.1]];
        sum += value;
        //print!(" -> {:?} - {:?}", grid.nodes[*i].data, value);
    }
    println!("\nCost: {} - sum: {:?}", cost, sum);
    println!("\n{:?}\n", array);
    cost
}

#[cfg(test)]
mod tests {
    static PATH : &str = "src/day15/input_day15.txt";
    static TESTPATH : &str = "src/day15/test_day15.txt";

    use super::*;

    #[test]
    fn read_path() {
        let array = read_input_to_array(PATH, (100,100));
        assert_eq!(array.is_empty(), false); 
        assert_eq!(array.shape(), [100,100]);
    }

    #[test]
    fn test_find_path() {
        let array = read_input_to_array(TESTPATH, (10,10));
        assert_eq!( create_path (array), 40);
    }

    #[test]
    fn riddle_1() {
        let array = read_input_to_array(PATH, (100,100));
        assert_eq!( create_path (array), 824);
    }
}