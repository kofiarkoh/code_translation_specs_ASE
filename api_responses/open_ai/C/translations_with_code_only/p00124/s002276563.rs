use std::io::{self, Read};

fn main() {
    // Read all input tokens like scanf("%s"/"%d") would (whitespace-separated)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut toks = input.split_whitespace();

    let mut cnt: i32 = 0;

    // Helper to parse next token as i32
    let mut next_i32 = || -> Option<i32> {
        toks.next().and_then(|s| s.parse::<i32>().ok())
    };

    // Read initial n
    let mut n = match next_i32() {
        Some(v) => v,
        None => return, // no input
    };

    while n != 0 {
        if cnt > 0 {
            // Print a single blank line between test cases (printf("\n"))
            println!();
        }

        let mut entries: Vec<(String, i32)> = Vec::new();

        if n > 0 {
            let m = n as usize;
            for _ in 0..m {
                // Read string and three ints like scanf("%s %d %d %d", ...)
                let name = match toks.next() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };
                // parse a, b, c as i32; if missing tokens, treat as 0 (scanf would block/undefined,
                // but we conservatively stop parsing here)
                let a = toks.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                let _b = toks.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                let c = toks.next().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

                // Compute ten = a*3 + b*0 + c*1.
                // Use wrapping arithmetic to emulate C-like (two's complement) wraparound behavior.
                let ten = a.wrapping_mul(3).wrapping_add(c);
                entries.push((name, ten));
            }

            // Perform the same pairwise swapping sort as the C code:
            // for(i=0;i<n-1;i++) for(j=i+1;j<n;j++) if(ten[i]<ten[j]) swap
            let m = entries.len();
            for i in 0..m.saturating_sub(1) {
                for j in (i + 1)..m {
                    if entries[i].1 < entries[j].1 {
                        entries.swap(i, j);
                    }
                }
            }

            // Print results: "%s,%d\n"
            for (name, ten) in entries {
                println!("{},{}", name, ten);
            }
        } else {
            // n < 0: C code's for-loops would simply be skipped (no reads), so do nothing.
        }

        cnt += 1;

        // Read next n; if absent, break
        n = match next_i32() {
            Some(v) => v,
            None => break,
        };
    }
}
