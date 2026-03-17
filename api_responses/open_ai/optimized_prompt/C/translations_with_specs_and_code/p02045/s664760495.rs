use std::io::{self, BufRead};

const MAX_POINTS: usize = 128;

fn nigeru(x: i32, y: i32, is_src: bool, a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut cost = 0;
    if x < a {
        cost += a - x;
    }
    if x > c {
        cost += x - c;
    }
    if y < b {
        cost += b - y;
    }
    if y > d {
        cost += y - d;
    }
    if is_src && cost > 0 {
        cost -= 1;
    }
    cost
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    if let Some(Ok(line)) = lines.next() {
        let n: i32 = line.trim().parse().unwrap_or(0);
        if n <= 0 {
            return;
        }

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        if let Some(Ok(line)) = lines.next() {
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if nums.len() != 4 {
                return;
            }
            a = nums[0];
            b = nums[1];
            c = nums[2];
            d = nums[3];
        }

        let mut x = vec![0; (n + 1) as usize];
        let mut y = vec![0; (n + 1) as usize];

        for i in 0..=n {
            if let Some(Ok(line)) = lines.next() {
                let coords: Vec<i32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if coords.len() != 2 {
                    return;
                }
                x[i as usize] = coords[0];
                y[i as usize] = coords[1];
            }
        }

        let mut mikakosi = 0;

        for i in 1..=n {
            let naka = (x[i as usize - 1] - x[i as usize]).abs() + (y[i as usize - 1] - y[i as usize]).abs();
            let simakaze = nigeru(x[i as usize - 1], y[i as usize - 1], true, a, b, c, d) +
                           nigeru(x[i as usize], y[i as usize], false, a, b, c, d);
            mikakosi += naka.min(simakaze);
        }

        println!("{}", mikakosi);
    }
}
