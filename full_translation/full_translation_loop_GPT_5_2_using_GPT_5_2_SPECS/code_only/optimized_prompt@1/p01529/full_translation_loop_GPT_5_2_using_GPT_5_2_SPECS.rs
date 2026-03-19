use std::io::{self, Read};

type Ll = i64;

fn main() {
    // Read all input; C uses fgets twice but overall it just parses integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Parse a (possibly signed) integer in a normal way for the first line (atoi).
    fn parse_i32(bytes: &[u8], idx: &mut usize) -> i32 {
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
        let mut sign: i32 = 1;
        if *idx < bytes.len() && bytes[*idx] == b'-' {
            sign = -1;
            *idx += 1;
        }
        let mut n: i32 = 0;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            n = n.saturating_mul(10).saturating_add((bytes[*idx] - b'0') as i32);
            *idx += 1;
        }
        sign * n
    }

    // Emulate the C getlong() behavior closely:
    // while (*p >= '0') n = n*10 + (*p & 0xf); p++; then p++ once more.
    // This assumes non-negative numbers and a single delimiter after each number.
    fn getlong(bytes: &[u8], idx: &mut usize) -> Ll {
        let mut n: Ll = 0;
        while *idx < bytes.len() && bytes[*idx] >= b'0' {
            n = (n << 3) + (n << 1) + ((bytes[*idx] & 0x0f) as Ll);
            *idx += 1;
        }
        if *idx < bytes.len() {
            *idx += 1; // skip one delimiter
        }
        n
    }

    let n_i32 = parse_i32(bytes, &mut idx);
    // Move to next line like fgets would (consume until '\n', then consume it).
    while idx < bytes.len() && bytes[idx] != b'\n' {
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b'\n' {
        idx += 1;
    }

    let n = if n_i32 < 0 { 0usize } else { n_i32 as usize };
    if n == 0 {
        // C would have UB; keep program from panicking.
        print!("0\n");
        return;
    }

    let mut w = vec![0i64; n];
    w[0] = getlong(bytes, &mut idx);
    for i in 1..n {
        w[i] = w[i - 1] + getlong(bytes, &mut idx);
    }

    // memset(dp, 0x33, sizeof(dp)) for ll => 0x3333333333333333
    let inf: Ll = 0x3333_3333_3333_3333u64 as i64;

    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // The C loop is: for (a=1; a<=n; a++) for (i=0,j=a; j<n; i++,j++)
    // When a==n, inner loop doesn't run. So in Rust: a in 1..n
    for a in 1..n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // In C, b ranges from k[i][j-1] to k[i+1][j].
            // This requires i+1 <= j, which holds because j = i + a and a>=1.
            let b_start = k[i][j - 1];
            let b_end = k[i + 1][j];

            for b in b_start..=b_end {
                // b is a split point; dp[i][b] and dp[b+1][j] require b < j.
                // Knuth optimization guarantees this in valid instances; still, guard to avoid panic.
                if b + 1 > j {
                    continue;
                }
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

    print!("{}\n", dp[0][n - 1]);
}
