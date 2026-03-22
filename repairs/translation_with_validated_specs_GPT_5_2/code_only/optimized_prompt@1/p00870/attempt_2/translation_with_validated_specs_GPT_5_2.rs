use std::io::{self, Read};

fn main() {
    // Tokenize like scanf("%d %d") and scanf("%s"): whitespace-delimited.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(x) => x.parse::<usize>().unwrap(),
            None => break,
        };
        let m = match it.next() {
            Some(x) => x.parse::<usize>().unwrap(),
            None => break,
        };
        if n == 0 && m == 0 {
            break;
        }

        // Read patterns
        let mut e: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap_or("");
            let b = s.as_bytes().to_vec();
            lens.push(b.len());
            e.push(b);
        }

        // Concatenate m strings into t
        let mut t: Vec<u8> = Vec::new();
        for _ in 0..m {
            let s = it.next().unwrap_or("");
            // The original C code attempted to strip a trailing control char,
            // but with scanf("%s") that cannot occur. We keep exact token bytes.
            t.extend_from_slice(s.as_bytes());
        }
        let l_s = t.len();

        // In the C code, flag is a fixed global: flag[5001][4096] of char.
        // To preserve corner cases (and avoid relying on input constraints),
        // we allocate the same fixed size and only use the needed prefix.
        const MAX_L: usize = 5001;
        const MAX_MASK: usize = 4096;
        let mut flag = vec![vec![0u8; MAX_MASK]; MAX_L];

        let loop_mask = 1usize << n; // expected <= 4096 in original problem
        // If n is larger than 12, the C code would index out of bounds (UB).
        // We emulate the intended AOJ constraints by clamping usage to allocated size.
        let loop_mask = loop_mask.min(MAX_MASK);

        // DP transitions
        let max_i = l_s.min(MAX_L - 1);
        for i in 0..max_i {
            for j in 0..loop_mask {
                if j != 0 && flag[i][j] == 0 {
                    continue;
                }
                let mut mask = 1usize;
                for k in 0..n {
                    if mask >= loop_mask {
                        break; // cannot represent further bits in our table
                    }
                    if (j & mask) != 0 {
                        mask <<= 1;
                        continue;
                    }
                    let l = lens[k];
                    if i + l <= l_s && i + l < MAX_L {
                        if i + l <= t.len() && t[i..i + l] == e[k][..] {
                            let pat = j | mask;
                            if pat < loop_mask {
                                flag[i + l][pat] = 1;
                            }
                        }
                    }
                    mask <<= 1;
                }
            }
        }

        // Count accepting states at any position i (0..=l_s)
        let full = loop_mask.saturating_sub(1);
        let mut cnt: i32 = 0;
        let max_count_i = l_s.min(MAX_L - 1);
        for i in 0..=max_count_i {
            if full < MAX_MASK {
                cnt += flag[i][full] as i32;
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
