const COUNT: usize = 580741;
const DISPLAY: usize = 10;

fn main() {
    // This was more complicated than expected, as I thought I'd need
    // to parse a more complicated string. Ooops.
    let mut v = vec![3, 7];
    let mut idx1 = 0;
    let mut idx2 = 1;

    let target = COUNT + DISPLAY;

    while v.len() < target {
        let score1 = v[idx1];
        let score2 = v[idx2];
        let sum = score1 + score2;
        if sum >= 10 {
            v.push(sum / 10);
        }
        v.push(sum % 10);
        idx1 = (idx1 + score1 + 1) % v.len();
        idx2 = (idx2 + score2 + 1) % v.len();
        // println!("{:?} {} {}", v, idx1, idx2);
    }

    while v.len() > target {
        v.pop();
    }

    // Slightly messy way of getting last 10 while avoiding indexing
    // calculations. :)
    let items = v.iter().rev().take(DISPLAY).collect::<Vec<_>>();
    for i in items.iter().rev() {
        print!("{}", i);
    }
    println!("");
}
