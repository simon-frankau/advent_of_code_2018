use std::collections::VecDeque;
use std::iter::once;
use std::io;
use std::io::BufRead;

fn print_grid(grid: &VecDeque<Vec<char>>) {
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn cell_step(grid: &VecDeque<Vec<char>>, x: usize, y: usize) -> char {
    let neighbours = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let neighbour_cells: Vec<char> = neighbours.iter().map(|(ox, oy)| grid[(y as isize + oy) as usize][(x as isize + ox) as usize]).collect::<Vec<char>>();
    match grid[y][x] {
        '.' => {
            if neighbour_cells.iter().filter(|x| **x == '|').count() >= 3 {
                '|'
            } else {
                '.'
            }
        }
        '|' => {
            if neighbour_cells.iter().filter(|x| **x == '#').count() >= 3 {
                '#'
            } else {
                '|'
            }
        }
        '#' => {
            if neighbour_cells.iter().filter(|x| **x == '#').count() >= 1 &&
                neighbour_cells.iter().filter(|x| **x == '|').count() >= 1 {
                '#'
            } else {
                '.'
            }
        }
        c => panic!("Unexpected character: {}", c),
    }
}


// One timestep. We could do clever implementations with
// iterators and stuff, but... let's just keep it simple.
fn step(grid: &VecDeque<Vec<char>>) -> VecDeque<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = grid.clone();

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            new_grid[y][x] = cell_step(grid, x, y);
        }
    }

    new_grid
}

fn score(grid: &VecDeque<Vec<char>>) {
    let mut woods = 0;
    let mut lumberyards = 0;
    for row in grid.iter() {
        for c in row.iter() {
            match c {
                '|' => woods += 1,
                '#' => lumberyards += 1,
                _ => {},
            }
        }
    }
    println!("{} * {} = {}", woods, lumberyards, woods * lumberyards);
}

fn main() {
    // Cheesily, we'll keep in character domain. We use VecDeque to
    // allow us to build a sentinel row around the edge.
    let stdin = io::stdin();
    let mut grid = stdin
        .lock()
        .lines()
        .map(|x| once('.').chain(x.unwrap().chars()).chain(once('.')).collect::<Vec<_>>())
        .collect::<VecDeque<_>>();
    let row_len = grid.front().unwrap().len();
    grid.push_front(vec!['.';row_len]);
    grid.push_back(vec!['.';row_len]);

    print_grid(&grid);
    for _ in 0..10 {
        grid = step(&grid);
        println!("");
        print_grid(&grid);
    }
    score(&grid);
}
