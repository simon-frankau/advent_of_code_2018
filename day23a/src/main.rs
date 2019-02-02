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

fn main() {
    let stdin = io::stdin();
    let nanobots: Vec<Nanobot> = stdin
        .lock()
        .lines()
        .map(|s| read_nanobot(&s.expect("Read error")))
        .collect();

    let strongest = nanobots.iter().max().unwrap();

    println!("{:?}", nanobots.iter().filter(|x| strongest.in_range_of(x)).count());
}
