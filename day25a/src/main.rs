use std::collections::HashMap;
use std::io;
use std::io::BufRead;

const CONSTELLATION_BOUND: i32 = 3;

#[derive(Debug)]
struct Point {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Point {
    fn dist(self: &Point, other: &Point) -> i32 {
        (self.a - other.a).abs()
        + (self.b - other.b).abs()
        + (self.c - other.c).abs()
        + (self.d - other.d).abs()

    }
}

fn read_point(str: &str) -> Point {
    let parts = str.split(",")
        .map(|s| s.trim().parse::<i32>().expect("Parse error"))
        .collect::<Vec<_>>();
    // Convert from origin/size to min/max coords.
    Point {
        a: parts[0],
        b: parts[1],
        c: parts[2],
        d: parts[3],
    }
}

struct UnionFind {
    u: HashMap<usize, Option<usize>>,
}

impl UnionFind {
    fn new() -> UnionFind {
        UnionFind {
            u: HashMap::new(),
        }
    }

    fn insert(self: &mut UnionFind, i: usize) {
        if self.u.insert(i, None) != None {
            panic!("Set {} already inserted", i);
        }
    }

    // Union the sets and return the current id of the unioned sets.
    fn union(self: &mut UnionFind, i: usize, j: usize) -> usize{
        if let Some(parent_i) = self.u.get(&i).unwrap().clone() {
            let res = self.union(parent_i, j);
            self.u.insert(i, Some(res));
            res
        } else if let Some(parent_j) = self.u.get(&j).unwrap().clone() {
            let res = self.union(i, parent_j);
            self.u.insert(j, Some(res));
            res
        } else {
            // Both have no parent, union 'em.
            if i != j {
                self.u.insert(j, Some(i));
            }
            i
        }
    }

    // Count parent-less things. This will be the number of distinct sets.
    fn count(self: &UnionFind) -> usize {
        self.u.values().filter(|x| x.is_none()).count()
    }
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<Point> = stdin
        .lock()
        .lines()
        .map(|s| read_point(&s.expect("Read error")))
        .collect();

    // Build a list of pairs of points in the same constellation.
    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            if points[i].dist(&points[j]) <= CONSTELLATION_BOUND {
                edges.push((i, j));
            }
        }
    }

    let mut uf = UnionFind::new();
    for i in 0..points.len() {
        uf.insert(i);
    }
    for (i, j) in edges.iter() {
        uf.union(*i, *j);
    }
    println!("{}", uf.count());
}
