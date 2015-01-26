#![feature(box_syntax)]
#![allow(unstable)]
extern crate argparse;

use std::collections::BinaryHeap;
use std::os;
use std::io::BufferedReader;
use std::io::File;
use argparse::{ArgumentParser, StoreTrue, Store};


pub mod graph;
pub mod search;

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
