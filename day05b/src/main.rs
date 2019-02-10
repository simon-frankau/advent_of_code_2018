use std::io;
use std::io::Read;

fn score(str: &str) -> i32 {
    let mut data = Vec::new();
    data.push('^');
    for c in str.trim().chars() {
        data.push(c);
    }
    data.push('$');

    // Initialise like this:
    //
    // prev: -1 0  1  2  3
    // data: ^  X  Y  Z  $
    // next: 1  2  3  4  -1
    //
    // Yes, I'm a bad person for using '-1' instead of Option.

    let n = data.len() as i32;
    let mut next = (1..n + 1).collect::<Vec<_>>();
    next[(n - 1) as usize] = -1;
    let mut prev = (-1..n - 1).collect::<Vec<_>>();

    // And now traverse the "list", looking for pairs to elide. We
    // always elide he current-pointed-to item and the next item.

    let mut p: i32 = 0;
    while next[p as usize] != -1 {
        let next_p = next[p as usize];
        let c = data[p as usize];
        let next_c = data[next_p as usize];

        if c.to_ascii_uppercase() == next_c.to_ascii_uppercase() && c != next_c {
            // Candidate for reaction!
            let forward = next[next_p as usize];
            let backward = prev[p as usize];
            next[backward as usize] = forward;
            prev[forward as usize] = backward;
            p = backward;
        } else {
            // Nothing, move on.
            p = next_p;
        }
    }

    // Get the length. Could be merged into previous loop, but this is
    // simpler.

    let mut l = 0;
    p = 0;
    while p != -1 {
        l += 1;
        p = next[p as usize];
    }

    // Remove sentinels from count.
    l - 2
}

fn main() {
    // Linked list would be the best representation. For simplicity,
    // let's fake it with arrays of "pointers". Fortran-style at its
    // best. We'll put in ^sentinels$ at either end of the array.

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");

    let (c, _) = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| {
            (
                score(&buffer.replace(x, "").replace(x.to_ascii_uppercase(), "")),
                x,
            )
        }).min()
        .expect("Can't happen");

    println!("{:?}", c);
}
