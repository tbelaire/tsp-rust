use super::graph;
use std::usize;

pub fn enumerate_all(g: graph::Graph) -> usize{
    let mut perm:Vec<usize> = range(0,g.len()).collect();
    let mut best_tour:Vec<usize> = Vec::new();
    let mut best_distance = usize::MAX;
    fn permute(perm: &mut Vec<usize>,
                best_tour: &mut Vec<usize>,
                best_distance: &mut usize,
                g: &graph::Graph, k: usize) {
        if k == 1 {
            let len = g.tour_length(perm);
            // print!("Tour: ");
            // g.print_tour(perm);
            // println!(" of length {}", len);

            if len < *best_distance {
                *best_distance = len;
                *best_tour = perm.clone();
            }
        } else {
            for i in range(0, k) {
                perm.swap(i, k-1);
                permute(perm, best_tour, best_distance, g, k-1);
                perm.swap(i, k-1)
            }
        }
    }
    permute(&mut perm, &mut best_tour, &mut best_distance, &g, g.len());
    print!("Best tour is: ");
    g.print_tour(&best_tour);
    println!("");
    best_distance

}

