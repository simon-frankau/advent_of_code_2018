use std::io;
use std::io::BufRead;

fn pair_parse(str: &str) -> (i32, i32) {
    let mut bits = str.split(',').map(|s| s.trim().parse().unwrap());
    let x = bits.next().unwrap();
    let y = bits.next().unwrap();
    (x, y)
}

fn sum_distance(points: &[(i32, i32)], x: i32, y: i32) -> i32 {
    points
        .iter()
        .map(|(px, py)| (x - px).abs() + (y - py).abs())
        .fold(0, |x, y| x + y)
}

fn main() {
    let stdin = io::stdin();
    let coords: Vec<_> = stdin
        .lock()
        .lines()
        .map(|s| pair_parse(&s.unwrap()))
        .collect();

    let xs: Vec<_> = coords.iter().map(|(x, _)| *x).collect();
    let ys: Vec<_> = coords.iter().map(|(_, y)| *y).collect();

    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();

    let mut near_count = 0;

    // Let's brute-force this because it's a lot easier than the
    // alternatives. We'll keep count of nearest points for all
    // coordinates.

    // We'll just slap an extra border on because it doesn't make the
    // cost prohibitive, and it's easier than special-casing the
    // calculations for outside the edge.
    let extra = (10000 / coords.len() as i32) + 1;

    let min_x = min_x - extra;
    let max_x = max_x + extra;
    let min_y = min_y - extra;
    let max_y = max_y + extra;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if sum_distance(&coords, x, y) < 10000 {
                near_count += 1;
            }
        }
    }

    println!("{}", near_count);
}
