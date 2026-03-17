use std::io::{self, BufRead};

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
        let n: i32 = line.trim().parse().expect("Invalid input for N");
        if n <= 0 {
            return;
        }

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        if let Some(Ok(line)) = lines.next() {
            let values: Vec<i32> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid boundary input"))
                .collect();
            if values.len() != 4 {
                eprintln!("Error: Expected 4 boundary values");
                return;
            }
            a = values[0];
            b = values[1];
            c = values[2];
            d = values[3];
        }

        let mut x = vec![0; (n + 1) as usize];
        let mut y = vec![0; (n + 1) as usize];

        for i in 0..=n {
            if let Some(Ok(line)) = lines.next() {
                let coords: Vec<i32> = line
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().expect("Invalid coordinate input"))
                    .collect();
                if coords.len() != 2 {
                    eprintln!("Error: Expected 2 coordinates");
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
