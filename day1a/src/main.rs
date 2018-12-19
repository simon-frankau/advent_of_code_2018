use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let parts = buffer.split('\n').filter(|s| s != &"").map(|s| s.trim().parse::<i32>().expect("Parse error"));

    let mut running_sum = 0;
    for part in parts {
        running_sum += part;
    }

    println!("{}", running_sum);
}
