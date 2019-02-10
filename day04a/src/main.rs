use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Entry {
    Shift(i32),
    Sleep(i32),
    Wake(i32),
}

fn get_minute(str: &str) -> i32 {
    let rhs = str.split(':').collect::<Vec<_>>()[1];
    let lhs = rhs.split(']').collect::<Vec<_>>()[0];
    lhs.parse().expect("Parse error")
}

fn parse_line(str: &str) -> Entry {
    if str.contains("wakes") {
        Entry::Wake(get_minute(str))
    } else if str.contains("asleep") {
        Entry::Sleep(get_minute(str))
    } else {
        let rhs = str.split('#').collect::<Vec<_>>()[1];
        let num = rhs
            .replace("begins shift", "")
            .trim()
            .parse()
            .expect("Parse error");
        Entry::Shift(num)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|s| s.expect("Read error"))
        .collect();

    // Nice date/time formats means lexicographical sort is time sort. \o/
    lines.sort();

    let entries = lines.iter().map(|s| parse_line(&s)).collect::<Vec<_>>();

    // For this puzzle, we don't need to track days when there was no
    // sleep, so the code is nice and simple...
    let mut sleep_map = HashMap::new();
    let mut guard = -1;
    let mut last_sleep = 0;

    for line in entries.iter() {
        // Assume entries are well-structured and don't bother
        // sanity-checking. I'm lazy.
        match line {
            Entry::Shift(g) => guard = *g,
            Entry::Sleep(t) => last_sleep = *t,
            Entry::Wake(t) => {
                let entry = sleep_map.entry(guard).or_insert_with(Vec::new);
                (*entry).push((last_sleep, *t));
            }
        }
    }

    // Build an iterator of guard/sleep durations...
    let sleeps = sleep_map
        .iter()
        .map(|(g, v)| (g, v.iter().map(|(s, e)| e - s).fold(0, |x, y| x + y)));

    // And find the sleepiest:
    let (sleepiest_guard, _) = sleeps.fold(
        (-1, -1),
        |(og, od), (g, d)| if d > od { (*g, d) } else { (og, od) },
    );

    println!("{}", sleepiest_guard);
    let sleep_times = &sleep_map[&sleepiest_guard];

    // And as there are only 60 minutes in an hour, let's just create
    // a small array...
    let mut minutes = vec![0; 60];

    for (s, e) in sleep_times.iter() {
        for minute in *s..*e {
            minutes[minute as usize] += 1;
        }
    }

    let sleep_iter = (0..).zip(minutes.iter());
    // Hmmm. Same pattern...
    let (sleepiest_minute, _) = sleep_iter.fold(
        (-1, -1),
        |(om, oc), (m, c)| if *c > oc { (m, *c) } else { (om, oc) },
    );
    println!("{}", sleepiest_minute);

    println!("{}", sleepiest_guard * sleepiest_minute);
}
