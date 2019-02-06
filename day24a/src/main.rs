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
    id: usize,
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
    fn attack_multiplier(self: &Group, target: &Group) -> u64 {
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

fn read_group(str: &str, id: usize) -> Group {
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

fn select_targets(groups: &[Group]) -> HashMap<usize, usize> {
    // Sort groups by effective power
    let mut group_refs = groups.iter().collect::<Vec<&Group>>();
    group_refs.sort_by_key(power_rank);

    // Targets that have been chosen already, and can't be re-selected.
    let mut chosen: HashSet<usize> = HashSet::new();
    // Mapping from attacker id to target id (if there is one).
    let mut mapping = HashMap::new();

    for attacker in group_refs.iter() {
        if attacker.unit_count == 0 {
            // Dead groups don't attack.
            continue;
        }

        // Use rev so that we get the highest effective power in a
        // draw (max_by_key returns the last of equals)
        let opt_target = groups.iter()
            .rev()
            .filter(|target| target.side != attacker.side && target.unit_count != 0 && !chosen.contains(&target.id))
            .max_by_key(|target| attacker.attack_multiplier(target));
        if let Some(target) = opt_target {
            // Hang on, if the best we can do is nothing, don't bother.
            if attacker.attack_multiplier(target) == 0 {
                continue;
            }

            let target_id = target.id;
            chosen.insert(target_id);
            mapping.insert(attacker.id, target_id);
        }
    }
    mapping
}

fn perform_attacks(groups: &mut [Group], targets: &HashMap<usize, usize>) {
    // Sort groups by initiative.
    let mut group_ids = (0..groups.len()).collect::<Vec<usize>>();
    group_ids.sort_by_key(|group_id| -(groups[*group_id].initiative as i64));

    for group_id in group_ids.iter() {
        // To work around the borrow-checker, we will work in two
        // phases: Decide the attack to make, then apply it.
        let (target_id, damage) = {
            let group = &groups[*group_id];
            // Dead groups don't attack.
            if group.unit_count == 0 {
                continue;
            }
            // Only do something if we have a target.
            if let Some(target_id) = targets.get(&group.id) {
                let damage = group.unit_count * group.attack_damage * group.attack_multiplier(&groups[*target_id]);
                println!("{} would attack {} for {}", group.id, target_id, damage);
                (target_id, damage)
            } else {
                continue;
            }
        };

        let target = &mut groups[*target_id];
        let kill_count = damage / target.hit_points;
        println!("{} attacked for {}, killing {}", target_id, damage, kill_count);
        target.unit_count -= kill_count.min(target.unit_count);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut groups: Vec<Group> = stdin
        .lock()
        .lines()
        .zip(0..)
        .map(|(s, i)| read_group(&s.expect("Read error"), i))
        .collect();

    loop {
        for group in groups.iter() {
            println!("{:?}", group);
        }

        let targets = select_targets(&groups);
        println!("{:?}", targets);
        if targets.is_empty() {
            break;
        }
        perform_attacks(&mut groups, &targets);
    }

    println!("{}", groups.iter().map(|group| group.unit_count).sum::<u64>());
}
// 15327 is too low.
// 15591 is too low.
