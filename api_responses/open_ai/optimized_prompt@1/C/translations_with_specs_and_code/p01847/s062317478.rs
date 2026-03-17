use std::io::{self, BufRead};

const INF: i32 = 20002;

fn read_int() -> Option<i32> {
    let mut input = String::new();
    if io::stdin().lock().read_line(&mut input).is_ok() {
        input.trim().parse().ok()
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(line) = input.next() {
        let n: i32 = match line {
            Ok(l) => l.trim().parse().unwrap_or(0),
            Err(_) => break,
        };

        if n <= 0 {
            break; // Invalid number of vertices
        }

        let mut x = vec![0; (n + 1) as usize];
        let mut y = vec![0; (n + 1) as usize];

        for i in 0..n {
            if let Some(Ok(line)) = input.next() {
                let coords: Vec<i32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                x[i as usize] = coords[0];
                y[i as usize] = coords[1];
            }
        }
        x[n as usize] = x[0];
        y[n as usize] = y[0];

        let mut xmin = INF;
        let mut xmax = -INF;
        let mut ymin = INF;
        let mut ymax = -INF;

        for _ in 0..4 {
            if let Some(Ok(line)) = input.next() {
                let coords: Vec<i32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                let a = coords[0];
                let b = coords[1];

                if a < xmin { xmin = a; }
                if a > xmax { xmax = a; }
                if b < ymin { ymin = b; }
                if b > ymax { ymax = b; }
            }
        }

        let mut win: i64 = 0;
        let mut cur: i64 = 0;

        for i in 0..n {
            if x[i as usize] == x[(i + 1) as usize] {
                cur += (x[i as usize] as i64) * (y[i as usize] - y[(i + 1) as usize]) as i64;

                let mut a = x[i as usize];
                if a > xmax { a = xmax; }
                if xmin > a { a = xmin; }

                let mut b = y[i as usize];
                if b > ymax { b = ymax; }
                if ymin > b { b = ymin; }

                let mut c = y[(i + 1) as usize];
                if c > ymax { c = ymax; }
                if ymin > c { c = ymin; }

                win += (a as i64) * (b - c) as i64;
            }
        }

        println!("{}", win - cur);
    }
}
