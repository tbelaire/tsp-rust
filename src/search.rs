use super::graph;
use std::usize;

pub fn enumerate_all(g: graph::Graph) -> usize{
    let mut perm:Vec<usize> = range(0,g.len()).collect();
    let mut best_tour:Vec<usize> = Vec::new();
    // let mut best_tour = [0; g.len()];
    let mut best_distance = usize::MAX;
    fn permute(perm: &mut Vec<usize>,
                best_tour: &mut Vec<usize>,
                best_distance: &mut usize,
                g: &graph::Graph, k: usize,
                tourlen: usize) {
        if k == 1 {
            // let len = g.tour_length(perm.as_slice());
            let len = (tourlen + g.dist(perm[0], perm[1])
                       + g.dist(perm[g.len()-1], perm[0]));


            // print!("Tour: ");
            // g.print_tour(perm);
            // println!(" of length {}", len);

            if len < *best_distance {
                *best_distance = len;
                best_tour.clone_from(perm);
            }
        } else {
            for i in range(0, k) {
                perm.swap(i, k-1);
                let a = perm[k-1];
                let b = perm[k];
                permute(perm, best_tour, best_distance, g, k-1,
                       tourlen + g.dist(a, b));
                perm.swap(i, k-1)
            }
        }
    }
    permute(&mut perm, &mut best_tour, &mut best_distance, &g, g.len()-1, 0);
    print!("Best tour is: ");
    g.print_tour(best_tour.as_slice());
    println!("");
    best_distance

}

