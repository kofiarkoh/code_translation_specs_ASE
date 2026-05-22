use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C uses global arrays of size 62; keep same sizes.
    // ans is used 1..=n, with ans[n+1]=0 and puts(ans+1).
    let mut ans = vec![0u8; 62];
    let mut low = vec![0u8; 62];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let i_tok = match it.next() {
            Some(v) => v,
            None => break,
        };
        let j_tok = match it.next() {
            Some(v) => v,
            None => break,
        };
        if n <= 0 {
            break;
        }

        // Use i64 to match C's long long. Use wrapping ops to emulate ~ and any overflow behavior.
        let mut i_val: i64 = i_tok.parse().unwrap();
        let mut j_val: i64 = j_tok.parse().unwrap();

        // C: i--, j--;
        i_val = i_val.wrapping_sub(1);
        j_val = j_val.wrapping_sub(1);

        let n_usize = n as usize;

        // for (k = 1; k <= n; k++) { low[n-k] = (i >> (n-k)) & 1; if (!low[n-k]) i = ~i; }
        for k in 1..=n_usize {
            let shift = (n_usize - k) as u32;
            // Arithmetic right shift on signed i64 matches typical C behavior; keep i64.
            let bit = ((i_val >> shift) & 1) as u8;
            low[n_usize - k] = bit;
            if bit == 0 {
                i_val = !i_val;
            }
        }

        // for (k = 1; k <= n; k++) { ans[k] = (((j >> (n-k)) & 1) == low[k-1])? 'L': 'R'; if (!low[k-1]) j = ~j; }
        for k in 1..=n_usize {
            let shift = (n_usize - k) as u32;
            let jbit = ((j_val >> shift) & 1) as u8;
            ans[k] = if jbit == low[k - 1] { b'L' } else { b'R' };
            if low[k - 1] == 0 {
                j_val = !j_val;
            }
        }

        // ans[n+1] = 0; puts(ans+1);
        ans[n_usize + 1] = 0;

        // Print bytes from ans[1..=n] followed by '\n' (puts behavior).
        let out = String::from_utf8_lossy(&ans[1..=n_usize]).to_string();
        print!("{out}\n");
    }
}
