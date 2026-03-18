use std::io::{self, Read};

type Ll = i64;

fn main() {
    // Read entire stdin (C uses fgets twice; but behavior depends only on the bytes present).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();

    // Parse first line as n using C-like atoi behavior (skip leading whitespace, optional sign, digits).
    let mut idx = 0usize;
    while idx < bytes.len() && (bytes[idx] == b' ' || bytes[idx] == b'\n' || bytes[idx] == b'\r' || bytes[idx] == b'\t') {
        idx += 1;
    }
    let mut sign: i64 = 1;
    if idx < bytes.len() && (bytes[idx] == b'+' || bytes[idx] == b'-') {
        if bytes[idx] == b'-' {
            sign = -1;
        }
        idx += 1;
    }
    let mut n_i64: i64 = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        n_i64 = n_i64 * 10 + (bytes[idx] - b'0') as i64;
        idx += 1;
    }
    let n: usize = (n_i64 * sign) as usize;

    // Move to start of second line (C does fgets for the second line).
    while idx < bytes.len() && bytes[idx] != b'\n' {
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b'\n' {
        idx += 1;
    }

    // getlong(): reads digits while *p >= '0' (note: this accepts ':' ';' ... too, but input is numeric),
    // then increments p once more to skip a delimiter.
    let mut getlong = |pos: &mut usize| -> Ll {
        let mut val: Ll = 0;
        while *pos < bytes.len() && bytes[*pos] >= b'0' {
            // n = (n<<3) + (n<<1) + (*p & 0xf)  == n*10 + digit
            val = (val << 3) + (val << 1) + ((bytes[*pos] & 0x0f) as Ll);
            *pos += 1;
        }
        // p++ (skip one delimiter if present)
        if *pos < bytes.len() {
            *pos += 1;
        }
        val
    };

    let mut w = vec![0i64; n.max(1)];
    if n > 0 {
        w[0] = getlong(&mut idx);
        for i in 1..n {
            let x = getlong(&mut idx);
            w[i] = w[i - 1] + x;
        }
    }

    // dp initialized with memset 0x33 => each byte 0x33, i64 becomes 0x3333333333333333
    const INF: i64 = 0x3333_3333_3333_3333u64 as i64;
    let mut dp = vec![vec![INF; n.max(1)]; n.max(1)];
    let mut k = vec![vec![0usize; n.max(1)]; n.max(1)];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    for a in 1..=n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // b from k[i][j-1] to k[i+1][j] inclusive
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
        // C would likely misbehave for n==0 (dp[0][-1]); but keep output defined here.
        // This branch should not be hit for valid AOJ inputs.
        print!("0\n");
    }
}
