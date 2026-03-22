use std::io::{self, Read};

type Ll = i64;

fn main() {
    // Read entire stdin (C code uses fgets twice; reading all is equivalent for parsing).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Parse first line as n via atoi semantics (skip leading whitespace, parse optional sign and digits).
    // The original uses fgets(buf,10) then atoi(buf). We'll emulate typical atoi behavior.
    fn atoi_like(bytes: &[u8], idx: &mut usize) -> i32 {
        // read until newline or end, but allow leading whitespace
        while *idx < bytes.len() && (bytes[*idx] == b' ' || bytes[*idx] == b'\t' || bytes[*idx] == b'\r') {
            *idx += 1;
        }
        let mut sign: i32 = 1;
        if *idx < bytes.len() && (bytes[*idx] == b'+' || bytes[*idx] == b'-') {
            if bytes[*idx] == b'-' {
                sign = -1;
            }
            *idx += 1;
        }
        let mut val: i32 = 0;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            val = val.saturating_mul(10).saturating_add((bytes[*idx] - b'0') as i32);
            *idx += 1;
        }
        // consume to end of line (fgets would stop at newline)
        while *idx < bytes.len() && bytes[*idx] != b'\n' {
            *idx += 1;
        }
        if *idx < bytes.len() && bytes[*idx] == b'\n' {
            *idx += 1;
        }
        val * sign
    }

    let n_i32 = atoi_like(bytes, &mut idx);
    let n = if n_i32 < 0 { 0usize } else { n_i32 as usize };

    // Now emulate the second fgets into buf and then parsing with getlong() using a pointer `p`.
    // In C, getlong():
    //   ll n=0; while (*p>='0') n = n*10 + digit; p++; p++; return n;
    // It assumes numbers are non-negative and separated by one non-digit (typically space/newline).
    // We'll implement the same: read digits while byte >= '0' (note: this includes ':' etc in ASCII,
    // but input is expected digits/spaces; we preserve the C condition).
    fn getlong_like(bytes: &[u8], p: &mut usize) -> Ll {
        let mut n: Ll = 0;
        while *p < bytes.len() && bytes[*p] >= b'0' {
            // n = n*10 + (digit)
            n = n.wrapping_mul(10).wrapping_add((bytes[*p] & 0x0f) as Ll);
            *p += 1;
        }
        // p++ unconditionally (in C), but avoid out-of-bounds in Rust.
        if *p < bytes.len() {
            *p += 1;
        }
        n
    }

    // `p` points to the start of the second line read by fgets.
    // We'll set p to current idx (start of remaining input).
    let mut p = idx;

    let mut w = vec![0 as Ll; 4001];
    if n > 0 {
        w[0] = getlong_like(bytes, &mut p);
        for i in 1..n {
            let v = getlong_like(bytes, &mut p);
            w[i] = w[i - 1].wrapping_add(v);
        }
    }

    // dp[4001][4001] initialized with memset 0x33.
    // For ll, that becomes 0x3333333333333333.
    let inf: Ll = 0x3333_3333_3333_3333u64 as i64;

    // Allocate only n x n (C allocates full 4001^2; but semantics for used indices are same).
    let mut dp = vec![vec![inf; n.max(1)]; n.max(1)];
    let mut k = vec![vec![0usize; n.max(1)]; n.max(1)];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    for a in 1..=n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // b from k[i][j-1] to k[i+1][j]
            let b_start = k[i][j - 1];
            let b_end = k[i + 1][j];
            for b in b_start..=b_end {
                let mut c = dp[i][b]
                    .wrapping_add(dp[b + 1][j])
                    .wrapping_add(w[j]);
                if i > 0 {
                    c = c.wrapping_sub(w[i - 1]);
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

    if n == 0 {
        // C would access dp[0][n-1] (UB). Not specified; avoid panic.
        // However, AOJ inputs should have n>=1. We'll print 0 for robustness.
        print!("0\n");
    } else {
        print!("{}\n", dp[0][n - 1]);
    }
}
