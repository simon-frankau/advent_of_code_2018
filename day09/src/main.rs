fn insert(curr: &mut usize, next: &mut Vec<usize>, prev: &mut Vec<usize>) -> usize {
    let val = next.len();

    if val % 23 == 0 {
        // Special case time!

        let mut to_remove = *curr;
        for _ in 0..7 {
            to_remove = prev[to_remove];
        }

        next[prev[to_remove]] = next[to_remove];
        prev[next[to_remove]] = prev[to_remove];
        *curr = next[to_remove];

        prev.push(0);
        next.push(0);

        return val + to_remove;
    }

    // Get points either side of insertion point.
    let before = next[*curr];
    let after = next[before];
    // Update them to point at our new node.
    next[before] = val;
    prev[after] = val;
    // Fill in links of our new node.
    prev.push(before);
    next.push(after);
    // Update pointer.
    *curr = val;
    0
}

// Could build an iterator, but I'm lazy.
fn _print_cycle(next: &Vec<usize>) {
    let mut curr = 0;
    loop {
        print!("{} ", curr);
        curr = next[curr];
        if curr == 0 {
            println!("");
            return;
        }
    }
}

fn play(players: usize, max_marble: usize) -> usize {
    // Let's use vectors as next/prev pointers into an array, again.
    let mut next = Vec::new();
    let mut prev = Vec::new();

    next.push(0);
    prev.push(0);
    let mut curr = 0;

    let mut scores = vec![0; players];

    for i in 0..max_marble + 1 {
        let score = insert(&mut curr, &mut next, &mut prev);
        if score > 0 {
            // println!("Score: {} to {}", score, i % players);
            scores[i % players] += score;
        }
        // print_cycle(&next);
    }

    let max_score = scores.iter().max().unwrap();
    println!(
        "Players: {} Max marble: {} High score: {}",
        players, max_marble, max_score
    );
    *max_score
}

fn main() {
    play(10, 1618);
    play(13, 7999);
    play(17, 1104);
    play(21, 6111);
    play(30, 5807);
    play(430, 71588);
    play(430, 7158800);
}
