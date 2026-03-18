use std::io::{self, Read};

fn abs_i32(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn main() {
    // Read entire stdin (fast enough and preserves C-like token parsing behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Emulate the C in(): reads a non-negative integer by consuming digits,
    // without skipping leading whitespace explicitly (C code assumes digits start immediately).
    let mut in_u32 = || -> u32 {
        let mut n: u32 = 0;
        if idx >= bytes.len() {
            return 0;
        }
        let mut c = bytes[idx];
        idx += 1;
        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as u32);
            if idx >= bytes.len() {
                break;
            }
            c = bytes[idx];
            idx += 1;
            if c < b'0' {
                break;
            }
        }
        n
    };

    let n = in_u32() as usize;

    let mut x = vec![0i32; n];
    let mut y = vec![0i32; n];
    for i in 0..n {
        x[i] = in_u32() as i32;
        y[i] = in_u32() as i32;
    }

    // C: memset(dp, 0x10, sizeof(dp)) on int dp[1001][1001]
    // This sets each byte to 0x10, so each i32 becomes 0x10101010.
    let inf: i32 = 0x1010_1010u32 as i32;
    let mut dp = vec![vec![inf; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    for l in 0..n {
        for i in 0..(n - l) {
            for j in i..(i + l) {
                // Use wrapping arithmetic to mirror C int overflow behavior.
                let t = dp[i][j]
                    .wrapping_add(dp[j + 1][i + l])
                    .wrapping_add(abs_i32(x[i].wrapping_sub(x[j + 1])))
                    .wrapping_add(abs_i32(y[j].wrapping_sub(y[i + l])));
                if t < dp[i][i + l] {
                    dp[i][i + l] = t;
                }
            }
        }
    }

    // Matches C output formatting exactly.
    if n > 0 {
        println!("{}", dp[0][n - 1]);
    } else {
        // C would access dp[0][-1] (UB) if n==0; input likely guarantees n>=1.
        // We avoid panic; print nothing would differ, so print 0 as a conservative fallback.
        println!("0");
    }
}
