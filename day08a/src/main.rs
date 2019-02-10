use std::io;
use std::io::Read;

fn read_node<I>(iter: &mut I) -> i32
where
    I: Iterator<Item=i32> {

    let num_children = iter.next().unwrap();
    let num_metadata = iter.next().unwrap();

    let mut sum = 0;
    for _ in 0..num_children {
        sum += read_node(iter);
    }
    for _ in 0..num_metadata {
        sum += iter.next().unwrap();
    }
    sum
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let mut parts = buffer
        .split(' ')
        .filter(|s| s != &"")
        .map(|s| s.trim().parse::<i32>().expect("Parse error"));

    println!("{}", read_node(&mut parts));
    if parts.next() != None {
        panic!("Trailing cruft");
    }
}
