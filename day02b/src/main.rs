use std::io;
use std::io::BufRead;

fn distance(str1: &str, str2: &str) -> i32 {
    // Assumes strings same length.
    let mut iter1 = str1.trim().chars();
    let mut iter2 = str2.trim().chars();
    let mut dist = 0;
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) => if c1 != c2 {
                dist += 1;
            },
            _ => return dist,
        }
    }
}

fn print_shared(str1: &str, str2: &str) {
    // Assumes strings same length.
    let mut iter1 = str1.trim().chars();
    let mut iter2 = str2.trim().chars();
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) => if c1 == c2 {
                print!("{}", c1);
            },
            _ => {
                println!("");
                return;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|s| s.expect("Read error"))
        .collect();

    // Number of lines is short, let's brute-force.
    for line1 in lines.iter() {
        for line2 in lines.iter() {
            if distance(&line1, &line2) == 1 {
                print_shared(&line1, &line2);
                return;
            }
        }
    }
}
