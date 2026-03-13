// Translated from given C code, preserving behavior as closely as practical in safe Rust.
// Notes:
// - Arrays sizes mirror the C version: wg and coin sized 10000, dp sized 1000x1000.
// - The parser treats non-digit characters (including commas) as separators, matching scanf("%d,%d") usage.
// - The algorithm and loops replicate the C logic exactly (including 1-based indexing for items).
// - This translation uses safe Rust; indexing will panic on out-of-bounds whereas C would have undefined behavior.
//   The original C program also relies on inputs that keep indices within the allocated ranges.

use std::io::{self, Read};

struct Tokenizer {
    bytes: Vec<u8>,
    pos: usize,
    len: usize,
}

impl Tokenizer {
    fn new(s: String) -> Self {
        let bytes = s.into_bytes();
        let len = bytes.len();
        Tokenizer { bytes, pos: 0, len }
    }

    // Parse next i32 from input, treating any non-digit and non-sign as separators.
    fn next_i32(&mut self) -> Option<i32> {
        // Skip until start of number (digit or '-' or '+')
        while self.pos < self.len {
            let b = self.bytes[self.pos];
            if (b'0'..=b'9').contains(&b) || b == b'-' || b == b'+' {
                break;
            }
            self.pos += 1;
        }
        if self.pos >= self.len {
            return None;
        }

        // capture sign if present
        let mut neg = false;
        if self.bytes[self.pos] == b'-' {
            neg = true;
            self.pos += 1;
        } else if self.bytes[self.pos] == b'+' {
            self.pos += 1;
        }

        // accumulate digits
        let mut val: i64 = 0;
        let mut any = false;
        while self.pos < self.len {
            let b = self.bytes[self.pos];
            if (b'0'..=b'9').contains(&b) {
                any = true;
                val = val * 10 + (b - b'0') as i64;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !any {
            // No digits after sign, treat as None
            return None;
        }
        if neg {
            val = -val;
        }
        // Fit into i32 like C's int (assuming inputs fit)
        Some(val as i32)
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = Tokenizer::new(input);

    // Mirror C arrays
    let mut wg = vec![0i32; 10000];
    let mut coin = vec![0i32; 10000];
    // dp dimensions 1000 x 1000, initialize to -1 as a reasonable safe equivalent.
    let mut dp = vec![vec![-1i32; 1000]; 1000];

    let mut count = 0i32;
    loop {
        // Read W
        let w_opt = toks.next_i32();
        if w_opt.is_none() {
            break;
        }
        let W = w_opt.unwrap();
        if W == 0 {
            break;
        }
        // Read N
        let N_opt = toks.next_i32();
        if N_opt.is_none() {
            break;
        }
        let N = N_opt.unwrap();

        // Initialize dp[i][j] = -1 for i=0..=N, j=0..=W (as in C)
        // Note: bounds are assumed to be within 0..999 for dp dimensions.
        for i in 0..=N as usize {
            for j in 0..=W as usize {
                dp[i][j] = -1;
            }
        }

        // Read N pairs formatted like "%d,%d"
        for i in 1..=N as usize {
            // We rely on tokenizer to treat comma as separator
            if let Some(c) = toks.next_i32() {
                coin[i] = c;
            } else {
                coin[i] = 0;
            }
            if let Some(g) = toks.next_i32() {
                wg[i] = g;
            } else {
                wg[i] = 0;
            }
        }

        // Replicate algorithm exactly
        dp[0][0] = 0;

        // local mutable copies for tracking
        let mut max_coin: i32 = 0;
        let mut weg: i32 = 0;

        for i in 1..=N as usize {
            // dp[i][0] = 0
            dp[i][0] = 0;

            // if dp[i][wg[i]] < coin[i] then set
            let wi = wg[i] as usize;
            // Note: rely on inputs to keep wi < 1000
            if dp[i][wi] < coin[i] {
                dp[i][wi] = coin[i];
            }
            // for j=i..=N dp[j][wg[i]] = dp[i][wg[i]]
            for j in i..=N as usize {
                dp[j][wi] = dp[i][wi];
            }

            for j in 1..=W as usize {
                // check dp[i-1][j] > 0
                if dp[i - 1][j] > 0 {
                    let targ = j + wi;
                    // assume targ < 1000 as in original program expectations
                    if dp[i][targ] < (coin[i] + dp[i - 1][j]) {
                        dp[i][targ] = coin[i] + dp[i - 1][j];
                        for k in i..=N as usize {
                            dp[k][targ] = dp[i][targ];
                        }
                    }
                }
            }
        }

        for i in 0..=N as usize {
            for j in 0..=W as usize {
                let val = dp[i][j];
                if max_coin < val {
                    max_coin = val;
                    weg = j as i32;
                } else if max_coin == val {
                    if weg >= j as i32 {
                        weg = j as i32;
                    }
                }
            }
        }

        count += 1;
        // Print exactly like the C program: "Case %d:\n%d\n%d\n"
        println!("Case {}:\n{}\n{}\n", count, max_coin, weg);

        // Reset for next case as C does
        // (coin and wg arrays are left; dp entries for relevant ranges will be reinitialized next loop)
        // Reset max_coin and weg (not strictly necessary due to local reassign, but keep same structure)
        // In this translation max_coin and weg are local per-case, so nothing to reset globally.
    }
}
