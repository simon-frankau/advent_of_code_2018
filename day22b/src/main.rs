use std::collections::HashMap;
use std::collections::HashSet;

// It's can be a very roundabout route there... some kind of A* or
// meet-in-the-middle would be better, but this is good enough.
const SLACK: usize = 1000;

// Step to build the map of erosion levels
fn get_erosion_levels(tgt_x: usize, tgt_y: usize, depth:usize) -> Vec<Vec<usize>> {
    let max_x = tgt_x + SLACK;
    let max_y = tgt_y + SLACK;

    let mut erosion_levels: Vec<Vec<usize>> = Vec::new();

    // Build the erosion levels for y = 0.
    erosion_levels.push((0..max_x + 1).map(|x| (x * 16807 + depth) % 20183).collect());

    // Then all following rows...
    for y in 1..max_y + 1 {
        let mut row = vec![(y * 48271 + depth) % 20183];
        for x in 1..max_x + 1 {
            let geologic_index = if y == tgt_y && x == tgt_x {
                0
            } else {
                row[x - 1] * erosion_levels[y - 1][x]
            };
            let erosion_level = (geologic_index + depth) % 20183;
            row.push(erosion_level);
        }
        erosion_levels.push(row);
    }

    // Override target
    erosion_levels[max_y][max_x] = depth % 20183;

    return erosion_levels;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Equipment {
    ClimingGear,
    Torch,
    Neither
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    x: isize,
    y: isize,
    e: Equipment
}

// Then perform a breadth-first search of the site
fn time_to_target(start: State, end: State, erosion_levels: &Vec<Vec<usize>>) -> usize {
    // Set of States we've found the optimal route for.
    let mut seen: HashSet<State> = HashSet::new();
    // Time ordered queue of places to go to.
    let mut queue = HashMap::new();
    // Current time-step.
    let mut time = 0;

    // Initialise the starting position...
    queue.insert(0, vec![start]);

    loop {
        // println!("Time {}", time);
        if let Some(states) = queue.remove(&time) {
            for s in states.into_iter() {
                if s == end {
                    return time;
                }
                if !seen.insert(s) {
                    continue;
                }
                // println!("Seen: {:?} at {}", s, time);
                {
                    let mut move_time = queue.entry(time + 1).or_insert(Vec::new());
                    add_if_safe(erosion_levels, &mut move_time, State{ x: s.x - 1, y: s.y, e: s.e });
                    add_if_safe(erosion_levels, &mut move_time, State{ x: s.x + 1, y: s.y, e: s.e });
                    add_if_safe(erosion_levels, &mut move_time, State{ x: s.x, y: s.y - 1, e: s.e });
                    add_if_safe(erosion_levels, &mut move_time, State{ x: s.x, y: s.y + 1, e: s.e });
                }
                {
                    let mut switch_time = queue.entry(time + 7).or_insert(Vec::new());
                    add_if_safe(erosion_levels, &mut switch_time, State{ x: s.x, y: s.y, e: Equipment::ClimingGear });
                    add_if_safe(erosion_levels, &mut switch_time, State{ x: s.x, y: s.y, e: Equipment::Torch });
                    add_if_safe(erosion_levels, &mut switch_time, State{ x: s.x, y: s.y, e: Equipment::Neither });
                }
            }
        }
        time += 1;
    }
}

// Add the new state only if it's a safe one.
fn add_if_safe(erosion_levels: &Vec<Vec<usize>>, queue: &mut Vec<State>, s: State) {
    if s.x < 0 || s.y < 0 {
        return;
    }

    let region_type = erosion_levels[s.y as usize][s.x as usize] % 3;

    match (s.e, region_type) {
        (Equipment::ClimingGear, 2) => return,
        (Equipment::Torch, 1) => return,
        (Equipment::Neither, 0) => return,
        _ => (),
    }

    queue.push(s);
}

fn main() {
    // Test input.
    // let (tgt_x, tgt_y, depth) = (10, 10, 510);
    // Real input.
    let (tgt_x, tgt_y, depth) = (7, 782, 11820);
    let erosion_levels = get_erosion_levels(tgt_x, tgt_y, depth);
    let start = State { x: 0, y: 0, e: Equipment::Torch };
    let end = State { x: tgt_x as isize, y: tgt_y as isize, e: Equipment::Torch };
    println!("Reached target at {}", time_to_target(start, end, &erosion_levels));
}
