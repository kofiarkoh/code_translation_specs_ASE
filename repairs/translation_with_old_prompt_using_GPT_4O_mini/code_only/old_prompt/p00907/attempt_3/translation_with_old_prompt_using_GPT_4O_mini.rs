use std::io::{self, BufRead};

fn calc(n: usize, v: &[f64], x: usize, k: usize) -> f64 {
    let mut s = 0.0;

    for i in 0..n {
        if i != k && i != x {
            let mut t = v[i];
            for j in 0..n {
                if j != i && j != k && j != x {
                    t = (t * (k as f64 - j as f64)) / (i as f64 - j as f64);
                }
            }
            s += t;
        }
    }
    s
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let d: usize = line.unwrap().trim().parse().unwrap();
        if d == 0 {
            break;
        }

        let n = d + 3;
        let mut v = vec![0.0; n];

        for i in 0..n {
            v[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
        }

        let mut outlier_index = n; // Default to n, which is out of bounds
        for i in 0..n {
            let mut is_outlier = true; // Assume current index is an outlier
            for j in 0..n {
                if j != i {
                    let t = calc(n, &v, i, j) - v[j];
                    if t >= -0.1 && t <= 0.1 {
                        is_outlier = false; // Found a match, not an outlier
                        break;
                    }
                }
            }
            if is_outlier {
                outlier_index = i; // Update outlier index if confirmed
                break; // Exit loop after finding the first outlier
            }
        }

        println!("{}", outlier_index);
    }
}
