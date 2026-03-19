use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more input; spec assumes valid input, but this is harmless.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Compute column sums directly (no need to store the whole matrix).
        let mut cnt = vec![0i64; m];
        for _ in 0..n {
            for j in 0..m {
                let v: i64 = it.next().unwrap().parse().unwrap();
                cnt[j] += v;
            }
        }

        // Output column indices sorted by descending sum, using the C code's
        // selection behavior: repeatedly pick the unpicked column with maximum sum;
        // ties are resolved by the smallest index (because the scan is left-to-right).
        let mut picked = vec![false; m];
        for i in 0..m {
            if i > 0 {
                out.push(' ');
            }
            let mut best: Option<usize> = None;
            for j in 0..m {
                if !picked[j] {
                    best = match best {
                        None => Some(j),
                        Some(b) => {
                            if cnt[b] < cnt[j] {
                                Some(j)
                            } else {
                                Some(b)
                            }
                        }
                    };
                }
            }
            let best = best.unwrap();
            out.push_str(&(best + 1).to_string()); // 1-based index
            picked[best] = true;
        }
        out.push('\n');
    }

    print!("{out}");
}
