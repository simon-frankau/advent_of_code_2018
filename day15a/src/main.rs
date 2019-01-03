use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Species {
    Elf,
    Gnome,
}

#[derive(Debug, PartialEq)]
struct Unit {
    species: Species,
    attack: u32,
    hp: u32,
}

impl Unit {
    fn new(species: Species) -> Unit {
        Unit {
            species: species,
            attack: 3,
            hp: 200,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Square {
    Wall,
    Space,
    Unit(Unit),
}

impl Square {
    fn from(c: char) -> Square {
        match c {
            '.' => Square::Space,
            '#' => Square::Wall,
            'E' => Square::Unit(Unit::new(Species::Elf)),
            'G' => Square::Unit(Unit::new(Species::Gnome)),
            _  => panic!("Unknown character: {}", c),
        }
    }

    fn disp(&self) -> char {
        match self {
            Square::Wall => '#',
            Square::Space => '.',
            Square::Unit(u) => {
                match u.species {
                    Species::Elf => 'E',
                    Species::Gnome => 'G',
                }
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<Square>>) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col.disp());
        }
        println!("");
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

impl Move {
    fn get_species(grid: &Vec<Vec<Square>>, x: usize, y: usize) -> Option<Species> {
        if let Square::Unit(ref u) = grid[y][x] {
            Some(u.species)
        } else {
            None
        }
    }

    fn neighbours(seen: &mut HashSet<(usize, usize)>, grid: &Vec<Vec<Square>>, x: usize, y: usize) -> Vec<(usize, usize, Move)> {
       // Use the fact there's a perimeter on the map to prevent underflow.
       let mut candidates = vec![(y-1, x, Move::Up), (y, x-1, Move::Left), (y, x+1, Move::Right), (y+1, x, Move::Down)];
       let result = candidates.into_iter().filter(|(y, x, _)| grid[*y][*x] != Square::Wall && !seen.contains(&(*x,*y))).collect::<Vec<_>>();
       for (y, x, _) in result.iter() {
           seen.insert((*x, *y));
       }
       result
    }

    fn find(grid: &Vec<Vec<Square>>, x: usize, y: usize) -> Option<Move> {
        let target = match Move::get_species(&grid, x, y).unwrap() {
            Species::Gnome => Species::Elf,
            Species::Elf => Species::Gnome,
        };
        // All squares we've already reached.
        let mut seen = HashSet::new();
        // All squares on the current distance frontier.
        let mut frontier = Move::neighbours(&mut seen, &grid, x, y);
        while !frontier.is_empty() {
            {
                let mut targets = frontier.iter().filter(|(y, x, _)| Move::get_species(grid, *x, *y) == Some(target)).collect::<Vec<_>>();
                if !targets.is_empty() {
                    // We can reach some target. We'll choose the one
                    // that's most top-left, and then tie break on most
                    // top-left starting movement direction.
                    targets.sort();
                    // Return the direction to move.
                    return Some(targets[0].2);
                }
            }

            // No target reachable at this distance. Build the set of
            // points we can reach. As the initial list of neighbours
            // was sorted, we'll always favour the top-left starting
            // direction.
            let mut new_frontier = Vec::new();
            for (y, x, original_move) in frontier.iter() {
                for (new_y, new_x, _) in Move::neighbours(&mut seen, &grid, *x, *y).iter() {
                    new_frontier.push((*new_y, *new_x, *original_move));
                }
            }
            // Is this O(1)? Who knows! I have much to learn about Rust.
            std::mem::swap(&mut frontier, &mut new_frontier);
        }
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<Square>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(Square::from).collect())
        .collect();

    println!("{:?}", grid);

    print_grid(& grid);
}
