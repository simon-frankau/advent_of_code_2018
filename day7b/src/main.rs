use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

// Pull out the graph nodes.
fn pair_parse(str: &str) -> (char, char) {
    let snipped = str
        .trim()
        .replace("Step ", "")
        .replace(" can begin.", "")
        .chars()
        .collect::<Vec<_>>();
    (snipped[0], snipped[snipped.len() - 1])
}

fn find_first_no_dep(deps: &BTreeMap<char, HashSet<char>>) -> Option<char> {
    for (node, node_deps) in deps.iter() {
        if node_deps.is_empty() {
            return Some(*node);
        }
    }
    None
}

fn schedule_work(
    events: &mut BTreeMap<i32, Vec<char>>,
    deps: &mut BTreeMap<char, HashSet<char>>,
    curr_time: i32,
    free_elves: &mut i32,
) {
    loop {
        if *free_elves == 0 {
            return;
        }
        match find_first_no_dep(deps) {
            None => return,
            Some(c) => {
                let duration = (c as i32) - ('A' as i32) + 61;
                let entry = events.entry(curr_time + duration).or_insert(Vec::new());
                (*entry).push(c);
                println!("{} from {} to {}", c, curr_time, curr_time + duration);
                *free_elves -= 1;
                deps.remove(&c);
            }
        }
    }
}

fn complete_work(
    events: &mut BTreeMap<i32, Vec<char>>,
    deps: &mut BTreeMap<char, HashSet<char>>,
    curr_time: &mut i32,
    free_elves: &mut i32,
) {
    let to_remove = match events.iter().next() {
        None => panic!("Nothing to do, not complete?!"),
        Some((t, nodes)) => {
            *curr_time = *t;
            for c in nodes.iter() {
                println!("{} finished at {}", c, t);

                // Unblock the work...
                for (_node, node_deps) in deps.iter_mut() {
                    node_deps.remove(&c);
                }
                // and free the elf.
                *free_elves += 1;
            }
            *t
        }
    };
    events.remove(&to_remove);
}

fn main() {
    let stdin = io::stdin();
    let edges: Vec<_> = stdin
        .lock()
        .lines()
        .map(|s| pair_parse(&s.unwrap()))
        .collect();

    // Build set of deps for each node.
    let mut deps = BTreeMap::new();
    for (before, after) in edges.iter() {
        deps.entry(*before).or_insert(HashSet::new());
        let after_entry = deps.entry(*after).or_insert(HashSet::new());
        (*after_entry).insert(*before);
    }

    // Use a fairly brute-force approach. At each time step, we
    // keep assigning work until we're out of elves or work.
    // Then we move time forward, complete work, etc.
    let mut free_elves = 5;
    let mut curr_time = 0;
    let mut events = BTreeMap::new();

    schedule_work(&mut events, &mut deps, curr_time, &mut free_elves);
    while !deps.is_empty() {
        complete_work(&mut events, &mut deps, &mut curr_time, &mut free_elves);
        schedule_work(&mut events, &mut deps, curr_time, &mut free_elves);
    }

    print!("{}", events.keys().max().unwrap());
}
