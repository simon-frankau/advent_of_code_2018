use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::slice::Iter;

// We can construct a DAG to represent the movements given, and then
// follow all the possible paths as a breadth-first search, tracking
// locations and remaining regexp. Drop the cases that visit existing
// nodes taking more time than necessary.

// I had a false start here, assuming the problem was the rather
// simpler "find the longest match for the regexp" problem.

enum Match {
   Literal(char),
   Alternation(Vec<Match>),
   Concatenation(Vec<Match>),
}

impl std::fmt::Debug for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Match::Literal(c) => write!(f, "{}", c)?,
            Match::Concatenation(xs) => {
                for x in xs.iter() {
                    x.fmt(f)?;
                }
            }
            Match::Alternation(xs) => {
                // We could do precedence-based printing, but let's always put them in...
                let mut first = true;
                for x in xs.iter() {
                    write!(f, "{}", if first {'('} else {'|'})?;
                    first = false;
                    x.fmt(f)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

fn parse_regexp(iter: &mut Peekable<Iter<char>>) -> Match {
    // Current alternation, made of a sequence of concatentations.
    let mut alternatives = Vec::new();
    // Current concatenation being built.
    let mut curr = Vec::new();
    loop {
        let c = match iter.peek() {
            Some(c) => Some(*c),
            None => None,
        };
        match c {
            Some('(') => {
                iter.next();
                curr.push(parse_regexp(iter));
                if iter.next() != Some(&')') {
                    panic!("Imbalanced brackets");
                }
            }
            Some('|') => {
                iter.next();
                alternatives.push(Match::Concatenation(curr));
                curr = Vec::new();
            }
            Some(')') => break,
            None => break,
            Some(c) => {
                curr.push(Match::Literal(*c));
                iter.next();
            }
        }
    }
    alternatives.push(Match::Concatenation(curr));
    Match::Alternation(alternatives)
}

// This just cleans up the regexp tree, without understanding paths.
fn opt_regexp(m: Match) -> Match {
    match m {
        Match::Alternation(xs) => {
            let xs: Vec<Match> = xs.into_iter().map(opt_regexp).collect();
            if xs.len() == 1 {
                // Take first element, and discard rest.
                xs.into_iter().next().unwrap()
            } else {
                Match::Alternation(xs)
            }
        }
        Match::Concatenation(xs) => {
            let xs: Vec<Match> = xs.into_iter().map(opt_regexp).collect();
            if xs.len() == 1 {
                // Take first element, and discard rest.
                xs.into_iter().next().unwrap()
            } else {
                Match::Concatenation(xs)
            }
        }
        Match::Literal(_) => m,
    }
}

// This removes obvious, basic back-tracking.
fn opt_backtracks(m: Match) -> Match {
    match m {
        Match::Alternation(xs) => {
            Match::Alternation(xs.into_iter().map(opt_backtracks).collect())
        }
        Match::Literal(_) => m,
        Match::Concatenation(xs) => {
            let mut xs = xs.into_iter().map(opt_backtracks).collect::<Vec<_>>();
            let mut i = 0;
            while i + 1 < xs.len() {
                if if let (Match::Literal(a), Match::Literal(b)) = (&xs[i], &xs[i+1]) {
                    match (a, b) {
                        ('N', 'S') => true,
                        ('S', 'N') => true,
                        ('W', 'E') => true,
                        ('E', 'W') => true,
                        _ => false,
                    }
                } else {
                    false
                } {
                    xs.drain(i..i+2);
                    if i > 0 {
                        i -= 1;
                    }
                } else {
                    i += 1;
                }
            }
            Match::Concatenation(xs)
        }
    }
}

// Is this an empty match? Used by opt_empties.
fn is_empty(m: &Match) -> bool {
    match m {
        Match::Literal(_) => false,
        Match::Concatenation(xs) => xs.iter().all(is_empty),
        Match::Alternation(xs) => xs.len() > 0 && xs.iter().all(is_empty),
    }
}

// And this removes alternatives of thing from concatenations. It's a
// specific optimisation, but seems key to this exercise.
fn opt_empties(m: Match) -> Match {
    match m {
        Match::Alternation(xs) => {
            Match::Alternation(xs.into_iter().map(opt_empties).collect())
        }
        Match::Literal(_) => m,
        Match::Concatenation(xs) => {
            Match::Concatenation(xs.into_iter().map(opt_empties).filter(|x| !is_empty(x)).collect())
        }
    }
}

// Find the longest match for a Match
fn find_longest_match(m: &Match) -> String {
    match m {
        Match::Literal(c) => std::iter::once(c).collect(),
        Match::Concatenation(xs) => {
            let mut s = String::new();
            for x in xs.iter() {
                s.push_str(&find_longest_match(x));
            }
            s
        }
       Match::Alternation(xs) => {
           xs.iter().map(find_longest_match).max_by_key(String::len).unwrap()
       }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let chars = buffer.replace('^', "").replace('$', "").trim().chars().collect::<Vec<_>>();

    println!("{:?}\n", chars);
    let res =  parse_regexp(&mut chars.iter().peekable());
    println!("{:?}\n", res);
    let res = opt_regexp(res);
    println!("{:?}\n", res);
    let res = opt_backtracks(res);
    println!("{:?}\n", res);
    let res = opt_empties(res);
    println!("{:?}\n", res);
    let res = find_longest_match(&res);
    println!("{}", res);
    if ["NS", "SN", "WE", "EW"].iter().map(|x| res.contains(x)).any(|x| x) {
        println!("Longest solution contains backtrack. :/")
    }
    println!("{}", res.len());
}

// Ugh, got bored trying to guess the algorithm's intention, and
// instead did a greedy long route, eliminated backtracks, and then found
// the longest route: Also got 3929 - too low.

// Turns out to be 3930, including a bit at the end for further
// distance that then gets back-tracked, and shouldn't be removed.
//

// My algorithm removes backtracks from regexp first, then finds
// route, but this doesn't seem to be necessary this way round, by
// construction of the problem?! So backtracking outside an alternation
// doesn't seem to be a thing? Generally, there seems to be some assumptions
// baked into the input that makes this thing doable greedily, rather than
// require exponential time or whatever. Ugh. Horrible question.
