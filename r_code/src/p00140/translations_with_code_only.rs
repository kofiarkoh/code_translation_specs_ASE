use std::io::{self, Read};

fn main() {
    // Read entire stdin (similar to repeated scanf calls in the C program).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // Parse n as i32 to preserve C-like signed-int behavior (e.g. negative n -> no iterations).
    let n_i32: i32 = match iter.next() {
        Some(s) => s.parse().unwrap_or(0),
        None => return,
    };

    // Buffer output to preserve exact formatting and be efficient.
    let mut out = String::new();

    if n_i32 > 0 {
        for _ in 0..(n_i32 as usize) {
            // If there aren't two more tokens, break (scanf in C would block/behave differently,
            // but typical inputs are well-formed; breaking is a pragmatic choice).
            let from_s = match iter.next() {
                Some(s) => s,
                None => break,
            };
            let to_s = match iter.next() {
                Some(s) => s,
                None => break,
            };
            let from: i32 = from_s.parse().unwrap();
            let to: i32 = to_s.parse().unwrap();

            if from < to {
                // for(j=from;j<to;j++){ printf("%d ",j); } printf("%d\n",to);
                let mut j = from;
                while j < to {
                    out.push_str(&format!("{} ", j));
                    j += 1;
                }
                out.push_str(&format!("{}\n", to));
            } else if to == 5 {
                // for(j=from;j<=9;j++){ printf("%d ",j); } puts("5");
                let mut j = from;
                while j <= 9 {
                    out.push_str(&format!("{} ", j));
                    j += 1;
                }
                out.push_str("5\n");
            } else if from > to {
                if (6..=9).contains(&from) {
                    // for(j=from;j<=9;j++){ printf("%d ",j); }
                    let mut j = from;
                    while j <= 9 {
                        out.push_str(&format!("{} ", j));
                        j += 1;
                    }
                    // for(j=5;j>to;j--){ printf("%d ",j); }
                    let mut j = 5;
                    while j > to {
                        out.push_str(&format!("{} ", j));
                        j -= 1;
                    }
                    // printf("%d\n",to);
                    out.push_str(&format!("{}\n", to));
                } else {
                    // for(j=from;j>to;j--){ printf("%d ",j); } printf("%d\n",to);
                    let mut j = from;
                    while j > to {
                        out.push_str(&format!("{} ", j));
                        j -= 1;
                    }
                    out.push_str(&format!("{}\n", to));
                }
            }
            // If from == to, the C code prints nothing; we do the same (no else branch).
        }
    }

    // Print the buffered output.
    print!("{}", out);
}
