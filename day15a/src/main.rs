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

fn print_units(grid: &Vec<Vec<Square>>) {
    for row in grid.iter() {
        for col in row.iter() {
            if let Square::Unit(u) = col {
                println!("{:?}: {}", u.species, u.hp);
            }
        }
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

// Moves a unit and returns its new location
fn move_unit(grid: &mut Vec<Vec<Square>>, x: usize, y: usize) -> (usize, usize) {
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
        grid[new_y][new_x] = grid[y][x];
        grid[y][x] = Square::Space;
        return (new_x, new_y);
    }
    return (x, y);
}

// Performs combat.
fn attack_with_unit(grid: &mut Vec<Vec<Square>>, x: usize, y: usize) {
    let target = match Move::get_species(&grid, x, y).unwrap() {
        Species::Gnome => Species::Elf,
        Species::Elf => Species::Gnome,
    };

    let candidates = vec![
        (y - 1, x),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x),
    ];

    let mut targets = candidates
        .iter()
        .map(|(y, x)| (grid[*y][*x], *y, *x))
        .filter(|(sq, _, _)| if let Square::Unit(u) = sq {
            u.species == target
        } else {
            false
        })
        .collect::<Vec<_>>();
    // Sort is stable, so if HPs match we use position
    targets.sort_by(|(a, _, _), (b, _, _)| if let (Square::Unit(au), Square::Unit(bu)) = (a, b) {
        au.hp.cmp(&bu.hp)
    } else {
        panic!("Not a unit?!")
    });
    // println!("Targets: {:?}", targets);

    // If there's a target, let's attack!
    if let Some((_, ty, tx)) = targets.iter().next() {
        let target = &mut grid[*ty][*tx];
        // println!("We have a target: {:?}", target);

        // Not sure how to update through a reference, so make a copy
        // of the struct and then copy it back...
        if let Square::Unit(mut u) = target {
            // Cheat: Rather than look up our AP, reduce by AP of target.
            if u.hp <= u.attack {
                 *target = Square::Space;
            } else {
                 u.hp -= u.attack;
                 *target = Square::Unit(u);
            }
        }
    }
}

// Run through the entire grid, moving pieces and doing combat.
fn update_all(grid: &mut Vec<Vec<Square>>) {
    // As we're updating the grid as we go, don't move the units we've
    // already moved, if we scan over them again.
    let mut moved_already = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if moved_already.contains(&(x, y)) {
                continue;
            }

            if let Square::Unit(_) = grid[y][x] {
                let (x, y) = move_unit(grid, x, y);
                moved_already.insert((x, y));
                attack_with_unit(grid, x, y);
            }
        }
    }
}

fn is_complete(grid: &Vec<Vec<Square>>) -> bool {
    let mut elves = 0;
    let mut gnomes = 0;
    for row in grid.iter() {
        for col in row.iter() {
            if let Square::Unit(u) = col {
                if u.species == Species::Elf {
                    elves += 1;
                } else {
                    gnomes +=1 ;
                }
            }
        }
    }
    elves == 0 || gnomes == 0
}

fn sum_hp(grid: &Vec<Vec<Square>>) -> usize {
    let mut hp = 0;
    for row in grid.iter() {
        for col in row.iter() {
            if let Square::Unit(u) = col {
                hp += u.hp;
            }
        }
    }
    hp as usize
}

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<Square>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(Square::from).collect())
        .collect();

    print_grid(&grid);
    print_units(&grid);
    let mut round = 0;
    while !is_complete(&grid) {
        round += 1;
        println!("\nRound {}", round);
        update_all(&mut grid);
        print_grid(&grid);
        print_units(&grid);
    }
    let hp = sum_hp(&grid);
    // For all the examples but the first, it seems the round count is one less??
    println!("{} * {} = {}", round - 1, hp, (round - 1) * hp);
}
