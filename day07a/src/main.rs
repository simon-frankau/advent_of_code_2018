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

fn find_first_no_dep(deps: &BTreeMap<char, HashSet<char>>) -> char {
    for (node, node_deps) in deps.iter() {
        if node_deps.is_empty() {
            return *node;
        }
    }
    panic!("Shouldn't happen");
}

fn remove_node(deps: &mut BTreeMap<char, HashSet<char>>, to_remove: char) {
    deps.remove(&to_remove);
    for (_node, node_deps) in deps.iter_mut() {
        node_deps.remove(&to_remove);
    }
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

    println!("{:?}", deps);

    // Now, let's do a brute-force and dumb "find earliest node with
    // dep, print it, remove, repeat" algorithm. This is ok since the
    // data size is small. It's easy to implement.
    while deps.len() > 0 {
        let node = find_first_no_dep(&deps);
        print!("{}", node);
        remove_node(&mut deps, node);
    }
}
