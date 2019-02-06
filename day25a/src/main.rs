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

    println!("{:?}", edges);
}
