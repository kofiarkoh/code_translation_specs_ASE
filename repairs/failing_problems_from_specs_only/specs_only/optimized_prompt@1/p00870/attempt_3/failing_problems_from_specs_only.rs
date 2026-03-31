use std::io::{self, Read};

fn main() {
    // Specification: all strings are read as whitespace-delimited tokens.
    // We therefore use split_whitespace() for tokenization.
    // The additional rule about stripping a trailing ASCII < ' ' from each text fragment
    // is effectively a no-op under whitespace tokenization, but we still implement it
    // defensively on the token bytes.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut first = true;
    let mut out = String::new();

    loop {
        let n: usize = match it.next() {
            None => break,
            Some(x) => x.parse().unwrap(),
        };
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            patterns.push(it.next().unwrap().as_bytes().to_vec());
        }

        let mut text: Vec<u8> = Vec::new();
        for _ in 0..m {
            let mut frag = it.next().unwrap().as_bytes().to_vec();
            if let Some(&last) = frag.last() {
                if last < b' ' {
                    frag.pop();
                }
            }
            text.extend_from_slice(&frag);
        }

        let l = text.len();
        let mask_count = 1usize << n;
        let full_mask = if n == 0 { 0 } else { mask_count - 1 };

        // Reachability states (P, S). Use a flat boolean table dp[P][S].
        let mut dp = vec![false; (l + 1) * mask_count];
        dp[0 * mask_count + 0] = true;

        // Propagation: scan P increasing, for each reachable subset S at P, try unused patterns.
        for p in 0..l {
            let row = p * mask_count;
            for s in 0..mask_count {
                if !dp[row + s] {
                    continue;
                }
                for k in 0..n {
                    let bit = 1usize << k;
                    if (s & bit) != 0 {
                        continue;
                    }
                    let pat = &patterns[k];
                    let lenk = pat.len();
                    if p + lenk > l {
                        continue;
                    }
                    if &text[p..p + lenk] == &pat[..] {
                        let np = p + lenk;
                        let ns = s | bit;
                        dp[np * mask_count + ns] = true;
                    }
                }
            }
        }

        let mut count = 0usize;
        for p in 0..=l {
            if dp[p * mask_count + full_mask] {
                count += 1;
            }
        }

        if !first {
            out.push('\n');
        }
        first = false;
        out.push_str(&count.to_string());
    }

    print!("{out}");
}
