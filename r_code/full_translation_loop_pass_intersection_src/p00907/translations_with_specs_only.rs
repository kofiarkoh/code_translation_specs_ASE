use std::io::{self, Read};

fn lagrange_predict(x: f64, xs: &[f64], ys: &[f64]) -> f64 {
    // Unique polynomial of degree <= d through (xs[t], ys[t]) for t=0..d
    // Evaluate at x using Lagrange form.
    let m = xs.len();
    let mut sum = 0.0f64;
    for a in 0..m {
        let mut term = ys[a];
        for b in 0..m {
            if a == b {
                continue;
            }
            term *= (x - xs[b]) / (xs[a] - xs[b]);
        }
        sum += term;
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let d_opt = it.next();
        if d_opt.is_none() {
            break;
        }
        let d: usize = d_opt.unwrap().parse().unwrap();
        if d == 0 {
            break;
        }

        let n = d + 3;
        let mut v = vec![0.0f64; n];
        for k in 0..n {
            v[k] = it.next().unwrap().parse::<f64>().unwrap();
        }

        let tol = 0.1f64;
        let mut answer: usize = 0;

        'outer: for i in 0..n {
            for j in 0..n {
                if j == i {
                    continue;
                }

                // Build the d+1 points excluding i and j.
                let mut xs: Vec<f64> = Vec::with_capacity(d + 1);
                let mut ys: Vec<f64> = Vec::with_capacity(d + 1);
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }
                    xs.push(k as f64);
                    ys.push(v[k]);
                }

                // Predict at index j.
                let pred = lagrange_predict(j as f64, &xs, &ys);
                if (pred - v[j]).abs() > tol {
                    continue 'outer;
                }
            }
            answer = i;
            break;
        }

        println!("{answer}");
    }
}
