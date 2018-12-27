use std::collections::HashSet;
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
                 'v' => {
                     trains.push(Train { y: y, x: x, facing: Facing::Down, step: TURN_LEFT_NEXT});
                 }
                 '^' => {
                     trains.push(Train { y: y, x: x, facing: Facing::Up, step: TURN_LEFT_NEXT});
                 }
                 '<' => {
                     trains.push(Train { y: y, x: x, facing: Facing::Left, step: TURN_LEFT_NEXT});
                 }
                 '>' => {
                     trains.push(Train { y: y, x: x, facing: Facing::Right, step: TURN_LEFT_NEXT});
                 }
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

fn check_collisions(trains: &Vec<Train>, x: usize, y: usize) -> Result<(), String> {
    // This is inefficient, but I think it's ok for a small number of trains.
    for train in trains.iter() {
        if train.x == x && train.y == y {

        }
    }
    Ok(())
}

fn step_trains(grid: &Vec<Vec<char>>, trains: &mut Vec<Train>) -> Result<(), String> {
    trains.sort();

    let mut locs = trains.iter().map(|t| (t.x, t.y)).collect::<HashSet<_>>();

    for train in trains.iter_mut() {
        // Update position
        {
            let mut x = train.x;
            let mut y = train.y;
            match train.facing {
                 Facing::Right => x += 1,
                 Facing::Down => y += 1,
                 Facing::Left => x -= 1,
                 Facing::Up => y -= 1,
            }
            if locs.contains(&(x, y)) {
                return Err(format!("Train collision at {}, {}", x, y));
            }
            locs.remove(&(train.x, train.y));
            train.x = x;
            train.y = y;
            locs.insert((x, y));
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

            ('+', _) => panic!("TODO: Junction at {}, {}", train.x, train.y),

            (c, _) => {
                panic!("Unexpected state: {:?} on {}", train, c);
            }
         }
    }

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

    print_track(&grid);
    println!("{:?}", trains);
    loop {
        step_trains(&grid, &mut trains);
        println!("{:?}", trains);
    }
}
