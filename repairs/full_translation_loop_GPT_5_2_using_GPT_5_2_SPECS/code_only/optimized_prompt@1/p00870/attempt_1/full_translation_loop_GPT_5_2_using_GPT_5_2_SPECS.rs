use std::io::{self, Read};

fn main() {
    // Read entire stdin, then tokenize like scanf("%d %d") / scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n == 0 && m == 0 {
            break;
        }

        // e[12][21], len[12]
        let mut e: Vec<Vec<u8>> = vec![Vec::new(); n];
        let mut lens: Vec<usize> = vec![0; n];
        for i in 0..n {
            let s = it.next().unwrap_or("");
            let bytes = s.as_bytes().to_vec();
            lens[i] = bytes.len();
            e[i] = bytes;
        }

        // Build concatenated string t from m tokens.
        // In the C code, there is a check:
        //   if(buf[ln-1]<' ') buf[ln-1]='\0';
        // But since scanf("%s") splits on whitespace, buf never contains trailing
        // whitespace/control chars; we keep the same effective behavior.
        let mut t: Vec<u8> = Vec::new();
        for _ in 0..m {
            let s = it.next().unwrap_or("");
            t.extend_from_slice(s.as_bytes());
        }
        let l_s = t.len();

        // flag[5001][4096] as bytes (0/1). We only need indices up to l_s and masks up to (1<<n)-1.
        // C uses char, and does cnt += flag[i][loop-1]; so values are 0/1.
        let loop_mask: usize = 1usize << n;
        let mut flag: Vec<Vec<u8>> = vec![vec![0u8; loop_mask]; l_s + 1];

        // DP transitions
        for i in 0..l_s {
            for j in 0..loop_mask {
                if j != 0 && flag[i][j] == 0 {
                    continue;
                }
                let mut mask = 1usize;
                for k in 0..n {
                    if (j & mask) != 0 {
                        mask <<= 1;
                        continue;
                    }
                    let l = lens[k];
                    if i + l <= l_s && t[i..i + l] == e[k][..] {
                        let pat = j | mask;
                        flag[i + l][pat] = 1;
                    }
                    mask <<= 1;
                }
            }
        }

        // Count ways (actually count of positions i where full mask is reachable; values are 0/1)
        let full = loop_mask - 1;
        let mut cnt: i32 = 0;
        for i in 0..=l_s {
            cnt += flag[i][full] as i32;
        }

        print!("{}\n", cnt);
    }
}
