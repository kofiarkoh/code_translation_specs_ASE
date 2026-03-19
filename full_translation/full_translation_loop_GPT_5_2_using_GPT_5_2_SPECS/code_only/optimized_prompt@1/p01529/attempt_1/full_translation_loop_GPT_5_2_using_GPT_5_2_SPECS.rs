use std::io::{self, Read};

type Ll = i64;

fn main() {
    // Read entire stdin (C uses fgets twice; this is equivalent for parsing).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Emulate the C getlong() behavior:
    // - It assumes the current char is a digit.
    // - It accumulates while *p >= '0' (note: no upper bound '9' check).
    // - Then it does p++ once more to skip a delimiter.
    // This matches typical space/newline-separated non-negative integers.
    fn getlong(bytes: &[u8], idx: &mut usize) -> Ll {
        let mut n: Ll = 0;
        while *idx < bytes.len() && bytes[*idx] >= b'0' {
            // n = n*10 + (ch & 0xf)
            n = (n << 3) + (n << 1) + ((bytes[*idx] & 0x0f) as Ll);
            *idx += 1;
        }
        // Skip one delimiter character (space/newline), as in C's p++.
        if *idx < bytes.len() {
            *idx += 1;
        }
        n
    }

    // Parse first line as n (C: fgets(buf,10), n=atoi(buf))
    // We'll parse the first integer in the input similarly.
    while idx < bytes.len() && (bytes[idx] == b' ' || bytes[idx] == b'\n' || bytes[idx] == b'\r' || bytes[idx] == b'\t') {
        idx += 1;
    }
    let mut n_i32: i32 = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        n_i32 = n_i32 * 10 + (bytes[idx] - b'0') as i32;
        idx += 1;
    }
    // Move to the start of the next line (C reads the next line with fgets).
    while idx < bytes.len() && bytes[idx] != b'\n' {
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b'\n' {
        idx += 1;
    }

    let n = n_i32 as usize;

    // Arrays sized as in C (4001). We'll allocate exactly n where possible.
    let mut w = vec![0i64; n.max(1)];
    if n > 0 {
        w[0] = getlong(bytes, &mut idx);
        for i in 1..n {
            let v = getlong(bytes, &mut idx);
            w[i] = w[i - 1] + v;
        }
    }

    // dp initialized with memset 0x33 => each byte 0x33.
    // For i64, that becomes 0x3333333333333333.
    let inf: Ll = 0x3333_3333_3333_3333u64 as i64;

    // dp and k are [n][n]
    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Knuth optimization DP as in C.
    for a in 1..=n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // b from k[i][j-1] to k[i+1][j]
            // Note: when i+1 == n, j cannot be < n with a>=1, so safe.
            let b_start = k[i][j - 1];
            let b_end = k[i + 1][j];
            for b in b_start..=b_end {
                let mut c = dp[i][b] + dp[b + 1][j] + w[j];
                if i > 0 {
                    c -= w[i - 1];
                }
                if dp[i][j] > c {
                    dp[i][j] = c;
                    k[i][j] = b;
                }
            }
            i += 1;
            j += 1;
        }
    }

    if n > 0 {
        print!("{}\n", dp[0][n - 1]);
    } else {
        // C would likely crash/UB for n==0 (dp[0][-1]).
        // We avoid panic; but such input is presumably not provided.
        print!("0\n");
    }
}
