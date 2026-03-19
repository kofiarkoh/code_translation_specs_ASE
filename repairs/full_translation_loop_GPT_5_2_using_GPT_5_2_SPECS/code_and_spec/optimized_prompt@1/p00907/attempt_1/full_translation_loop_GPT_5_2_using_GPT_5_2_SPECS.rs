use std::io::{self, Read};

fn calc(n: usize, v: &[f64], x: usize, k: usize) -> f64 {
    // Computes the value predicted at index k by the unique polynomial of degree <= n-3
    // that interpolates all points except indices x and k.
    //
    // This matches the C code's Lagrange-form computation:
    // s = sum_{i != k,x} v[i] * prod_{j != i,k,x} (k-j)/(i-j)
    let mut s = 0.0f64;
    for i in 0..n {
        if i == k || i == x {
            continue;
        }
        let mut t = v[i];
        for j in 0..n {
            if j != i && j != k && j != x {
                let num = (k as f64) - (j as f64);
                let den = (i as f64) - (j as f64);
                t = (t * num) / den;
            }
        }
        s += t;
    }
    s
}

fn main() {
    // Read all input tokens (no special error handling, per spec/C behavior assumptions).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let d_opt = it.next();
        if d_opt.is_none() {
            break;
        }
        let d: i32 = d_opt.unwrap().parse().unwrap();
        if d == 0 {
            break;
        }

        let n = (d as usize) + 3;
        // Spec says n <= 10; C uses fixed array size 10.
        let mut v = vec![0.0f64; n];
        for i in 0..n {
            v[i] = it.next().unwrap().parse().unwrap();
        }

        let mut i_out = 0usize;
        'outer: for i in 0..n {
            for j in 0..=n {
                if j == n {
                    i_out = i;
                    break 'outer; // corresponds to goto done in C
                }
                if j != i {
                    let t = calc(n, &v, i, j) - v[j];
                    if t < -0.1 || t > 0.1 {
                        break; // try next i
                    }
                }
            }
        }

        out.push_str(&format!("{i_out}\n"));
    }

    print!("{out}");
}
