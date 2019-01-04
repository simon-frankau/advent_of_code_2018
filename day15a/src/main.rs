use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Species {
    Elf,
    Gnome,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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
            _ => panic!("Unknown character: {}", c),
        }
    }

    fn disp(&self) -> char {
        match self {
            Square::Wall => '#',
            Square::Space => '.',
            Square::Unit(u) => match u.species {
                Species::Elf => 'E',
                Species::Gnome => 'G',
            },
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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

    // We try to move *onto* the enemy, and then decide to never do
    // that final move later. I *think* it would be equivalent to just
    // find the top-left-est square next to an enemy, but I'm not sure if
    // there's nasty corner case bugs or not, so I'm avoiding that
    // optimisation.
    fn can_move_to(square: &Square, target: Species) -> bool {
        match square {
            Square::Wall => false,
            Square::Space => true,
            Square::Unit(u) => u.species == target,
        }
    }

    fn neighbours(
        seen: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<Square>>,
        target: Species,
        x: usize,
        y: usize,
    ) -> Vec<(usize, usize, Move)> {
        // Use the fact there's a perimeter on the map to prevent underflow.
        let candidates = vec![
            (y - 1, x, Move::Up),
            (y, x - 1, Move::Left),
            (y, x + 1, Move::Right),
            (y + 1, x, Move::Down),
        ];
        let result = candidates
            .into_iter()
            .filter(|(y, x, _)| {
                Move::can_move_to(&grid[*y][*x], target) && !seen.contains(&(*x, *y))
            }).collect::<Vec<_>>();
        for (y, x, _) in result.iter() {
            seen.insert((*x, *y));
        }
        result
    }

    fn find(grid: &Vec<Vec<Square>>, x: usize, y: usize) -> Option<Move> {
        // println!("Finding target for {} {}", x, y);
        let target = match Move::get_species(&grid, x, y).unwrap() {
            Species::Gnome => Species::Elf,
            Species::Elf => Species::Gnome,
        };
        // All squares we've already reached.
        let mut seen = HashSet::new();
        // All squares on the current distance frontier.
        let mut frontier = Move::neighbours(&mut seen, &grid, target, x, y);
        while !frontier.is_empty() {
            // println!("Frontier: {:?}", frontier);
            {
                let mut targets = frontier
                    .iter()
                    .filter(|(y, x, _)| Move::get_species(grid, *x, *y) == Some(target))
                    .collect::<Vec<_>>();
                if !targets.is_empty() {
                    // println!("Targets: {:?}", targets);
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
                for (new_y, new_x, _) in Move::neighbours(&mut seen, &grid, target, *x, *y).iter() {
                    new_frontier.push((*new_y, *new_x, *original_move));
                }
            }
            // Is this O(1)? Who knows! I have much to learn about Rust.
            std::mem::swap(&mut frontier, &mut new_frontier);
        }
        None
    }
}

// Run through the entire grid, moving pieces.
fn move_all(grid: &mut Vec<Vec<Square>>) {
    // As we're updating the grid as we go, don't move the units we've
    // already moved, if we scan over them again.
    let mut moved_already = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if moved_already.contains(&(x, y)) {
                continue;
            }

            if let Square::Unit(_) = grid[y][x] {
                let new_loc = match Move::find(&grid, x, y) {
                    Some(Move::Up) => Some((x, y - 1)),
                    Some(Move::Left) => Some((x - 1, y)),
                    Some(Move::Right) => Some((x + 1, y)),
                    Some(Move::Down) => Some((x, y + 1)),
                    None => None,
                };
                // "find" is willing to move onto the enemy. Don't do that.
                let new_loc = if let Some((x, y)) = new_loc {
                    if grid[y][x] == Square::Space {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some((new_x, new_y)) = new_loc {
                    // println!("{}, {} -> {}, {}", x, y, new_x, new_y);
                    moved_already.insert((new_x, new_y));
                    grid[new_y][new_x] = grid[y][x];
                    grid[y][x] = Square::Space;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<Square>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(Square::from).collect())
        .collect();

    print_grid(&grid);
    for i in 0..5 {
        move_all(&mut grid);
        print_grid(&grid);
    }
}
