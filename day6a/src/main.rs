use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn pair_parse(str: &str) -> (i32, i32) {
    let mut bits = str.split(',').map(|s| s.trim().parse().unwrap());
    let x = bits.next().unwrap();
    let y = bits.next().unwrap();
    (x, y)
}

fn no_draw_min<I, T, U>(mut iter: I) -> Option<U>
where
    I: Iterator<Item = (T, U)>,
    T: std::cmp::PartialOrd,
{
    let mut val = iter.next().unwrap();
    let mut is_draw = false;
    loop {
        match iter.next() {
            None => {
                if is_draw {
                    return None;
                } else {
                    return Some(val.1);
                }
            }
            Some(x) => {
                if x.0 == val.0 {
                    is_draw = true;
                }
                if x.0 < val.0 {
                    is_draw = false;
                    val = x;
                }
            }
        }
    }
}

fn find_nearest(points: &[(i32, i32)], x: i32, y: i32) -> Option<(i32, i32)> {
    no_draw_min(
        points
            .iter()
            .map(|(px, py)| ((x - px).abs() + (y - py).abs(), (*px, *py))),
    )
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

    // Let's brute-force this because it's a lot easier than the
    // alternatives. We'll keep count of nearest points for all
    // coordinates.
    let mut counts = HashMap::new();
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let nearest = find_nearest(&coords, x, y);
            *(counts.entry(nearest).or_insert(0)) += 1;
        }
    }

    // Now, let's remove all the points that have infinite area. Any
    // point around the edge can be extrapolated to a line to infinity,
    // so let's just remove all points that occur around the edge.
    for x in min_x..max_x + 1 {
        counts.remove(&find_nearest(&coords, x, min_y));
        counts.remove(&find_nearest(&coords, x, max_y));
    }
    for y in min_y..max_y + 1 {
        counts.remove(&find_nearest(&coords, min_x, y));
        counts.remove(&find_nearest(&coords, max_x, y));
    }
    // Nearest to nothing cannot be a solution.
    counts.remove(&None);

    println!("{}", counts.values().max().unwrap());
}
