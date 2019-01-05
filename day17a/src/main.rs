use std::io;
use std::io::BufRead;

const START_X: usize = 500;

#[derive(Debug)]
struct Line {
    const_val: usize,
    min_val: usize,
    max_val: usize,
    is_vertical: bool,
}

impl Line {
    fn new(s: &str) -> Line {
        let s = s.replace('=', ":").replace(", ", ":").replace("..", ":");
        let mut parts = s.split(":");
        let is_vertical = parts.next().unwrap() == "x";
        let const_val = parts.next().unwrap().parse().unwrap();
        parts.next().unwrap(); // Skip other direction variable.
        let min_val = parts.next().unwrap().parse().unwrap();
        let max_val = parts.next().unwrap().parse().unwrap();
        Line {
            const_val: const_val,
            min_val: min_val,
            max_val: max_val,
            is_vertical: is_vertical,
        }
    }
}

// Returns x range, y range - inclusive in both direction.
fn get_extents(lines: &[Line]) -> (usize, usize, usize, usize) {
    let min_x = lines
        .iter()
        .map(|l| {
            if l.is_vertical {
                l.const_val
            } else {
                l.min_val
            }
        }).min().unwrap();
    let max_x = lines
        .iter()
        .map(|l| {
            if l.is_vertical {
                l.const_val
            } else {
                l.max_val
            }
        }).max().unwrap();
    let min_y = lines
        .iter()
        .map(|l| {
            if !l.is_vertical {
                l.const_val
            } else {
                l.min_val
            }
        }).min().unwrap();
    let max_y = lines
        .iter()
        .map(|l| {
            if !l.is_vertical {
                l.const_val
            } else {
                l.max_val
            }
        }).max().unwrap();
    (min_x, max_x, min_y, max_y)
}

#[derive(Debug, Clone, PartialEq)]
enum Square {
    Sand,
    Clay,
    Flowing,
    Still,
}

struct Reservoir {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    grid: Vec<Vec<Square>>,
}

impl Reservoir {
    fn new(extents: (usize, usize, usize, usize)) -> Reservoir {
        let (min_x, max_x, min_y, max_y) = extents;
        // Extend the level slightly to the sides to avoid the need
        // for boundary checks on the sides.
        let min_x = min_x - 2;
        let max_x = max_x + 2;

        let x_size = max_x - min_x + 1;
        let y_size = max_y - min_y + 1;
        Reservoir {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            grid: (0..y_size).map(|_| vec![Square::Sand;x_size]).collect(),
        }
    }

    fn sq(&mut self, x: usize, y: usize) -> &mut Square {
        &mut self.grid[y - self.min_y][x - self.min_x]
    }

    fn add(&mut self, line: &Line) {
        if line.is_vertical {
            for y in line.min_val..line.max_val + 1 {
                *self.sq(line.const_val, y) = Square::Clay;
            }
        } else {
            for x in line.min_val..line.max_val + 1 {
                *self.sq(x, line.const_val) = Square::Clay;
            }
        }
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                print!("{}", match cell {
                    Square::Sand => '.',
                    Square::Clay => '#',
                    Square::Flowing => '|',
                    Square::Still => '~',
                });
            }
            println!("");
        }
    }

    // Pour water into the given square.
    fn pour(&mut self, x: usize, y: usize) {
        // Can only flow into sand.
        if *self.sq(x, y) != Square::Sand {
            return;
        }
        // Always flow out the bottom.
        if y == self.max_y {
            *self.sq(x, y) = Square::Flowing;
            return;
        }

        // Flow left and right, favouring downwards
        let mut outflowing = false;
        let mut lx = x;
        while *self.sq(lx, y) == Square::Sand {
            *self.sq(lx, y) = Square::Flowing;
            self.pour(lx, y + 1);
            if *self.sq(lx, y + 1) == Square::Flowing {
                outflowing = true;
                break;
            }
            lx -= 1;
        }
        let mut rx = x;
        // Messy corner-case-ness. Can't think how to make this pretty.
        *self.sq(rx, y) = Square::Sand;
        while *self.sq(rx, y) == Square::Sand {
            *self.sq(rx, y) = Square::Flowing;
            self.pour(rx, y + 1);
            if *self.sq(rx, y + 1) == Square::Flowing {
                outflowing = true;
                break;
            }
            rx += 1;
        }

        if !outflowing {
            for sx in lx + 1..rx {
                *self.sq(sx, y) = Square::Still;
            }
        }

        // self.print();
        // println!("");
    }

    fn count_water_row(row: &Vec<Square>) -> usize {
        row.iter().map(|x| if *x == Square::Flowing || *x == Square::Still {
                1
            } else {
                0
            })
            .sum()
    }

    fn count_water(&self) -> usize {
        self.grid.iter().map(Reservoir::count_water_row).sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| Line::new(&l.unwrap()))
        .collect::<Vec<_>>();

    let mut reservoir = Reservoir::new(get_extents(&lines));
    for line in lines.iter() {
        reservoir.add(line);
    }
    let min_y = reservoir.min_y;
    reservoir.pour(START_X, min_y);
    reservoir.print();
    println!("{}", reservoir.count_water());
}
