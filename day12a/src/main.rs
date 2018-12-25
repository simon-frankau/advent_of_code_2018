use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt;

const PATTERN_SIZE: i32 = 5;
const PATTERN_CENTRE: i32 = 2;
const GENERATIONS: i32 = 20;

const TEST: bool = false;

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

fn build_test_map() -> HashSet<String> {
    [
        "####.", "###.#", "###..", "##.##", "##.#.", "#.###", "#.#.#", ".####", ".##..", ".#.##",
        ".#.#.", ".#...", "..#..", "...##",
    ]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn build_real_map() -> HashSet<String> {
    [
        "#####", "###.#", "###..", "##...", "#.#..", "#..#.", ".###.", ".##.#", ".##..", ".#.#.",
        ".#...", "..###", "..#.#", "..#..", "...#.",
    ]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let test_str = "#..#.#..##......###...###";
    let real_str = "##..##....#.#.####........##.#.#####.##..#.#..#.#...##.#####.###.##...#....##....#..###.#...#.#.#.#";

    let mut state = State::from_str(if TEST { test_str } else { real_str });
    let map = if TEST {
        build_test_map()
    } else {
        build_real_map()
    };
    for _ in 0..GENERATIONS {
        state = state.update(&map);
    }
    println!("{:?} {}", state, state.score());
}
