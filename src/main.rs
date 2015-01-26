#![feature(box_syntax)]
#![allow(unstable)]
extern crate argparse;

use std::collections::BinaryHeap;
use std::os;
use std::io::BufferedReader;
use std::io::File;
use argparse::{ArgumentParser, StoreTrue, Store};


pub mod graph {
    use std::num::Float;
    use std::num::Int;
    pub struct Graph {
        size: usize,
        nodes: Vec<(char, i32, i32)>,
        distances: Vec<Vec<usize>>
    }
    impl Graph {
        pub fn new(v : Vec<(char, i32, i32)>) -> Graph {
            let dist =  v.iter().map(
                    |&(_, ax, ay)| {
                        v.iter().map(|&(_, bx, by)| {
                            let d2:f32 = ((ax - bx).pow(2) + (ay - by).pow(2)) as f32;
                            (d2.sqrt() + 0.5) as usize
                        }).collect()
                    }).collect();
            Graph{
                size: v.len(),
                nodes: v,
                distances: dist
            }
        }
        pub fn len(&self) -> usize {
            self.size
        }

        pub fn tour_length(&self, path: &Vec<usize>) -> usize{
            let mut len = 0;
            for (a,b) in path.iter().zip(
                    path.iter().cycle().skip(1)) {
                len += self.distances[*a][*b];
            }
            len
        }
        pub fn path_length(&self, path: &Vec<usize>) -> usize{
            let mut len = 0;
            for (a,b) in path.iter().zip(
                    path.iter().skip(1)) {
                len += self.distances[*a][*b];
            }
            len
        }
    }

    #[test]
    fn path_test(){
        let g = Graph::new(vec![('a', 0,0), ('b', 0,4), ('c', 4,4), ('d', 4,0)]);
        let perm = vec!(0,1,2,3);
        assert_eq!(g.tour_length(&perm), 16);
        let perm = vec!(0,1);
        assert_eq!(g.tour_length(&perm), 8);
        assert_eq!(g.path_length(&perm), 4);

    }
}

pub mod search {
    use super::graph;
    pub fn enumerate_all(g: graph::Graph) -> usize{
        let mut perm:Vec<usize> = Vec::new();
        let mut best_toup:Vec<usize> = Vec::new();
        let mut best_distance = 0;



    }

}

fn main() {

    let mut verbose = false;
    let mut filename = "-".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Search for good tsp solution");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], box StoreTrue,
            "Be verbose");
        ap.refer(&mut filename)
            .add_option(&["-f", "--filename"], box Store::<String>,
            "File containing the cities");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                os::set_exit_status(x);
                return;
            }
        }
    }
    if verbose {
        println!("Filename is {}", filename);
    }

    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let mut ls = file.lines();
    let n_str = ls.next();
    let graph_lst:graph::Graph = graph::Graph::new(ls.map(|line| {
        let l2 = line.unwrap();
        // Trim removes the trailing whitespace
        let words:Vec<&str> = l2.trim().split_str(" ").collect();
        let c:char = words[0].chars().next()
                        .expect("Failed to parse city name");
        let x:i32 = words[1].parse().expect("Failed to parse x co-ordinate");
        let y:i32 = words[2].parse().expect("Failed to parse y co-ordinate");
        (c,x,y)
    }).collect());

    println!("Best path is {}", search::enumerate_all(graph_lst));
}
