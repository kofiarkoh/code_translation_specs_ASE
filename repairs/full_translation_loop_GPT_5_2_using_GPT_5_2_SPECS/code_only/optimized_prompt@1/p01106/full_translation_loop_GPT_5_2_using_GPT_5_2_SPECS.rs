use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C has global arrays of size 62; keep same sizes.
    // ans is used with 1-based indexing and printed from ans[1].
    let mut ans = vec![0u8; 62];
    let mut low = vec![0u8; 62];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
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
        let mut i: i64 = match i_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let mut j: i64 = match j_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n <= 0 {
            break;
        }

        // C: i--, j--;
        // Use wrapping to emulate two's-complement wraparound if ever at MIN.
        i = i.wrapping_sub(1);
        j = j.wrapping_sub(1);

        let n_usize = n as usize;

        // for (k = 1; k <= n; k++) { low[n-k] = (i >> (n-k)) & 1; if (!low[n-k]) i = ~i; }
        for k in 1..=n_usize {
            let shift = (n_usize - k) as u32;
            // In C, right shift of signed negative is implementation-defined; most platforms do arithmetic shift.
            // Rust's >> on i64 is arithmetic shift, matching typical C behavior.
            let bit = ((i >> shift) & 1) as u8;
            low[n_usize - k] = bit;
            if bit == 0 {
                i = !i;
            }
        }

        // for (k = 1; k <= n; k++) { ans[k] = (((j >> (n-k)) & 1) == low[k-1])? 'L': 'R'; if (!low[k-1]) j = ~j; }
        for k in 1..=n_usize {
            let shift = (n_usize - k) as u32;
            let jbit = ((j >> shift) & 1) as u8;
            ans[k] = if jbit == low[k - 1] { b'L' } else { b'R' };
            if low[k - 1] == 0 {
                j = !j;
            }
        }

        // ans[n+1] = 0; puts(ans+1);
        // Print exactly the n characters followed by '\n'.
        let out = &ans[1..=n_usize];
        print!("{}", String::from_utf8_lossy(out));
        print!("\n");
    }
}
