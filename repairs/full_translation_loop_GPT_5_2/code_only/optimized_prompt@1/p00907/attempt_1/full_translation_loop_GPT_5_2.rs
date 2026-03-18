use std::io::{self, Read};

static mut N: i32 = 0;
static mut V: [f64; 10] = [0.0; 10];

fn calc(x: i32, k: i32) -> f64 {
    // Uses global N and V exactly like the C code.
    let mut s = 0.0f64;
    unsafe {
        let n = N;
        for i in 0..n {
            if i != k && i != x {
                let mut t = V[i as usize];
                for j in 0..n {
                    if j != i && j != k && j != x {
                        // Preserve floating-point behavior (including division by zero -> inf/NaN)
                        // as in C's double arithmetic.
                        t = (t * ((k - j) as f64)) / ((i - j) as f64);
                    }
                }
                s += t;
            }
        }
    }
    s
}

fn main() {
    // Read all input, parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let d_opt = it.next();
        if d_opt.is_none() {
            break; // scanf would fail -> loop ends
        }
        let d: i32 = d_opt.unwrap().parse().unwrap();
        if d == 0 {
            break; // while (scanf(...) && d)
        }

        let n = d + 3;
        unsafe {
            N = n;
            for i in 0..n {
                let val: f64 = it.next().unwrap().parse().unwrap();
                V[i as usize] = val;
            }
        }

        // Replicate the C control flow with a labeled break (goto done).
        let mut i_done: i32 = 0;
        'outer: for i in 0..n {
            let mut j: i32 = 0;
            loop {
                if j == n {
                    i_done = i;
                    break 'outer; // goto done
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

        out.push_str(&format!("{}\n", i_done));
    }

    print!("{out}");
}
