use std::io;
use std::io::BufRead;
use std::str::FromStr;

const NUM_REGS: usize = 6;

#[derive(Debug)]
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

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => return Err(String::from("Parse error")),
        })
    }
}

fn to_i(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

#[derive(Debug)]
struct Instr {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instr {
    fn from(instr: &str) -> Instr {
        let instr = instr.trim().split(' ').collect::<Vec<_>>();
        Instr {
            opcode: instr[0].parse().unwrap(),
            a: instr[1].parse().unwrap(),
            b: instr[2].parse().unwrap(),
            c: instr[3].parse().unwrap(),
        }
    }

    fn step(&self, regs: &mut Vec<usize>) {
        regs[self.c] = match self.opcode {
            Opcode::Addr => regs[self.a] + regs[self.b],
            Opcode::Addi => regs[self.a] + self.b,
            Opcode::Mulr => regs[self.a] * regs[self.b],
            Opcode::Muli => regs[self.a] * self.b,
            Opcode::Banr => regs[self.a] & regs[self.b],
            Opcode::Bani => regs[self.a] & self.b,
            Opcode::Borr => regs[self.a] | regs[self.b],
            Opcode::Bori => regs[self.a] | self.b,
            Opcode::Setr => regs[self.a],
            Opcode::Seti => self.a,
            Opcode::Gtir => to_i(self.a > regs[self.b]),
            Opcode::Gtri => to_i(regs[self.a] > self.b),
            Opcode::Gtrr => to_i(regs[self.a] > regs[self.b]),
            Opcode::Eqir => to_i(self.a == regs[self.b]),
            Opcode::Eqri => to_i(regs[self.a] == self.b),
            Opcode::Eqrr => to_i(regs[self.a] == regs[self.b]),
        }
    }

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let ip: usize = lines.next().unwrap().replace("#ip ", "").parse().unwrap();;
    let instrs = lines.map(|x| Instr::from(&x)).collect::<Vec<_>>();

    // Day 19a
    let mut regs = vec![0; NUM_REGS];

    while regs[ip] < instrs.len() {
        instrs[regs[ip]].step(&mut regs);
        regs[ip] += 1;
    }

    println!("{:?}", regs);

    // Day 19b
    // Turns out this runs for a very long time. Either I've a bug or it's
    // time to reverse the code and skip to the end...
/*
    let mut regs = vec![0; NUM_REGS];
    regs[0] = 1;

    while regs[ip] < instrs.len() {
        instrs[regs[ip]].step(&mut regs);
        regs[ip] += 1;
    }

    println!("{:?}", regs);
*/
}
