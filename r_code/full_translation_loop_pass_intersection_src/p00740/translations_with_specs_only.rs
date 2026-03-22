use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();

        let m_opt = it.next();
        if m_opt.is_none() {
            break;
        }
        let m: i32 = m_opt.unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // Specification preconditions: n > 0, n <= 50, m >= 0.
        // No explicit error handling is required for violations.
        let n_usize = n as usize;

        // Fixed storage for up to 50 participants as per constraints.
        // We still size to n for the scenario; behavior for n > 50 is unspecified.
        let mut holdings = vec![0i32; n_usize];

        let mut pool = m;
        let mut idx: usize = 0;

        // Termination is only guaranteed by the spec when initial pool is positive.
        loop {
            if pool > 0 {
                pool -= 1;
                holdings[idx] += 1;
            } else {
                pool += holdings[idx];
                holdings[idx] = 0;
            }

            // Terminate at the first step where pool == 0 and exactly one participant is nonzero.
            if pool == 0 {
                let mut nonzero_count = 0usize;
                let mut last_nonzero_idx = 0usize;
                for (i, &h) in holdings.iter().enumerate() {
                    if h != 0 {
                        nonzero_count += 1;
                        last_nonzero_idx = i;
                        if nonzero_count > 1 {
                            break;
                        }
                    }
                }
                if nonzero_count == 1 {
                    outputs.push(last_nonzero_idx.to_string());
                    break;
                }
            }

            idx += 1;
            if idx == n_usize {
                idx = 0;
            }
        }
    }

    print!("{}", outputs.join("\n"));
}
