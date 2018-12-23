use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Point {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .replace("position=<", "")
            .replace("> velocity=<", ",")
            .replace(">", "")
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Ok(Point {
            px: parts[0],
            py: parts[1],
            vx: parts[2],
            vy: parts[3],
        })
    }
}

fn step_time(points: &[Point], time: i32) -> Vec<Point> {
    let update = |p: &Point| Point {
        px: p.px + time * p.vx,
        py: p.py + time * p.vy,
        vx: p.vx,
        vy: p.vy,
    };

    points.iter().map(update).collect()
}

fn extents(points: &[Point]) -> (i32, i32) {
    let min_x = points.iter().map(|p| p.px).min().unwrap();
    let max_x = points.iter().map(|p| p.px).max().unwrap();
    let min_y = points.iter().map(|p| p.py).min().unwrap();
    let max_y = points.iter().map(|p| p.py).max().unwrap();
    (max_x - min_x, max_y - min_y)
}

fn render(points: &[Point]) {
    let origin_x = points.iter().map(|p| p.px).min().unwrap();
    let origin_y = points.iter().map(|p| p.py).min().unwrap();
    let width = points.iter().map(|p| p.px).max().unwrap() + 1 - origin_x;
    let height = points.iter().map(|p| p.py).max().unwrap() + 1 - origin_y;

    let mut display = vec!['.'; (width * height) as usize];
    for p in points.iter() {
        let x = p.px - origin_x;
        let y = p.py - origin_y;
        display[(y * width + x) as usize] = '#';
    }

    let mut i = 0;
    for c in display.iter() {
        if i == width {
            i = 0;
            println!("");
        }
        i += 1;
        print!("{}", c);
    }
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<_> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<Point>().unwrap())
        .collect();

    let mut t = 0;
    while extents(&step_time(&points, t)).1 > 10 {
        t += 1;
    }

    render(&step_time(&points, t));

    println!("");
    println!("{}", t);
}
