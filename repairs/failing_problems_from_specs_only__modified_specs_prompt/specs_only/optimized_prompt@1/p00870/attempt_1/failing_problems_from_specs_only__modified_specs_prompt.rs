use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as bytes; tokens are whitespace-delimited.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // unspecified behavior on premature EOF; we just stop
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Read patterns as raw bytes (exact byte-for-byte matching).
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap();
            patterns.push(s.as_bytes().to_vec());
        }

        // Build target by concatenating normalized fragments.
        // Normalization: if last character has ASCII code < ' ' (0x20), remove it.
        // With split_whitespace(), such control chars typically won't appear as token content,
        // but we implement the rule exactly on the token bytes we receive.
        let mut target: Vec<u8> = Vec::new();
        for _ in 0..m {
            let frag = it.next().unwrap().as_bytes().to_vec();
            let mut frag_norm = frag;
            if let Some(&last) = frag_norm.last() {
                if last < b' ' {
                    frag_norm.pop();
                }
            }
            target.extend_from_slice(&frag_norm);
        }

        let l = target.len();
        let all_mask: usize = if n == 0 { 0 } else { (1usize << n) - 1 };

        // Reachability table: reachable[i][mask] where 0<=i<=L and 0<=mask<2^n.
        // n <= 12, L <= 5000 => (L+1)*4096 booleans is feasible.
        let masks = 1usize << n;
        let mut reachable = vec![vec![false; masks]; l + 1];

        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        reachable[0][0] = true;
        q.push_back((0, 0));

        while let Some((i, mask)) = q.pop_front() {
            // Try each unused pattern.
            for k in 0..n {
                let bit = 1usize << k;
                if (mask & bit) != 0 {
                    continue;
                }
                let pat = &patterns[k];
                let plen = pat.len();
                if i + plen > l {
                    continue;
                }
                // Exact match at position i.
                if target[i..i + plen] == pat[..] {
                    let ni = i + plen;
                    let nmask = mask | bit;
                    if !reachable[ni][nmask] {
                        reachable[ni][nmask] = true;
                        q.push_back((ni, nmask));
                    }
                }
            }
        }

        // Count indices p such that (p, all_mask) is reachable.
        let mut count = 0usize;
        for p in 0..=l {
            if reachable[p][all_mask] {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
