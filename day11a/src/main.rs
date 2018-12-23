const GRID_SERIAL: i32 = 5719;

fn score(x: usize, y: usize) -> i32 {
    let (x, y) = ((x as i32) + 1, (y as i32) + 1);

    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + GRID_SERIAL;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;
    power_level - 5
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
            row.push(score(x, y));
        }
       grid.push(row);
    }

    // Find highest power
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_score = -1000000;
    for y in 0..HEIGHT-SIZE+1 {
        for x in 0..WIDTH-SIZE+1 {
            let mut score = 0;
            for dx in 0..SIZE {
                for dy in 0..SIZE {
                    score += grid[y + dy][x + dx];
                }
            }
           if score > best_score {
               best_score = score;
               best_x = x;
               best_y = y;
           }
        }
    }

    println!("{},{}", best_x+1, best_y+1);
}
