use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

const TURN_LEFT_NEXT: i32 = 0;
const GO_STRAIGHT_NEXT: i32 = 1;
const TURN_RIGHT_NEXT: i32 = 2;

#[derive(Debug)]
struct Train {
    x: usize,
    y: usize,
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
                     trains.push(Train { x: x, y: y, facing: Facing::Down, step: TURN_LEFT_NEXT});
                 }
                 '^' => {
                     trains.push(Train { x: x, y: y, facing: Facing::Up, step: TURN_LEFT_NEXT});
                 }
                 '<' => {
                     trains.push(Train { x: x, y: y, facing: Facing::Left, step: TURN_LEFT_NEXT});
                 }
                 '>' => {
                     trains.push(Train { x: x, y: y, facing: Facing::Right, step: TURN_LEFT_NEXT});
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

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    let mut trains = extract_trains(&mut grid);

    print_track(&grid);
    println!("{:?}", trains)
}
