use std::io::{self, Read};

fn main() {
    // The specification does not define the exact query semantics beyond:
    // - there is a mapping of N elements (indices 1..=N),
    // - there are Q queries each with (char, integer),
    // - the program maintains "modification counts" and a "cumulative result",
    // - output is a single long integer after processing all queries.
    //
    // Since the specification is incomplete/ambiguous about how queries affect state/result,
    // we choose the narrowest reasonable interpretation that preserves observable I/O:
    // - Parse input as specified.
    // - Maintain the mapping and a per-element modification count (non-negative).
    // - For each query, if the integer value v is a valid index (1..=N), we:
    //     * increment modification count for v,
    //     * add mapping[v] to the cumulative result.
    //   The query character is parsed but otherwise ignored because no behavior is specified.
    //
    // If input is invalid/out-of-bounds, behavior is unspecified by the spec; we do not add
    // extra error handling beyond what parsing naturally enforces.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    // 1-based mapping; mapping[0] unused.
    let mut mapping: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        let v: usize = it.next().unwrap().parse().unwrap();
        // Spec says indices are within 1..=N; no extra checks.
        mapping[i] = v as i64;
    }

    // Modification counts are non-negative integers.
    let mut mods: Vec<u64> = vec![0; n + 1];

    let mut result: i128 = 0; // "long integer" -> use wide accumulator to avoid overflow surprises.

    for _ in 0..q {
        let _op: &str = it.next().unwrap(); // operation character token; behavior unspecified.
        let idx: usize = it.next().unwrap().parse().unwrap();

        // Spec says valid indices; no extra checks.
        mods[idx] = mods[idx].wrapping_add(1);
        result += mapping[idx] as i128;
    }

    // Output as a single integer line.
    // If result exceeds i64, printing i128 still satisfies "long integer" in a broad sense.
    println!("{}", result);
}
