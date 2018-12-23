const GRID_SERIAL: i32 = 5719;

fn get_power(x: usize, y: usize) -> i32 {
    let (x, y) = ((x as i32) + 1, (y as i32) + 1);

    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + GRID_SERIAL;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;
    power_level - 5
}

// Build running sums, starting at 0.
fn sum_row(row: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut sum = 0;
    res.push(0);
    for x in row.iter() {
        sum += *x;
        res.push(sum);
    }
    res
}

fn build_sum_grid(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let width = grid[0].len();

    // Generate a list of row sums.
    let mut res = Vec::new();
    res.push(vec![0; width + 1]);
    for row in grid.iter() {
        res.push(sum_row(row));
    }

    // And then generate running sums down the columns.
    for y in 1..res.len() {
        for x in 0..width + 1 {
            res[y][x] += res[y - 1][x];
        }
    }

    res
}

fn get_score(grid: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> i32 {
    grid[y + size][x + size] - grid[y][x + size] - grid[y + size][x] + grid[y][x]
}

fn main() {
    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;
    const SIZE: usize = 3;

    // Build powers.
    let mut grid = Vec::new();
    for y in 0..HEIGHT {
        let mut row = Vec::new();
        for x in 0..WIDTH {
            row.push(get_power(x, y));
        }
        grid.push(row);
    }

    let sum_grid = build_sum_grid(&grid);

    // Find highest power
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_score = -1000000;
    for y in 0..HEIGHT - SIZE + 1 {
        for x in 0..WIDTH - SIZE + 1 {
            let score = get_score(&sum_grid, x, y, 3);
            if score > best_score {
                best_score = score;
                best_x = x;
                best_y = y;
            }
        }
    }

    println!("{},{}", best_x + 1, best_y + 1);
}
