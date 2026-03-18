use std::io::{self, Read};

fn main() {
    // The original attempt assumed queries are whitespace-separated as: "<char> <int>".
    // The runtime error indicates the actual input may contain queries like "A123" (no space)
    // or other compact forms where the operation character is attached to the integer.
    //
    // The specification only guarantees: each query consists of a character and an integer.
    // To preserve that with the narrowest interpretation, we parse each query token and
    // extract:
    //   - the first non-whitespace character as the operation,
    //   - the remaining characters (possibly empty) as the integer, otherwise read next token.
    //
    // Query semantics are still unspecified; we keep the minimal state:
    // - mapping[1..=N] (values are indices 1..=N per spec),
    // - modification counts (non-negative),
    // - cumulative result: for each query with index idx, increment mods[idx] and add mapping[idx].

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut mapping: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        let v: usize = it.next().unwrap().parse().unwrap();
        mapping[i] = v as i64;
    }

    let mut mods: Vec<u64> = vec![0; n + 1];
    let mut result: i128 = 0;

    for _ in 0..q {
        let tok = it.next().unwrap();

        // Extract op + maybe-attached integer.
        let mut chars = tok.chars();
        let _op = chars.next().unwrap(); // operation character; behavior unspecified
        let rest: String = chars.collect();

        let idx_str = if rest.is_empty() {
            // Integer is in the next token.
            it.next().unwrap()
        } else {
            // Integer is attached to the op in the same token.
            rest.as_str()
        };

        // Allow leading '+'/'-' as part of integer token; spec says valid input.
        let idx_i64: i64 = idx_str.parse().unwrap();
        let idx: usize = idx_i64 as usize;

        mods[idx] = mods[idx].wrapping_add(1);
        result += mapping[idx] as i128;
    }

    println!("{}", result);
}
