use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
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
        let p: i32 = it.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut i: i32 = 0;
        let mut w: i32 = p;
        let mut c = [0i32; 50]; // matches C's int c[50]
        // MEMSET(c) => already zeroed

        loop {
            let idx = (i % n) as usize;

            if w != 0 {
                w -= 1;
                c[idx] += 1;

                if w == 0 {
                    // rep(j,n) with early breaks
                    let mut j: i32 = 0;
                    while j < n {
                        if j == (i % n) {
                            j += 1;
                            continue;
                        }
                        if c[j as usize] != 0 {
                            break;
                        }
                        j += 1;
                    }
                    if j == n {
                        out.push_str(&format!("{}\n", i % n));
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
