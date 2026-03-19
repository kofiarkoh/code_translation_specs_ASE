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
            break; // no more input
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let p: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break, // incomplete pair; undefined in C, we just stop
        };

        if n == 0 {
            break; // terminating pair produces no output
        }

        // C uses fixed array size 50; spec says n <= 50 for supported scenarios.
        let n_usize = n as usize;
        let mut c = vec![0i32; 50];

        let mut i: i32 = 0;
        let mut w: i32 = p;

        loop {
            let idx = (i.rem_euclid(n)) as usize;

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
