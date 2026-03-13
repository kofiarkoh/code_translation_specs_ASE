use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Hoge {
    n: i32,
    count: i32,
}

fn main() {
    // Read all input tokens as i32
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // s and sn mimic the C globals:
    // s is an array of 13 hoge structs, zero-initialized like in C globals
    let mut s: [Hoge; 13] = [Hoge { n: 0, count: 0 }; 13];
    let mut sn: usize = 0; // corresponds to global 'sn' in C (zero-initialized)

    loop {
        // Read n
        let n = match it.next() {
            Some(v) => v,
            None => break, // EOF
        };
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        // Read sequence of length n
        let mut seq: [i32; 13] = [0; 13];
        for i in 0..n_usize {
            // In the C code scanf used "%d " (with trailing space). Here we
            // consume whitespace-separated tokens which preserves typical behavior
            seq[i] = it.next().unwrap();
        }

        // The core iterative process: replace each element by its frequency
        // among the current sequence until it becomes fixed.
        let mut iter_count: i32 = 0;
        for i in 0.. {
            // The C code has `for(j=0;j<n;j++) fix=1;` which ends up setting fix=1.
            // We emulate that directly.
            let mut fix: i32 = 1;

            // First pass: build counts into s[0..sn) — but sn is reset to 0 at the
            // start of each iteration in the C code logic after the second pass.
            // Here we follow the same logic: use current sn (which should be 0),
            // but we will append entries as we find them.
            // We'll treat sn as starting at 0 for this iteration.
            // (In C, sn is set to 0 at end of each iteration; it is also 0 before first)
            // To match behavior, ensure we start counting from current sn value (initialized 0).
            // Actually, in the C code sn is set to 0 at the end of the previous iteration
            // and before the next usage, so it is 0 here. We preserve the same variable.
            // We'll use and mutate sn as in C.
            // (sn may have non-zero garbage from previous testcases only if not reset;
            // but C initialized it to 0 as global, and also sets sn=0 in the loop,
            // so this matches.)
            // For safety, ensure sn starts at 0 for this transformation loop:
            // (sn already is 0 here according to the logic; but keep as-is)

            // Build counts
            // We'll search s[0..sn) for seq[j], incrementing count if found or adding new entry.
            for j in 0..n_usize {
                let val = seq[j];
                let mut found = false;
                for k in 0..sn {
                    if s[k].n == val {
                        s[k].count = s[k].count.wrapping_add(1); // mimic C int increment (wrap on overflow)
                        found = true;
                        break;
                    }
                }
                if !found {
                    // add new entry
                    if sn < s.len() {
                        s[sn].n = val;
                        s[sn].count = 1;
                        sn += 1;
                    } else {
                        // In the original C code array is size 13; if more distinct values appear,
                        // it would overflow. Here we avoid panic by ignoring extra values,
                        // but this situation should not occur given problem constraints.
                    }
                }
            }

            // Second pass: replace seq[j] with its count from s and check whether fixed
            for j in 0..n_usize {
                let val = seq[j];
                let mut replaced = false;
                for k in 0..sn {
                    if s[k].n == val {
                        if val != s[k].count {
                            fix = 0;
                        }
                        seq[j] = s[k].count;
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    // In C, this should not happen because every seq[j] was added to s above.
                }
            }

            // Reset sn as in C
            sn = 0;

            // If fixed, break and report the iteration count (i)
            if fix != 0 {
                iter_count = i as i32;
                break;
            }
            // else continue to next iteration
        }

        // Print iteration count (one per line)
        println!("{}", iter_count);

        // Before printing sequence, the C code sets s[i].n = s[i].count = -1 for i in 0..n-1
        // (i.e., indices 0..=n-2). We replicate that side-effect.
        if n_usize >= 2 {
            for idx in 0..(n_usize - 1) {
                s[idx].n = -1;
                s[idx].count = -1;
            }
        }

        // Print sequence elements separated by single spaces, ending with newline.
        for idx in 0..(n_usize - 1) {
            print!("{} ", seq[idx]);
        }
        // The last element (for n>=1) — in C, due to loop semantics, the index used is n-1.
        println!("{}", seq[n_usize - 1]);
    }
}
