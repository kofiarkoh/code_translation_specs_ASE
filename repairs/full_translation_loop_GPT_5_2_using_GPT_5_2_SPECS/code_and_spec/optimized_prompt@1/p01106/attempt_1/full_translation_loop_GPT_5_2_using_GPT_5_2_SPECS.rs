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
            break; // incomplete triple => stop (per spec)
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let i_tok = match it.next() {
            Some(v) => v,
            None => break,
        };
        let j_tok = match it.next() {
            Some(v) => v,
            None => break,
        };

        if n <= 0 {
            break; // terminating triple not processed
        }

        // Use i64 to match C's long long and preserve bitwise ~ semantics.
        let mut i: i64 = match i_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let mut j: i64 = match j_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        i -= 1;
        j -= 1;

        let n_usize = n as usize;
        let mut low = vec![0i64; n_usize];

        // First phase: compute low[] from i, mutating i with bitwise NOT when low bit is 0.
        for k in 1..=n_usize {
            let idx = n_usize - k; // corresponds to (n-k) in C
            low[idx] = (i >> idx) & 1;
            if low[idx] == 0 {
                i = !i;
            }
        }

        // Second phase: compute answer from j and low[], mutating j similarly.
        let mut ans = String::with_capacity(n_usize);
        for k in 1..=n_usize {
            let bit = (j >> (n_usize - k)) & 1;
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
