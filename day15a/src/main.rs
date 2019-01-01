use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Species {
    Elf,
    Gnome,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
