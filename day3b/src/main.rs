use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

// Representation of rectangle with lower bound included, upper bound
// excluded.
#[derive(Debug)]
struct Rect {
    id: i32,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn read_rect(str: &str) -> Rect {
    // Normalise the separators, and split up.
    let parts = str
        .replace('#', "")
        .replace('@', ",")
        .replace(':', ",")
        .replace('x', ",")
        .split(',')
        .map(|s| s.trim().parse::<i32>().expect("Parse error"))
        .collect::<Vec<_>>();
    // Convert from origin/size to min/max coords.
    Rect {
        id: parts[0],
        x_min: parts[1],
        x_max: parts[1] + parts[3],
        y_min: parts[2],
        y_max: parts[2] + parts[4],
    }
}

// Representation of edges at a point. Stored left and right edge ids.
#[derive(Debug)]
struct Edge {
    end_ids: HashSet<i32>,
    start_ids: HashSet<i32>,
}

impl Edge {
    fn new() -> Edge {
        Edge { end_ids: HashSet::new(), start_ids: HashSet::new() }
    }

    fn add_left(&mut self, id: i32) -> () {
        self.start_ids.insert(id);
    }

    fn add_right(&mut self, id: i32) -> () {
        self.end_ids.insert(id);
    }

    fn remove_left(&mut self, id: i32) -> () {
        self.start_ids.remove(&id);
    }

    fn remove_right(&mut self, id: i32) -> () {
        self.end_ids.remove(&id);
    }

    fn is_empty(&self) -> bool {
        self.end_ids.is_empty() && self.start_ids.is_empty()
    }
}

fn main() {
    let stdin = io::stdin();
    let rects: Vec<Rect> = stdin
        .lock()
        .lines()
        .map(|s| read_rect(&s.expect("Read error")))
        .collect();

    #[derive(PartialEq)]
    enum LR {
        Left,
        Right
    }
    #[derive(PartialEq)]
    enum TB {
        Top,
        Bottom
    }

    // And now we scan convert, starting by making a list of changes
    // that happen as we scan the y direction.
    let mut y_deltas = BTreeMap::new();
    for rect in rects.iter() {
        {
            let y_min_entry = y_deltas.entry(rect.y_min).or_insert_with(Vec::new);
            (*y_min_entry).push((rect.x_min, rect.id, LR::Left, TB::Top));
            (*y_min_entry).push((rect.x_max, rect.id, LR::Right, TB::Top));
        }
        {
            let y_max_entry = y_deltas.entry(rect.y_max).or_insert_with(Vec::new);
            (*y_max_entry).push((rect.x_min, rect.id, LR::Left, TB::Bottom));
            (*y_max_entry).push((rect.x_max, rect.id, LR::Right, TB::Bottom));
        }
    }

    // Now, let's step through the ordered y changes, updating and
    // processing the x extents and using that to calculate accumulated
    // area.
    let mut area = 0;
    let mut last_extent = 0;
    let mut last_y = 0;
    let mut x_deltas = BTreeMap::new();
    for (y, y_delta) in y_deltas.iter() {
        // Start by accumulating area since the last y_delta, update y.
        area += last_extent * (y - last_y);
        last_y = *y;
        // Now update our current x_deltas.
        for (x, x_id, lr, tb) in y_delta.iter() {
            let to_remove = {
                let x_entry = x_deltas.entry(*x).or_insert(Edge::new());
                match (lr, tb) {
                    (LR::Left, TB::Top) => (*x_entry).add_left(*x_id),
                    (LR::Right, TB::Top) => (*x_entry).add_right(*x_id),
                    (LR::Left, TB::Bottom) => (*x_entry).remove_left(*x_id),
                    (LR::Right, TB::Bottom) => (*x_entry).remove_right(*x_id),
                }
                (*x_entry).is_empty()
            };
            if to_remove {
                x_deltas.remove(x);
            }
        }
        // And calculate our current accumulating extent.
        last_extent = 0;
        let mut last_x = 0;
        let mut inside = HashSet::new();
        for (x, x_delta) in x_deltas.iter() {
            // Accumulate extent since last x_delta, update x.
            if inside.len() > 1 {
                last_extent += *x - last_x;
            }
            last_x = *x;
            // Update current overlaps.
            for end in x_delta.end_ids.iter() {
                inside.remove(end);
            }
            for start in x_delta.start_ids.iter() {
                inside.insert(start);
            }
        }
    }
    println!("Area: {}", area);
}
