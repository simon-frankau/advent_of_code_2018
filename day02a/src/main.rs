use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn counts(str: &str) -> (i32, i32) {
    let mut counts = HashMap::new();
    for ch in str.trim().chars() {
        let counter = counts.entry(ch).or_insert(0);
        *counter += 1;
    }
    let mut twos = 0;
    let mut threes = 0;
    for value in counts.values() {
        if *value == 2 {
            twos += 1;
        } else if *value == 3 {
            threes += 1;
        }
    }
    return (twos, threes);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut twos = 0;
    let mut threes = 0;

    for line in lines {
        let line = line.expect("Read error");
        let line = line.trim();
        let (count2, count3) = counts(&line);
        if count2 > 0 {
            twos += 1;
        }
        if count3 > 0 {
            threes += 1;
        }
    }

    println!("{} {} {}", twos, threes, twos * threes);
}
