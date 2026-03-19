use std::io::{self, Read};

fn main() {
    // Narrowest interpretation consistent with the specification:
    // Count positions in the concatenated string where *all* n given strings match as prefixes
    // starting at that position. (I.e., for a position i, every pattern must match s[i..].)
    //
    // This matches "how many times the entire set of strings can be found as prefixes in the
    // concatenated string" and the described "flag structure that tracks the presence of prefixes
    // for each position".
    //
    // Input terminates when n == 0 && m == 0.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: usize = n_tok.parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            patterns.push(it.next().unwrap().as_bytes().to_vec());
        }

        let mut s: Vec<u8> = Vec::new();
        for _ in 0..m {
            s.extend_from_slice(it.next().unwrap().as_bytes());
        }
        let len = s.len();

        // flags[i] = how many patterns match as a prefix at position i.
        // Using u32 is sufficient for typical constraints; semantics are just counting.
        let mut flags: Vec<u32> = vec![0; len + 1];

        for pat in &patterns {
            let plen = pat.len();
            if plen == 0 {
                // Empty string is a prefix at every position including len.
                for f in &mut flags {
                    *f += 1;
                }
                continue;
            }
            if plen > len {
                continue;
            }
            for i in 0..=len - plen {
                if &s[i..i + plen] == &pat[..] {
                    flags[i] += 1;
                }
            }
        }

        let mut count: u64 = 0;
        let need = n as u32;
        for i in 0..=len {
            if flags[i] == need {
                count += 1;
            }
        }

        outputs.push(count.to_string());
    }

    print!("{}", outputs.join("\n"));
}
