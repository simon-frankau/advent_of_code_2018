use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;

const SPLIT_FACTOR: i64 = 5;

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
// Arbirary "Ord" implementation to allow use in a BinaryHeap.
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone)]
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

    fn score(self: &BoundingBox, nanobots: &[Nanobot]) -> usize {
        nanobots.iter().filter(|x| self.intersects(x)).count()
    }

    fn subdivide(self: &BoundingBox) -> Vec<BoundingBox> {
        let mut res = Vec::new();
        for (min_x, max_x) in split_range(self.min_x, self.max_x, SPLIT_FACTOR) {
            for (min_y, max_y) in split_range(self.min_y, self.max_y, SPLIT_FACTOR) {
                for (min_z, max_z) in split_range(self.min_z, self.max_z, SPLIT_FACTOR) {
                    res.push(BoundingBox {
                        min_x: min_x,
                        max_x: max_x,
                        min_y: min_y,
                        max_y: max_y,
                        min_z: min_z,
                        max_z: max_z,
                    });
                }
            }
        }
        return res;
    }
}

// Find the distance from val to the given range.
fn range_dist(val: i64, min_val: i64, max_val: i64) -> i64 {
    (min_val - val).max(val - max_val).max(0)
}

// Split up an interval into x pieces. Pretty rough. Meh.
fn split_range(lower: i64, upper: i64, pieces: i64) -> impl Iterator<Item = (i64, i64)> {
    let num_entries = upper - lower + 1;
    // Divide with rounding up.
    let entries_per_piece = (num_entries + pieces - 1) / pieces;
    return (0..pieces).map(move |x| {
        let piece_lower = lower + x * entries_per_piece;
        let piece_upper = (piece_lower + entries_per_piece - 1).min(upper);
        (piece_lower, piece_upper)
    }).filter(|(l, u)| l <= u);
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
    best_score: (usize, i64),
    // Heap ordered by best-case number of possible overlaps with the
    // bounding box.
    candidates: BinaryHeap<(usize, BoundingBox)>,
}

impl State {
    fn new(bb: &BoundingBox) -> State {
        let mut candidates: BinaryHeap<(usize, BoundingBox)> = BinaryHeap::new();
        // Initial score doesn't matter, since it's the only item in the queue.
        candidates.push((1, (*bb).clone()));
        State {
            // Let's set the initial bar at having at least 1 nanobot nearby.
            best_score: (1, 0),
            candidates: candidates,
        }
    }

    fn process_candidate(self: &mut State, nanobots: &[Nanobot], candidate: (usize, BoundingBox)) {
        let (score, bb) = candidate;
        if bb.is_unit_box() {
            let origin_dist = 0 - bb.min_x.abs() - bb.min_y.abs() - bb.min_z.abs();
            // Can't subdivide any further. Update score if needed.
            let new_score = (score, origin_dist);
            self.best_score = self.best_score.min(new_score);
        } else {
            for new_bb in bb.subdivide().into_iter() {
                let new_score = new_bb.score(nanobots);
                // Only queue it up if it's a plausible candidate
                if new_score >= self.best_score.0 {
                    self.candidates.push((new_score, new_bb));
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let nanobots: Vec<Nanobot> = stdin
        .lock()
        .lines()
        .map(|s| read_nanobot(&s.expect("Read error")))
        .collect();

    let bb = get_bounding_box(&nanobots);
    println!("{:?} {}", bb, bb.score(&nanobots));

    let mut state = State::new(&bb);
    let candidate = state.candidates.pop().unwrap();
    state.process_candidate(&nanobots, candidate);
    for candidate in state.candidates.iter() {
        println!("{:?}", candidate);
    }
}
