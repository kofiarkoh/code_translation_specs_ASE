use std::io::{self, Read};

fn calc(n: usize, v: &[f64], x: usize, k: usize) -> f64 {
    let mut s = 0.0f64;
    for i in 0..n {
        if i == k || i == x {
            continue;
        }
        let mut t = v[i];
        for j in 0..n {
            if j != i && j != k && j != x {
                // All indices are distinct here, so (i - j) != 0.
                t = (t * ((k as f64) - (j as f64))) / ((i as f64) - (j as f64));
            }
        }
        s += t;
    }
    s
}

fn main() {
    // Read all input tokens.
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

        // Spec: if input integer is zero, terminate without processing any values.
        if d == 0 {
            break;
        }

        // C code behavior: n = d + 3, and reads n floating-point numbers.
        // Spec says "reads an integer ... followed by that many floating-point numbers",
        // but this conflicts with the provided C code. We follow the C code as the
        // source-of-truth for actual I/O shape and semantics.
        let n = (d as usize) + 3;
        let mut v = vec![0.0f64; n];
        for i in 0..n {
            v[i] = it.next().unwrap().parse::<f64>().unwrap();
        }

        let mut i_out = n; // will be set before printing
        'done: for i in 0..n {
            for j in 0..=n {
                if j == n {
                    i_out = i;
                    break 'done;
                }
                if j != i {
                    let t = calc(n, &v, i, j) - v[j];
                    if t < -0.1 || t > 0.1 {
                        break;
                    }
                }
            }
        }

        out.push_str(&format!("{i_out}\n"));
    }

    print!("{out}");
}
