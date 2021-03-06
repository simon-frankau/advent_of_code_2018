use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Clone)]
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

////////////////////////////////////////////////////////////////////////
// This is the bit for problem 20a...
//

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

// This removes obvious, basic back-tracking (back-tracking that
// occurs only within a single concatenation of literals).
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

////////////////////////////////////////////////////////////////////////
// Problem 20b part
//

// Find the route to the turning point for a sequence of literals
fn get_literal_partial(xs: &[Match]) -> Option<Vec<Match>> {
    if xs.len() == 0 {
        return None;
    }
    for elem in xs.iter().zip(xs.iter().rev()) {
        match elem {
            (Match::Literal('N'), Match::Literal('S')) => (),
            (Match::Literal('S'), Match::Literal('N')) => (),
            (Match::Literal('W'), Match::Literal('E')) => (),
            (Match::Literal('E'), Match::Literal('W')) => (),
            _ => return None,
        }
    }
    Some(xs.iter().take(xs.len() / 2).map(|x| (*x).clone()).collect())
}

// Given a route that involves back-tracks, generate a list of routes
// up to the turning-around point. e.g. NEWS -> NE.
fn get_partials(m: &Match) -> Vec<Match> {
    match m {
        Match::Alternation(xs) => {
            let mut res = Vec::new();
            for alternative in xs.iter() {
                res.extend(get_partials(alternative).into_iter());
            }
            res
        }
        // A single literal will have no backtrackable parts.
        Match::Literal(_) => Vec::new(),
        Match::Concatenation(xs) => {
            match get_literal_partial(xs) {
                Some(x) => vec![Match::Concatenation(x)],
                None => {
                    let mut res = Vec::new();
                    for i in 0..xs.len() {
                        let partials = get_partials(&xs[i]);
                        for partial in partials.into_iter() {
                            let mut element = xs.iter().take(i).map(|x| (*x).clone()).collect::<Vec<Match>>();
                            element.push(partial);
                            res.push(Match::Concatenation(element));
                        }
                    }
                    res
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////
// Generate all the possible strings.
//

fn generate_all(m: &Match) -> HashSet<String> {
    let mut res: HashSet<String> = HashSet::new();
    match m {
        Match::Literal(x) => {
            res.insert(x.to_string());
            ()
        }
        Match::Alternation(xs) => {
            for x in xs.iter() {
                res.extend(generate_all(x).into_iter());
            }
        }
        Match::Concatenation(xs) => {
            // Ugh. Cross products are potentially expensive.
            res.insert(String::new());
            for x in xs.iter() {
                let to_cross = generate_all(x);
                add_cross_string(&mut res, &to_cross);
            }
        }
    }
    res
}

fn add_cross_string(lhs: &mut HashSet<String>, rhs: &HashSet<String>) {
    let mut res = HashSet::new();

    for s1 in lhs.iter() {
        for s2 in rhs.iter() {
            let mut s = s1.clone();
            s.push_str(&s2);
            res.insert(s);
        }
    }

    // This is where I'd like to swap lhs and res.
    lhs.clear();
    lhs.extend(res.into_iter());
}

// Generate all the incremental paths
fn all_prefixes(strs: &HashSet<String>) -> HashSet<String> {
    let mut seen = HashSet::new();
    for str in strs.iter() {
        for l in 0..str.len() {
            seen.insert(str.get(0..l+1).unwrap().to_string());
        }
    }
    seen
}

// Given a path, generate the coordinates of its end point.
fn get_coords(s: &str) -> (i32, i32) {
    let y = s.chars().map(|c| match c {
        'N' => 1,
        'S' => -1,
        _ => 0,
    }).sum();
    let x = s.chars().map(|c| match c {
        'E' => 1,
        'W' => -1,
        _ => 0,
    }).sum();
    (x, y)
}

// Build a mapping from coord to shortest route there.
fn build_mapping(strs: &HashSet<String>) -> HashMap<(i32, i32), usize> {
    let mut map = HashMap::new();
    for s in strs.iter() {
        let xy = get_coords(s);
        let l = s.len();
        let e = map.entry(xy).or_insert(1000000);
        if l < *e {
            *e = l;
        }
    }
    map
}

// Count the long routes
fn count_long(l: usize, mapping: &HashMap<(i32, i32), usize>) -> usize {
    mapping.iter().filter(|(_, l2)| **l2 >= l).count()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Read error");
    let chars = buffer.replace('^', "").replace('$', "").trim().chars().collect::<Vec<_>>();

    // println!("{:?}\n", chars);
    let res =  parse_regexp(&mut chars.iter().peekable());
    // println!("{:?}\n", res);

    // All the backtracks form a trivial pattern, so we'll extract all
    // the routes up to a backtrack (plus original route).
    let mut partials = get_partials(&res);
    partials.push(res);
    // println!("{:?}\n", partials);

    // Then we'll eliminate the back-tracks, etc.
    let partials = partials.into_iter().map(|x| opt_empties(opt_backtracks(opt_regexp(x)))).collect::<Vec<_>>();
    // println!("{:?}\n", partials);
    println!("{}\n", partials.len());

    // And now build the regexp of doom.
    let regex = Match::Alternation(partials);

    let all = generate_all(&regex);
    // println!("{:?}\n", all);
    println!("{}\n", all.len());

    // We have all the paths, now generate all the partial paths.
    let prefixes = all_prefixes(&all);
    println!("{}\n", prefixes.len());

    // Some paths will overlap, so for each coordinate, find the shortest path there.
    let mapping = build_mapping(&prefixes);
    println!("{}\n", mapping.len());

    // And find the count of coordinates over length 1000.
    println!("{}\n", count_long(1000, &mapping));

    // My, that was really, really tedious.

    // If I'd known you could just generate all of the paths in
    // sensible time once you'd taken out the obvious
    // backtracking... *sigh*.
}
