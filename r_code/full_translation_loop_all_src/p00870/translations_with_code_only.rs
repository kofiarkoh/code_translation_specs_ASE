use std::io::{self, Read};

fn main() {
    // scanf-style tokenization
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        let m: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n == 0 && m == 0 {
            break;
        }

        // C arrays imply n<=12, each pattern length <=20, but we don't assume beyond input.
        let mut e: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap_or("");
            let b = s.as_bytes().to_vec();
            lens.push(b.len());
            e.push(b);
        }

        // Build t as a C string: concatenation plus a trailing NUL.
        // This matters because C uses strncmp(&e[k][0], &t[i], l) which can read past
        // the logical end of the concatenation and see '\0' bytes.
        let mut t: Vec<u8> = Vec::new();
        for _ in 0..m {
            let s = it.next().unwrap_or("");
            t.extend_from_slice(s.as_bytes());
        }
        t.push(0); // NUL terminator like C's t buffer after strcat
        let l_s = t.len() - 1; // strlen(t) in C (excluding NUL)

        // Fixed-size flag like the C global: flag[5001][4096]
        const ROWS: usize = 5001;
        const COLS: usize = 4096;
        let mut flag = vec![vec![0u8; COLS]; ROWS];

        // loop = 1<<n (C int). For n>12, C would index beyond 4096 and invoke UB.
        // We clamp to 4096 to avoid panics while staying close for valid inputs.
        let loop_states = (1usize << n).min(COLS);
        let full = loop_states.saturating_sub(1);

        // DP
        // In C, i ranges 0..l_s-1, and i+l can exceed l_s (up to 5000) and is still written.
        // Also, strncmp compares l bytes even if it crosses the NUL terminator; it will then
        // compare against '\0' bytes from the t buffer.
        for i in 0..l_s {
            if i >= ROWS {
                break; // avoid OOB; C would be UB for oversized inputs
            }
            for j in 0..loop_states {
                if j != 0 && flag[i][j] == 0 {
                    continue;
                }
                for k in 0..n {
                    let mask = 1usize << k;
                    if (j & mask) != 0 {
                        continue;
                    }
                    let l = lens[k];

                    // Emulate strncmp(e[k], &t[i], l) with t being NUL-terminated.
                    // If i+l goes beyond t.len(), C would read beyond the buffer (UB).
                    // We guard to avoid panic; for valid inputs this won't trigger.
                    if i + l <= t.len() {
                        if t[i..i + l] == e[k][..] {
                            let pat = j | mask;
                            let idx = i + l;
                            if idx < ROWS && pat < COLS {
                                flag[idx][pat] = 1;
                            }
                        }
                    }
                }
            }
        }

        // Count flag[i][full] for i=0..=l_s (as in C)
        let mut cnt: i32 = 0;
        let upper = l_s.min(ROWS - 1);
        for i in 0..=upper {
            cnt += flag[i][full] as i32;
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
