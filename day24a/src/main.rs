use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Side {
    Immune,
    Infection,
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Immune" => Ok(Side::Immune),
            "Infection" => Ok(Side::Infection),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Group {
    side: Side,
    unit_count: u64,
    hit_points: u64,
    immune_modifiers: HashSet<String>,
    weak_modifiers: HashSet<String>,
    attack_damage: u64,
    attack_type: String,
    initiative: u64,
}

fn read_immune_weak(str: &str) -> (HashSet<String>, HashSet<String>) {
    let mut immunes = HashSet::new();
    let mut weaks = HashSet::new();

    let mut current = 0;
    for part in str.split(' ') {
        match part {
            "weak" => current = 1,
            "immune" => current = 2,
            _ => {
                if current == 1 {
                    weaks.insert(part.to_string());
                } else if current == 2 {
                    immunes.insert(part.to_string());
                } else {
                    panic!("Unclassified modifier: {}", part);
                }
            }
        }
    }

    return (immunes, weaks);
}

fn read_group(str: &str) -> Group {
    // Normalise the separators, and split up.
    let parts = str.split(',').collect::<Vec<_>>();
        // .map(|s| s.trim().parse::<i32>().expect("Parse error"))
    assert_eq!(parts.len(), 7, "Wrong number of fields in: {}", str);
    let (immunes, weaks) = read_immune_weak(parts[3]);
    Group {
        side: parts[0].parse().unwrap(),
        unit_count: parts[1].parse().unwrap(),
        hit_points: parts[2].parse().unwrap(),
        immune_modifiers: immunes,
        weak_modifiers: weaks,
        attack_damage: parts[4].parse().unwrap(),
        attack_type: parts[5].to_string(),
        initiative: parts[6].parse().unwrap(),
    }
}



fn main() {
    let stdin = io::stdin();
    let groups: Vec<Group> = stdin
        .lock()
        .lines()
        .map(|s| read_group(&s.expect("Read error")))
        .collect();

    for group in groups.iter() {
        println!("{:?}", group);
    }
}
