use std::io::{self, Read};

fn main() {
    // Match C's scanf-based tokenization.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Read n patterns.
        // Unlike the previous attempt, do NOT cap n or truncate: the specification does not
        // define such behavior. (The original C uses fixed arrays, but intended inputs fit.)
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap();
            let b = s.as_bytes().to_vec();
            lens.push(b.len());
            patterns.push(b);
        }

        // Concatenate m substrings into t (no truncation; spec only says it's built correctly).
        let mut t: Vec<u8> = Vec::new();
        for _ in 0..m {
            let s = it.next().unwrap();
            t.extend_from_slice(s.as_bytes());
        }
        let l_s = t.len();

        // DP over (position, subset-of-patterns-matched).
        // This requires 2^n states; intended constraints (from C) imply n <= 12.
        let loop_masks = 1usize << n;
        let full_mask = loop_masks - 1;

        // Flat byte array for flags: flag[pos * loop_masks + mask] in {0,1}.
        let mut flag = vec![0u8; (l_s + 1) * loop_masks];

        for i in 0..l_s {
            let base_i = i * loop_masks;
            for j in 0..loop_masks {
                if j != 0 && flag[base_i + j] == 0 {
                    continue;
                }
                for k in 0..n {
                    let mask = 1usize << k;
                    if (j & mask) != 0 {
                        continue;
                    }
                    let l = lens[k];
                    if i + l <= l_s && t[i..i + l] == patterns[k][..] {
                        let pat = j | mask;
                        flag[(i + l) * loop_masks + pat] = 1;
                    }
                }
            }
        }

        let mut cnt: i32 = 0;
        for i in 0..=l_s {
            if flag[i * loop_masks + full_mask] != 0 {
                cnt += 1;
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
