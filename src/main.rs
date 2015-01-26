#![feature(box_syntax)]
extern crate argparse;

use std::collections::BinaryHeap;
use std::os;
use std::io::BufferedReader;
use std::io::File;
use argparse::{ArgumentParser, StoreTrue, Store};


pub mod graph {
    pub struct Graph {
        nodes: Vec<(char, i32, i32)>
    }
}

pub mod search {

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
    for line in ls {
        let l2 = line.unwrap();
        let words:Vec<&str> = l2.trim().split_str(" ").collect();
        let c:char = words[0].chars().next().unwrap();
        let x:i32 = words[1].parse().unwrap();
        let y:i32 = words[2].parse().unwrap();
        println!("{} {} {}", c, x, y);
    }
}
