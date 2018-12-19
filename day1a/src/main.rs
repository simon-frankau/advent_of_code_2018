use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let parts = buffer.split('\n').filter(|s| s != &"").map(|s| s.trim().parse::<i32>().expect("Parse error"));

    let mut seen = HashSet::new();
    let mut cycled = false;

    let mut running_sum = 0;
    for part in parts {
        running_sum += part;
        if !cycled {
            if !seen.insert(running_sum) {
                println!("Cycle: {}", running_sum);
                cycled = true;
            }
        }
    }

    println!("Running sum: {}", running_sum);
}
