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

    for line in reader.lines() {
        let line = line.unwrap();
        let d: usize = line.trim().parse().unwrap();
        if d == 0 {
            break;
        }

        let n = d + 3;
        let mut v = vec![0.0; n];

        for i in 0..n {
            let value: f64 = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            v[i] = value;
        }

        let mut outlier_index = n - 1; // Default to the last index
        for i in 0..n {
            let mut found_outlier = false;
            for j in 0..n {
                if j != i {
                    let t = calc(n, &v, i, j) - v[j];
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
