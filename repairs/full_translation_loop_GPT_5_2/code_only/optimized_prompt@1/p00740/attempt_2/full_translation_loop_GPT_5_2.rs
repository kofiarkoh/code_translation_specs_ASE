use std::io::{self, Read};

fn main() {
    // Tokenize stdin like scanf would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break, // no more input
        };
        let p: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break, // incomplete pair; avoid panic (C would have undefined behavior)
        };

        if n == 0 {
            break;
        }

        let mut i: i32 = 0;
        let mut w: i32 = p;
        let mut c = [0i32; 50];

        loop {
            // In the original problem, n is expected positive; still, guard to avoid %0 panic.
            if n == 0 {
                break;
            }
            let idx = (i.rem_euclid(n)) as usize;

            if w != 0 {
                w -= 1;
                c[idx] += 1;

                if w == 0 {
                    let mut j: i32 = 0;
                    while j < n {
                        if j == i.rem_euclid(n) {
                            j += 1;
                            continue;
                        }
                        if c[j as usize] != 0 {
                            break;
                        }
                        j += 1;
                    }
                    if j == n {
                        out.push_str(&format!("{}\n", i.rem_euclid(n)));
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
