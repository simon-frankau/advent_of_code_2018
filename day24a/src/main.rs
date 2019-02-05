use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
    id: u64,
    side: Side,
    unit_count: u64,
    hit_points: u64,
    immune_modifiers: HashSet<String>,
    weak_modifiers: HashSet<String>,
    attack_damage: u64,
    attack_type: String,
    initiative: u64,
}

impl Group {
    fn attack_multiplier(self: &Group, target: &Group) -> i64 {
        let t = &self.attack_type;
        if target.immune_modifiers.contains(t) {
            0
        } else if target.weak_modifiers.contains(t) {
            2
        } else {
            1
        }
    }
}

fn read_immune_weak(str: &str) -> (HashSet<String>, HashSet<String>) {
    let mut immunes = HashSet::new();
    let mut weaks = HashSet::new();

    let mut current = 0;
    for part in str.split(' ') {
        match part {
            "" => (),
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

fn read_group(str: &str, id: u64) -> Group {
    // Normalise the separators, and split up.
    let parts = str.split(',').collect::<Vec<_>>();
        // .map(|s| s.trim().parse::<i32>().expect("Parse error"))
    assert_eq!(parts.len(), 7, "Wrong number of fields in: {}", str);
    let (immunes, weaks) = read_immune_weak(parts[3]);
    Group {
        id: id,
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

// Return a number based on effective power, then initiative (assumes initiative < 100), to sort by.
fn power_rank(group: &&Group) -> i64 {
    0 - (group.unit_count * group.attack_damage * 100 + group.initiative) as i64
}

fn select_targets(groups: &[Group]) -> HashMap<u64, Option<u64>> {
    // Sort groups by effective power
    let mut group_refs = groups.iter().collect::<Vec<&Group>>();
    group_refs.sort_by_key(power_rank);

    // Targets that have been chosen already, and can't be re-selected.
    let mut chosen: HashSet<u64> = HashSet::new();
    group_refs.iter().map(|attacker| {
        // Use rev so that we get the highest effective power in a
        // draw (max_by_key returns the last of equals)
        let target = groups.iter()
            .rev()
            .filter(|target| target.side != attacker.side && target.unit_count != 0 && !chosen.contains(&target.id))
            .max_by_key(|target| attacker.attack_multiplier(target));
        let target_id = match target {
            Some(some_target) => {
                chosen.insert(some_target.id);
                Some(some_target.id)
            }
            None => None
        };
        (attacker.id, target_id)
    }).collect()
}


fn main() {
    let stdin = io::stdin();
    let mut groups: Vec<Group> = stdin
        .lock()
        .lines()
        .zip(0..)
        .map(|(s, i)| read_group(&s.expect("Read error"), i))
        .collect();

    println!("{:?}", select_targets(&mut groups));

    for group in groups.iter() {
        println!("{:?}", group);
    }
}
