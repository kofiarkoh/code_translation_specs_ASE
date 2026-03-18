use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        // Terminate when both n and m are zero.
        if n == 0 && m == 0 {
            break;
        }

        // Read n pattern strings.
        let mut patterns: Vec<String> = Vec::with_capacity(n);
        for _ in 0..n {
            patterns.push(it.next().unwrap().to_string());
        }

        // Read m substrings and concatenate them.
        let mut concat = String::new();
        for _ in 0..m {
            concat.push_str(it.next().unwrap());
        }
        let bytes = concat.as_bytes();
        let len = bytes.len();

        // Flag structure: for each position i in concat, flags[i] is a bitset of which patterns
        // match as a prefix starting at i. Bit j corresponds to patterns[j].
        //
        // We use u128 to keep it simple and safe; if n > 128, we fall back to Vec<bool> per position.
        let ans = if n <= 128 {
            let mut flags: Vec<u128> = vec![0u128; len + 1];

            for (j, pat) in patterns.iter().enumerate() {
                let p = pat.as_bytes();
                let plen = p.len();
                if plen == 0 {
                    // Empty string is a prefix at every position.
                    let bit = 1u128 << j;
                    for i in 0..=len {
                        flags[i] |= bit;
                    }
                    continue;
                }
                if plen > len {
                    continue;
                }
                let bit = 1u128 << j;
                for i in 0..=len.saturating_sub(plen) {
                    if &bytes[i..i + plen] == p {
                        flags[i] |= bit;
                    }
                }
            }

            let all_mask: u128 = if n == 128 { u128::MAX } else { (1u128 << n) - 1 };

            let mut count: u64 = 0;
            for i in 0..=len {
                if (flags[i] & all_mask) == all_mask {
                    count += 1;
                }
            }
            count.to_string()
        } else {
            // Narrowest reasonable fallback when n is large: store per-position Vec<bool>.
            // This preserves semantics but may be slower/more memory-heavy.
            let mut flags: Vec<Vec<bool>> = vec![vec![false; n]; len + 1];

            for (j, pat) in patterns.iter().enumerate() {
                let p = pat.as_bytes();
                let plen = p.len();
                if plen == 0 {
                    for i in 0..=len {
                        flags[i][j] = true;
                    }
                    continue;
                }
                if plen > len {
                    continue;
                }
                for i in 0..=len.saturating_sub(plen) {
                    if &bytes[i..i + plen] == p {
                        flags[i][j] = true;
                    }
                }
            }

            let mut count: u64 = 0;
            'pos: for i in 0..=len {
                for j in 0..n {
                    if !flags[i][j] {
                        continue 'pos;
                    }
                }
                count += 1;
            }
            count.to_string()
        };

        outputs.push(ans);
    }

    print!("{}", outputs.join("\n"));
}
