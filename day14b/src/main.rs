use std::collections::VecDeque;

fn main() {
    let mut v: Vec<u8> = vec![3, 7];
    let mut idx1: usize = 0;
    let mut idx2: usize = 1;

    let mut target_tracker = VecDeque::new();
    let target: Vec<u8> = vec![5, 8, 0, 7, 4, 1];
    // let target = vec![5,9,4,1,4];

    while target_tracker != target {
        let score1 = v[idx1];
        let score2 = v[idx2];
        let sum = score1 + score2;
        if sum >= 10 {
            v.push(sum / 10);
            target_tracker.push_back(sum / 10);
            // Don't miss this case!
            while target_tracker.len() > target.len() {
                target_tracker.pop_front();
            }
            if target_tracker == target {
                break;
            }
        }
        v.push(sum % 10);
        target_tracker.push_back(sum % 10);
        idx1 = (idx1 + score1 as usize + 1) % v.len();
        idx2 = (idx2 + score2 as usize + 1) % v.len();
        // println!("{:?} {} {}", v, idx1, idx2);
        while target_tracker.len() > target.len() {
            target_tracker.pop_front();
        }
    }

    println!("{}", v.len() - target.len());
}
