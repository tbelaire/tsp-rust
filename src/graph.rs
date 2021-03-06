use std::num::Float;
use std::num::Int;
pub struct Graph {
    size: usize,
    nodes: Vec<(char, i32, i32)>,
    distances: Vec<Vec<usize>>,
    min_cost_edges: Vec<usize>

}
impl Graph {
    pub fn new(v : Vec<(char, i32, i32)>) -> Graph {
        let dist:Vec<Vec<usize>> = v.iter().map(
                |&(_, ax, ay)| {
                    v.iter().map(|&(_, bx, by)| {
                        let d2:f32 = ((ax - bx).pow(2) + (ay - by).pow(2)) as f32;
                        (d2.sqrt() + 0.5) as usize
                    }).collect()
                }).collect();
        let mce:Vec<usize> = dist.iter().map(
            // Note, if there are cities on top of each other, the min cost is
            // wrong
            |ds|->usize *(ds.iter().filter(|&&d| d!= 0).min().unwrap())).collect();
        Graph{
            size: v.len(),
            nodes: v,
            distances: dist,
            min_cost_edges: mce

        }
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn tour_length(&self, path: &[usize]) -> usize {
        let mut len = 0;
        for (a,b) in path.iter().zip(
                path.iter().cycle().skip(1)) {
            len += self.distances[*a][*b];
        }
        len
    }
    pub fn path_length(&self, path: &[usize]) -> usize {
        let mut len = 0;
        for (a,b) in path.iter().zip(
                path.iter().skip(1)) {
            len += self.distances[*a][*b];
        }
        len
    }

    pub fn print_tour(&self, path: &[usize]) {
        let mut it = path.iter();
        let first = it.next();
        match first {
            Some(i) => {
                let (c, _, _) = self.nodes[*i];
                print!("[{}", c);
                for i in it {
                    let (c, _, _) = self.nodes[*i];
                    print!(", {}", c);
                }
                print!("]")
            },
            None => print!("[]"),
        }
    }

    pub fn lower_bound_tour_mce(&self, path: &[usize], k:usize)
        -> usize {
            let p = self.path_length(&path[0..k]);

            let s = path[k..].iter()
                .fold(0, |sum, &i| sum + self.min_cost_edges[i]);
                // .fold(0, |sum, &i| sum + i);
            p+s
    }
    pub fn dist(&self, a : usize, b : usize) -> usize {
        self.distances[a][b]
    }

}

#[test]
fn path_test(){
    let g = Graph::new(vec![('a', 0,0), ('b', 0,4), ('c', 4,4), ('d', 4,0)]);
    let perm = vec!(0,1,2,3);
    assert_eq!(g.tour_length(perm.as_slice()), 16);
    let perm = vec!(0,1);
    assert_eq!(g.tour_length(perm.as_slice()), 8);
    assert_eq!(g.path_length(perm.as_slice()), 4);

}

#[test]
fn mce_test(){
    let g = Graph::new(vec![('a', 0,0), ('b', 0,4), ('c', 4,4), ('d', 4,0)]);
    let perm = vec!(0,1,2,3);
    assert_eq!(g.lower_bound_tour_mce(perm.as_slice(), 2), 16);
}

