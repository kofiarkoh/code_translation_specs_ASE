use std::io::{self, BufRead};

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of nodes and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize distance array and edge array
    let mut d = vec![f64::INFINITY; n + 1];
    let mut e = vec![-1; n + 1];
    let mut c = vec![vec![0; n + 1]; n + 1];

    d[n] = 0.0;

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let i: usize = edge_parts.next().unwrap().parse().unwrap();
        let j: usize = edge_parts.next().unwrap().parse().unwrap();
        c[i][j] = 1;
    }

    // Calculate minimum average distances
    let mut min_distance = d[1];
    for i in (1..n).rev() {
        let mut s = 0.0;
        let mut a = 0;
        let mut b = 0;

        for j in i..=n {
            if c[i][j] == 1 {
                s += d[j] + 1.0;
                if b == 0 || d[b] < d[j] {
                    b = j;
                }
                a += 1;
            }
        }

        if a > 0 {
            d[i] = s / a as f64;
            if a > 1 {
                e[i] = b;
            }
        }
    }

    min_distance = d[1];

    // Check for valid paths excluding specific edges
    for k in 1..=n {
        if e[k] == -1 {
            continue;
        }

        // Reset distances for the next iteration
        for j in 0..=n {
            d[j] = f64::INFINITY;
        }
        d[n] = 0.0;

        for i in (1..n).rev() {
            let mut s = 0.0;
            let mut a = 0;

            for j in i..=n {
                if c[i][j] == 0 || (i == k && j == e[k]) {
                    continue;
                }
                s += d[j] + 1.0;
                a += 1;
            }

            if a > 0 {
                d[i] = s / a as f64;
            }
        }

        if min_distance > d[1] {
            min_distance = d[1];
        }
    }

    // Print the result formatted to ten decimal places
    println!("{:.10}", min_distance);
}
