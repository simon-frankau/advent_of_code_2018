use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let parts: Vec<i32> = buffer
        .split('\n')
        .filter(|s| s != &"")
        .map(|s| s.trim().parse::<i32>().expect("Parse error"))
        .collect();

    let mut seen = HashSet::new();
    let mut running_sum = 0;
    loop {
        for part in parts.iter() {
        running_sum += part;
            if !seen.insert(running_sum) {
                println!("{}", running_sum);
                return;
            }
        }
    }
}
