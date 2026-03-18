use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d %d") and scanf("%s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Read n patterns (max length 20 per spec; C code uses up to 12 patterns).
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap();
            let bytes = s.as_bytes().to_vec();
            lens.push(bytes.len());
            patterns.push(bytes);
        }

        // Concatenate m substrings into t.
        // Note: The C code has a check for trailing control char in buf, but with scanf("%s")
        // and our split_whitespace parsing, such characters are not present. Behavior matches
        // intended token-based concatenation.
        let mut t: Vec<u8> = Vec::new();
        for _ in 0..m {
            let s = it.next().unwrap();
            t.extend_from_slice(s.as_bytes());
        }
        let l_s = t.len();

        // DP flags: flag[pos][mask] = reachable.
        // C uses fixed [5001][4096]; we size dynamically to preserve semantics for given input.
        let loop_masks = 1usize << n;
        let mut flag = vec![vec![false; loop_masks]; l_s + 1];

        for i in 0..l_s {
            for j in 0..loop_masks {
                if j != 0 && !flag[i][j] {
                    continue;
                }
                let mut mask = 1usize;
                for k in 0..n {
                    if (j & mask) == 0 {
                        let l = lens[k];
                        if i + l <= l_s && t[i..i + l] == patterns[k][..] {
                            let pat = j | mask;
                            flag[i + l][pat] = true;
                        }
                    }
                    mask <<= 1;
                }
            }
        }

        let full = loop_masks - 1;
        let mut cnt: i32 = 0;
        for i in 0..=l_s {
            if flag[i][full] {
                cnt += 1;
            }
        }

        println!("{}", cnt);
    }
}
