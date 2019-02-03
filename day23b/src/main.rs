use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

// Representation of rectangle with lower bound included, upper bound
// excluded.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Nanobot {
    r: i64,
    x: i64,
    y: i64,
    z: i64
}

impl Nanobot {
    fn in_range_of(&self, n: &Nanobot) -> bool {
        (n.x - self.x).abs() + (n.y - self.y).abs() + (n.z - self.z).abs() <= self.r
    }
}

fn read_nanobot(str: &str) -> Nanobot {
    // Normalise the separators, and split up.
    let parts = str
        .replace("pos=<", "")
        .replace(">, r=", ",")
        .split(',')
        .map(|s| s.trim().parse::<i64>().expect("Parse error"))
        .collect::<Vec<_>>();
    Nanobot {
        r: parts[3],
        x: parts[0],
        y: parts[1],
        z: parts[2],
    }
}

// Plan:
// 1. Count intersections with cuboid. Keep heap of cuboids by
//    ordered by intersections, decreasing.
// 2. Take first cube off list, slice up into 8, add to heap.
// 3. When you're down to a single location, set optimal score
//    if it's >= current one, update distance if it's less
//    than current one.
// 4. Stop when best cuboid has a score worse than your optimal score.

// Min and max values are inclusive for the bounding box.
#[derive(Debug)]
struct BoundingBox {
  min_x: i64,
  max_x: i64,
  min_y: i64,
  max_y: i64,
  min_z: i64,
  max_z: i64,
}

impl BoundingBox {
    fn intersects(self: &BoundingBox, nanobot: &Nanobot) -> bool {
        let dx = range_dist(nanobot.x, self.min_x, self.max_x);
        let dy = range_dist(nanobot.y, self.min_y, self.max_y);
        let dz = range_dist(nanobot.z, self.min_z, self.max_z);
        dx + dy + dz <= nanobot.r
    }

    fn is_unit_box(self: &BoundingBox) -> bool {
        self.min_x == self.max_x && self.min_y == self.max_y && self.min_z == self.max_z
    }
}

// Find the distance from val to the given range.
fn range_dist(val: i64, min_val: i64, max_val: i64) -> i64 {
    (min_val - val).max(val - max_val).max(0)
}

fn get_bounding_box(nanobots: &[Nanobot]) -> BoundingBox {
    let xs = nanobots.iter().map(|b| b.x).collect::<Vec<_>>();
    let ys = nanobots.iter().map(|b| b.y).collect::<Vec<_>>();
    let zs = nanobots.iter().map(|b| b.z).collect::<Vec<_>>();
    BoundingBox {
        min_x: *xs.iter().min().unwrap(),
        max_x: *xs.iter().max().unwrap(),
        min_y: *ys.iter().min().unwrap(),
        max_y: *ys.iter().max().unwrap(),
        min_z: *zs.iter().min().unwrap(),
        max_z: *zs.iter().max().unwrap(),
    }
}

struct State {
    // Score is combination for number of overlapping nanobots (LHS)
    // and negated distance from origin (RHS). Lexicographically higher
    // is better.
    best_score: (i64, i64),
    // Heap ordered by best-case number of possible overlaps with the
    // bounding box.
    candidates: BinaryHeap<(i64, BoundingBox)>,
}

fn main() {
    let stdin = io::stdin();
    let nanobots: Vec<Nanobot> = stdin
        .lock()
        .lines()
        .map(|s| read_nanobot(&s.expect("Read error")))
        .collect();

    let bb = get_bounding_box(&nanobots);
    println!("{:?} {}", bb, nanobots.iter().filter(|x| bb.intersects(x)).count());
}
