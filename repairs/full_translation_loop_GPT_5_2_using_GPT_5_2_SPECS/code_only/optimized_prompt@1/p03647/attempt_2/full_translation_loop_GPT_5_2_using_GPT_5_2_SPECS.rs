use std::io::{self, Read};

fn main() {
    // Tokenize stdin like scanf("%d").
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    // C: scanf("%d%*d",&n);
    // Read n; ignore the next int if present.
    let mut n: i32 = it.next().and_then(|t| t.parse().ok()).unwrap_or(0);
    let _ignored = it.next();

    // C globals: int m[1<<18]; int *p;
    // Zero-initialized.
    let mut m = vec![0i32; 1 << 18];

    // In the C code, p is always set to m + (...) where (...) is 0, a, or b.
    // For the intended inputs, these are within [0, 2^18).
    // We keep it safe by only indexing when in-bounds; otherwise we treat it as 0.
    let mut p_idx: usize = 0;

    // Loop while we can read pairs (a,b), matching scanf("%d%d") success.
    while let (Some(a_tok), Some(b_tok)) = (it.next(), it.next()) {
        let a: i32 = a_tok.parse().unwrap_or(0);
        let b: i32 = b_tok.parse().unwrap_or(0);

        // Body: p = m + (a-1 ? (b-n ? 0 : a) : b);
        let off: i32 = if a - 1 != 0 {
            if b - n != 0 { 0 } else { a }
        } else {
            b
        };

        // Convert to index if valid; otherwise use 0 (best-effort safe emulation).
        p_idx = if off >= 0 && (off as usize) < m.len() {
            off as usize
        } else {
            0
        };

        // Iteration expression (executed after body each time):
        // p-m && *p ? n=0 : ++*p
        //
        // Important: In C, p-m is a ptrdiff_t; when p==m it's 0, otherwise nonzero.
        // Also, ++*p is pre-increment of the pointed-to int.
        if p_idx != 0 && m[p_idx] != 0 {
            n = 0;
        } else {
            // Signed overflow is UB in C, but for this problem domain it won't happen.
            // Use wrapping_add to avoid Rust overflow panics in debug and be robust.
            m[p_idx] = m[p_idx].wrapping_add(1);
        }
    }

    // puts(n?"IMPOSSIBLE":"POSSIBLE");
    if n != 0 {
        print!("IMPOSSIBLE\n");
    } else {
        print!("POSSIBLE\n");
    }
}
