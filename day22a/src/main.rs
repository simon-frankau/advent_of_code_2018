fn get_erosion_levels(max_x: usize, max_y: usize, depth:usize) -> Vec<Vec<usize>> {
    let mut erosion_levels: Vec<Vec<usize>> = Vec::new();

    // Build the erosion levels for y = 0.
    erosion_levels.push((0..max_x + 1).map(|x| (x * 16807 + depth) % 20183).collect());

    // Then all following rows...
    for y in 1..max_y + 1 {
        let mut row = vec![(y * 48271 + depth) % 20183];
        for x in 1..max_x + 1 {
            let erosion_level = (row[x - 1] * erosion_levels[y - 1][x] + depth) % 20183;
            row.push(erosion_level);
        }
        erosion_levels.push(row);
    }

    // Override target
    erosion_levels[max_y][max_x] = depth % 20183;

    return erosion_levels;
}

fn print_erosion_levels(erosion_levels: &Vec<Vec<usize>>) {
   for row in erosion_levels.iter() {
        for cell in row.iter() {
            match *cell % 3 {
                0 => print!("."),
                1 => print!("="),
                _ => print!("|"),
            }
        }
        println!("");
    }
}

fn sum_erosion_levels(erosion_levels: &Vec<Vec<usize>>) -> usize {
    erosion_levels.iter().map(|xs| xs.iter().map(|x| x % 3).sum::<usize>()).sum()
}

fn main() {
    // Test input.
    // let erosion_levels = get_erosion_levels(10, 10, 510);
    let erosion_levels = get_erosion_levels(7, 782, 11820);
    println!("{}\n", sum_erosion_levels(&erosion_levels));
}
