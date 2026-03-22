use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // malformed/short input: undefined by spec; just stop
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();

        let p_opt = it.next();
        if p_opt.is_none() {
            break; // malformed/short input: undefined by spec; just stop
        }
        let p: i32 = p_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // Spec preconditions: n > 0, n <= 50, p >= 0. No explicit error handling.
        let n_usize = n as usize;

        let mut c = vec![0i32; 50]; // fixed storage up to 50 participants (as in C)
        let mut w = p; // pool
        let mut i: i32 = 0;

        loop {
            let idx = (i as usize) % n_usize;

            if w != 0 {
                w -= 1;
                c[idx] += 1;

                if w == 0 {
                    // Check if all other participants have zero.
                    let mut j = 0usize;
                    while j < n_usize {
                        if j != idx && c[j] != 0 {
                            break;
                        }
                        j += 1;
                    }
                    if j == n_usize {
                        out.push_str(&format!("{}\n", idx));
                        break;
                    }
                }
            } else {
                w += c[idx];
                c[idx] = 0;
            }

            i += 1;
        }
    }

    print!("{out}");
}
