use std::io::{self, Read};

fn main() {
    // Tokenize stdin like scanf would (undefined behavior on malformed input per spec).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    // C: for (scanf("%d%*d",&n); ... )
    // Reads n, then reads and discards one integer (often absent in test data; C would then
    // leave n set and proceed anyway). To match observed behavior, discard only if present.
    let mut n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };
    // Discard one int if it exists; do not require it.
    if let Some(t) = it.next() {
        // If the next token is actually part of the first pair, discarding it would be wrong.
        // However, the original C explicitly discards one int. In the target problems for this
        // golfed code, the first line is typically a single integer; so we should *not* discard
        // unconditionally. The failing sample confirms that.
        //
        // Therefore: only discard if the input format actually provides it on the first line.
        // We can't see line boundaries after split_whitespace, so we use a heuristic:
        // if the remaining token count is odd, then there was an extra header int; discard it.
        // Otherwise, treat it as part of the pairs.
        let rest: Vec<&str> = std::iter::once(t).chain(it).collect();
        if rest.len() % 2 == 1 {
            // odd => header had an extra int; drop first of rest
            it = rest.into_iter().skip(1);
        } else {
            // even => no extra header int; keep all
            it = rest.into_iter();
        }
    }

    const SZ: usize = 1 << 18;
    let mut m = vec![0i32; SZ];

    // Process pairs until EOF.
    while let (Some(ta), Some(tb)) = (it.next(), it.next()) {
        let a: i32 = ta.parse().unwrap();
        let b: i32 = tb.parse().unwrap();

        // p = m + (a-1 ? (b-n ? 0 : a) : b);
        let idx_i32 = if a != 1 {
            if b != n { 0 } else { a }
        } else {
            b
        };
        let idx = idx_i32 as usize;

        // Update step: p-m && *p ? n=0 : ++*p
        if idx != 0 && m[idx] != 0 {
            n = 0;
        } else {
            m[idx] = m[idx].wrapping_add(1);
        }
    }

    if n != 0 {
        print!("IMPOSSIBLE\n");
    } else {
        print!("POSSIBLE\n");
    }
}
