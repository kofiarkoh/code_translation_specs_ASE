// AOJ 1328 Find the Outlier
// Translated from C to Rust

use std::io::{self, BufRead};

fn calc(v: &[f64], n: usize, x: usize, k: usize) -> f64 {
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

    for line in reader.lines() {
        let line = line.unwrap();
        let d: usize = line.trim().parse().unwrap();
        
        if d == 0 {
            break; // Terminate if input integer is zero
        }

        let n = d + 3;
        let mut v = vec![0.0; n];

        for i in 0..n {
            let value: f64 = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            v[i] = value;
        }

        let mut outlier_index = n; // Default to the last index if no outlier is found

        for i in 0..n {
            let mut found_outlier = false;
            for j in 0..n {
                if j != i {
                    let t = calc(&v, n, i, j) - v[j];
                    if t < -0.1 || t > 0.1 {
                        found_outlier = true;
                        break;
                    }
                }
            }
            if found_outlier {
                outlier_index = i;
                break;
            }
        }

        println!("{}", outlier_index);
    }
}
