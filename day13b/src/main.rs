use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

const TURN_LEFT_NEXT: i32 = 0;
const GO_STRAIGHT_NEXT: i32 = 1;
const TURN_RIGHT_NEXT: i32 = 2;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
// NB: Field ordering used for sorting into correct update order.
struct Train {
    y: usize,
    x: usize,
    facing: Facing,
    step: i32,
}

impl Train {
    fn new(x: usize, y: usize, facing: Facing) -> Train {
        Train {
            y: y,
            x: x,
            facing: facing,
            step: TURN_LEFT_NEXT,
        }
    }
}

fn extract_trains(grid: &mut Vec<Vec<char>>) -> Vec<Train> {
    // Find all the trains.
    let mut trains = Vec::new();
    for (row, y) in grid.iter().zip(0..) {
        for (c, x) in row.iter().zip(0..) {
            match c {
                ' ' => {}
                '-' => {}
                '|' => {}
                '+' => {}
                '/' => {}
                '\\' => {}
                'v' => trains.push(Train::new(x, y, Facing::Down)),
                '^' => trains.push(Train::new(x, y, Facing::Up)),
                '<' => trains.push(Train::new(x, y, Facing::Left)),
                '>' => trains.push(Train::new(x, y, Facing::Right)),
                _ => {
                    panic!("Unexpected input: {}", c);
                }
            }
        }
    }

    // Remove all the trains from the grid to simplify processing...
    for train in trains.iter() {
        let replacement = match train.facing {
            Facing::Right => '-',
            Facing::Down => '|',
            Facing::Left => '-',
            Facing::Up => '|',
        };
        grid[train.y][train.x] = replacement;
    }

    trains
}

fn print_track(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!("");
    }
}

fn new_facing(facing: Facing, step: i32) -> Facing {
    match (step, facing) {
        (TURN_LEFT_NEXT, Facing::Right) => Facing::Up,
        (TURN_LEFT_NEXT, Facing::Down) => Facing::Right,
        (TURN_LEFT_NEXT, Facing::Left) => Facing::Down,
        (TURN_LEFT_NEXT, Facing::Up) => Facing::Left,
        (GO_STRAIGHT_NEXT, facing) => facing,
        (TURN_RIGHT_NEXT, Facing::Right) => Facing::Down,
        (TURN_RIGHT_NEXT, Facing::Down) => Facing::Left,
        (TURN_RIGHT_NEXT, Facing::Left) => Facing::Up,
        (TURN_RIGHT_NEXT, Facing::Up) => Facing::Right,
        (_, _) => panic!("Nope, shouldn't happen"),
    }
}

fn step_trains(grid: &Vec<Vec<char>>, trains: &mut Vec<Train>) -> Result<(), String> {
    trains.sort();

    // Number of trains per location.
    let mut locs = trains.iter().map(|t| ((t.x, t.y), 1)).collect::<HashMap<_, _>>();

    for train in trains.iter_mut() {
        // Update position
        {
            let mut x = train.x;
            let mut y = train.y;

            // Don't move a train if it's already crashed.
            if *locs.get(&(x, y)).unwrap() > 1 {
                continue;
            }

            match train.facing {
                Facing::Right => x += 1,
                Facing::Down => y += 1,
                Facing::Left => x -= 1,
                Facing::Up => y -= 1,
            }

            *(locs.get_mut(&(train.x, train.y)).unwrap()) -= 1;
            train.x = x;
            train.y = y;
            *(locs.entry((x, y)).or_insert(0)) += 1;
        }

        match (grid[train.y][train.x], train.facing) {
            (' ', _) => panic!("Fell off the rails: {:?}", train),
            ('-', _) => {}
            ('|', _) => {}

            ('/', Facing::Left) => train.facing = Facing::Down,
            ('/', Facing::Right) => train.facing = Facing::Up,
            ('/', Facing::Up) => train.facing = Facing::Right,
            ('/', Facing::Down) => train.facing = Facing::Left,

            ('\\', Facing::Left) => train.facing = Facing::Up,
            ('\\', Facing::Right) => train.facing = Facing::Down,
            ('\\', Facing::Up) => train.facing = Facing::Left,
            ('\\', Facing::Down) => train.facing = Facing::Right,

            ('+', f) => {
                train.facing = new_facing(f, train.step);
                train.step = (train.step + 1) % 3;
            }

            (c, _) => {
                panic!("Unexpected state: {:?} on {}", train, c);
            }
        }
    }

    // Remove crashed trains.
    trains.retain(|t| locs[&(t.x, t.y)] < 2);

    Ok(())
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    let mut trains = extract_trains(&mut grid);

    // print_track(&grid);
    // println!("{:?}", trains);
    while trains.len() > 1 {
        step_trains(&grid, &mut trains).unwrap();
        // println!("{} {:?}", trains.len(), trains);
    }
    let t = trains.iter().next().unwrap();
    println!("{},{}", t.x, t.y);
}
