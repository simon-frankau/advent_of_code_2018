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
        let instr = instr
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        TestCase {
            before: before
                .trim()
                .replace("Before: [", "")
                .replace("]", "")
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
            after: after
                .trim()
                .replace("After:  [", "")
                .replace("]", "")
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
            opcode: instr[0],
            a: instr[1],
            b: instr[2],
            c: instr[3],
        }
    }
}

enum Opcode {
    Addr, // (add register) stores into register C the result of adding register A and register B.
    Addi, // (add immediate) stores into register C the result of adding register A and value B.
    Mulr, // (multiply register) stores into register C the result of multiplying register A and register B.
    Muli, // (multiply immediate) stores into register C the result of multiplying register A and value B.
    Banr, // (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Bani, // (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Borr, // (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Bori, // (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Setr, // (set register) copies the contents of register A into register C. (Input B is ignored.)
    Seti, // (set immediate) stores value A into register C. (Input B is ignored.)
    Gtir, // (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtri, // (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtrr, // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Eqir, // (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqri, // (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqrr, // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

fn to_i(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

fn step(regs: &mut Vec<usize>, opcode: Opcode, a: usize, b: usize, c: usize) {
    regs[c] = match opcode {
        Opcode::Addr => regs[a] + regs[b],
        Opcode::Addi => regs[a] + b,
        Opcode::Mulr => regs[a] * regs[b],
        Opcode::Muli => regs[a] * b,
        Opcode::Banr => regs[a] & regs[b],
        Opcode::Bani => regs[a] & b,
        Opcode::Borr => regs[a] | regs[b],
        Opcode::Bori => regs[a] | b,
        Opcode::Setr => regs[a],
        Opcode::Seti => a,
        Opcode::Gtir => to_i(a > regs[b]),
        Opcode::Gtri => to_i(regs[a] > b),
        Opcode::Gtrr => to_i(regs[a] > regs[b]),
        Opcode::Eqir => to_i(a == regs[b]),
        Opcode::Eqri => to_i(regs[a] == b),
        Opcode::Eqrr => to_i(regs[a] == regs[b]),
    }
}

fn does_opcode_work(test_case: &TestCase, opcode: Opcode) -> bool {
    let mut regs = test_case.before.clone();
    step(&mut regs, opcode, test_case.a, test_case.b, test_case.c);
    regs == test_case.after
}

// Count the number of opcodes that transform the state to match the test case.
fn count_candidates(test_case: &TestCase) -> usize {
    vec![
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ].into_iter()
    .map(|opcode| does_opcode_work(test_case, opcode))
    .filter(|x| *x)
    .count()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

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

    let mut case_count = 0;
    for c in cases.iter() {
        let count = count_candidates(c);
        println!("{:?} -> {}", c, count);
        if count >= 3 {
            case_count += 1;
        }
    }
    println!("... {}", case_count);
}
