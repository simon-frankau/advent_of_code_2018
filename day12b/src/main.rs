// In the long run, either the cellular automata is some kind of
// annoying Turing machine simulator or something, or (more likely) it
// goes to either a cycling pattern, or forms a glider, or a glider gun,
// or something like that. We're expecting repetition, so we'll just
// print a bunch of generations and then work from there...
//

// We do this, and see it iterates out to a fixed pattern which then
// glides, creating an arithmetic sequence of plant scores.

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt;

const PATTERN_SIZE: i32 = 5;
const PATTERN_CENTRE: i32 = 2;
const GENERATIONS: i32 = 200;

struct State {
    state: BTreeSet<i32>,
}

impl State {
    fn from_str(str: &str) -> State {
        State {
            state: str
                .chars()
                .zip(0..)
                .filter(|(c, _)| *c == '#')
                .map(|(_, i)| i)
                .collect(),
        }
    }

    fn update(&self, plants: &HashSet<String>) -> State {
        let mut next = BTreeSet::new();
        let min = *self.state.iter().next().unwrap() - PATTERN_SIZE;
        let max = *self.state.iter().next_back().unwrap();
        for i in min..max {
            let seq: String = (0..PATTERN_SIZE)
                .map(|j| {
                    if self.state.contains(&(i + j)) {
                        '#'
                    } else {
                        '.'
                    }
                }).collect();
            // println!("{}", seq);
            if plants.contains(&seq) {
                next.insert(i + PATTERN_CENTRE);
            }
        }
        State { state: next }
    }

    fn score(&self) -> i32 {
        self.state.iter().sum()
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min = *self.state.iter().next().unwrap();
        let max = *self.state.iter().next_back().unwrap();
        write!(f, "{}-{} ", min, max)?;
        for i in min..max + 1 {
            write!(f, "{}", if self.state.contains(&i) { '#' } else { '.' })?;
        }
        Ok(())
    }
}

fn build_map() -> HashSet<String> {
    [
        "#####", "###.#", "###..", "##...", "#.#..", "#..#.", ".###.", ".##.#", ".##..", ".#.#.",
        ".#...", "..###", "..#.#", "..#..", "...#.",
    ]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
/*
    let init_state = "##..##....#.#.####........##.#.#####.##..#.#..#.#...##.#####.###.##...#....##....#..###.#...#.#.#.#";

    let mut state = State::from_str(init_state);
    let map = build_map();
    let mut prev_score = 0;
    for generation in 0..GENERATIONS {
        let score = state.score();
        let estimate = generation * 80;
        println!("{:?} {} {}", state, score, estimate); // Accurate estimate!
        prev_score = score;
        state = state.update(&map);
    }
*/

    println!("{}", 50000000000i64 * 80);
}
