use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct TestCase {
    before: Vec<usize>,
    after: Vec<usize>,
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl TestCase {
    fn from(before: &str, after: &str, instr: &str) -> TestCase {
        let instr = instr.trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        TestCase {
            before: before.trim().replace("Before: [", "").replace("]", "").split(", ").map(|x| x.parse().unwrap()).collect(),
            after: after.trim().replace("After:  [", "").replace("]", "").split(", ").map(|x| x.parse().unwrap()).collect(),
            opcode: instr[0],
            a: instr[1],
            b: instr[2],
            c: instr[3],
        }
    }

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(Result::unwrap);

    let mut cases = Vec::new();

    loop {
        let before = lines.next().unwrap();
        if !before.contains("Before") {
            break;
        }
        let instr = lines.next().unwrap();
        let after = lines.next().unwrap();
        lines.next();
        cases.push(TestCase::from(&before, &after, &instr))
    }

    println!("{:?}", cases);
}
