use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // malformed/partial input: unspecified; just stop
        }
        let n: usize = n_opt.unwrap().parse().unwrap();

        let p_opt = it.next();
        if p_opt.is_none() {
            break; // malformed/partial input: unspecified; just stop
        }
        let p: i64 = p_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // Fixed storage up to 50 participants as per constraints.
        let mut held = [0i64; 50];
        for i in 0..n {
            held[i] = 0;
        }

        let mut pool: i64 = p;
        let mut k: usize = 0;

        loop {
            if pool > 0 {
                pool -= 1;
                held[k] += 1;

                // Terminate immediately after a transfer that makes pool zero
                // and current participant holds all p tokens.
                if pool == 0 && held[k] == p {
                    println!("{}", k);
                    break;
                }
            } else {
                // pool == 0: move all tokens from participant k back to pool
                pool += held[k];
                held[k] = 0;
            }

            k += 1;
            if k == n {
                k = 0;
            }
        }
    }
}
