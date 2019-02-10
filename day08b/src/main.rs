use std::io;
use std::io::Read;

fn read_node<I>(iter: &mut I) -> i32
where
    I: Iterator<Item = i32>,
{
    let num_children = iter.next().unwrap();
    let num_metadata = iter.next().unwrap();

    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(read_node(iter));
    }
    let mut sum = 0;
    if num_children > 0 {
        for _ in 0..num_metadata {
            let idx = iter.next().unwrap() - 1;
            if 0 <= idx && idx < num_children {
                sum += children[idx as usize];
            }
        }
    } else {
        for _ in 0..num_metadata {
            sum += iter.next().unwrap();
        }
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
