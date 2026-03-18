use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let i_in: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let j_in: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        if n <= 0 {
            break; // terminate without producing output
        }
        let n_usize = n as usize;

        // C code does i--, j-- first.
        let mut i = i_in.wrapping_sub(1);
        let mut j = j_in.wrapping_sub(1);

        // low[0..n] holds bits (0/1) as in the C code.
        let mut low: Vec<u8> = vec![0; n_usize];

        // for (k = 1; k <= n; k++) { low[n-k] = (i >> (n-k)) & 1; if (!low[n-k]) i = ~i; }
        for k in 1..=n_usize {
            let idx = n_usize - k;
            let bit = ((i >> idx) & 1) as u8;
            low[idx] = bit;
            if bit == 0 {
                i = !i; // bitwise NOT, wrapping as in C
            }
        }

        // Build answer string of length n.
        // for (k = 1; k <= n; k++) { ans[k] = (((j >> (n-k)) & 1) == low[k-1])? 'L': 'R'; if (!low[k-1]) j = ~j; }
        let mut ans = String::with_capacity(n_usize);
        for k in 1..=n_usize {
            let bit = ((j >> (n_usize - k)) & 1) as u8;
            let c = if bit == low[k - 1] { 'L' } else { 'R' };
            ans.push(c);
            if low[k - 1] == 0 {
                j = !j;
            }
        }

        out.push_str(&ans);
        out.push('\n');
    }

    print!("{out}");
}
