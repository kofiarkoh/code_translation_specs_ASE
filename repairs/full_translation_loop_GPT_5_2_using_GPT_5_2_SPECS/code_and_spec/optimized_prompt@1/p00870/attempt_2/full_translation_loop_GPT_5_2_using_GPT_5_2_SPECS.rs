use std::io::{self, Read};

fn main() {
    // Token-based input, equivalent to scanf("%d %d") and scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: usize = n_tok.parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        if n == 0 && m == 0 {
            break;
        }

        // SPEC requires n <= 12; we assume input respects it.
        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        let mut lens: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = it.next().unwrap();
            let b = s.as_bytes().to_vec();
            lens.push(b.len());
            patterns.push(b);
        }

        // Concatenate fragments to form target text.
        let mut text: Vec<u8> = Vec::new();
        for _ in 0..m {
            let frag = it.next().unwrap();
            text.extend_from_slice(frag.as_bytes());
        }
        let l_s = text.len();

        let states = 1usize << n;
        let full = states - 1;

        // Reachability table: flag[pos][mask] == true if reachable.
        // Follow SPEC: sequences must start at position 0 with empty used-set.
        let mut flag = vec![vec![false; states]; l_s + 1];
        flag[0][0] = true;

        for i in 0..l_s {
            for mask in 0..states {
                if !flag[i][mask] {
                    continue;
                }
                for k in 0..n {
                    let bit = 1usize << k;
                    if (mask & bit) != 0 {
                        continue;
                    }
                    let l = lens[k];
                    if i + l <= l_s && text[i..i + l] == patterns[k][..] {
                        flag[i + l][mask | bit] = true;
                    }
                }
            }
        }

        // Count all end positions where all patterns have been used.
        let mut cnt: i32 = 0;
        for i in 0..=l_s {
            if flag[i][full] {
                cnt += 1;
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
