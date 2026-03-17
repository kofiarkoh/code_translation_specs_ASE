// AOJ 1328 Find the Outlier
// 2018.2.9 bal4u

use std::io::{self, BufRead};

fn calc(x: usize, k: usize, n: usize, v: &[f64]) -> f64 {
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
        let d: i32 = line.unwrap().trim().parse().unwrap();
        if d == 0 {
            break;
        }

        let n = (d + 3) as usize;
        let mut v = vec![0.0; n];

        for i in 0..n {
            v[i] = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
        }

        let mut outlier_index = n; // Default to n, which indicates no outlier found

        for i in 0..n {
            let mut found = false;
            for j in 0..n {
                if j != i {
                    let t = calc(i, j, n, &v) - v[j];
                    if t < -0.1 || t > 0.1 {
                        found = true;
                        break;
                    }
                }
            }
            if found {
                outlier_index = i;
                break;
            }
        }

        println!("{}", outlier_index);
    }
}
