use std::io::{self, Read};

static mut N: i32 = 0;
static mut V: [f64; 10] = [0.0; 10];

fn calc(x: i32, k: i32) -> f64 {
    // Mirrors the C logic exactly; uses global N and V.
    let n = unsafe { N } as i32;
    let v = unsafe { &V };

    let mut s: f64 = 0.0;
    for i in 0..n {
        if i != k && i != x {
            let mut t = v[i as usize];
            for j in 0..n {
                if j != i && j != k && j != x {
                    // All arithmetic is in f64, as in C (t is double).
                    t = (t * ((k - j) as f64)) / ((i - j) as f64);
                }
            }
            s += t;
        }
    }
    s
}

fn main() {
    // Read entire stdin and parse as whitespace-separated tokens (like scanf).
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

        let n = d + 3;
        unsafe {
            N = n;
        }
        for i in 0..n {
            let val: f64 = it.next().unwrap().parse().unwrap();
            unsafe {
                V[i as usize] = val;
            }
        }

        let mut i_found: i32 = 0;
        'outer: for i in 0..n {
            let mut j: i32 = 0;
            loop {
                if j == n {
                    i_found = i;
                    break 'outer; // corresponds to goto done;
                }
                if j != i {
                    let t = calc(i, j) - unsafe { V[j as usize] };
                    if t < -0.1 || t > 0.1 {
                        break; // break inner loop, continue with next i
                    }
                }
                j += 1;
            }
        }

        out.push_str(&format!("{}\n", i_found));
    }

    print!("{out}");
}
