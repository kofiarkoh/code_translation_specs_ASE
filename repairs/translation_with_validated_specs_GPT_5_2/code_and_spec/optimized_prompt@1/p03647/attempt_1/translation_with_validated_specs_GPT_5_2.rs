use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse integers until a non-integer is encountered.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Preconditions: at least N and one additional integer exist.
    let mut n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return, // No output possible if even N is missing (outside spec preconditions).
    };
    // Second integer is read and ignored.
    let _ignored = it.next().and_then(|s| s.parse::<i32>().ok());
    // If the ignored integer is missing/malformed, C would stop reading further and still print
    // based on current n. Here, we follow spec: malformed input terminates processing.
    if _ignored.is_none() {
        println!("{}", if n != 0 { "IMPOSSIBLE" } else { "POSSIBLE" });
        return;
    }

    // Fixed-size category usage table: 1<<18 entries.
    // C uses an int array and increments; only "already used" matters.
    let mut used = vec![0u8; 1 << 18];

    // Process pairs (A, B) until EOF or malformed token.
    loop {
        let a = match it.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break, // malformed => stop processing
            },
            None => break, // EOF
        };
        let b = match it.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break, // malformed => stop processing
            },
            None => break, // incomplete record => stop processing
        };

        // Category index derivation exactly as in C:
        // p = m + (a-1 ? (b-n ? 0 : a) : b);
        let idx_i32 = if a - 1 != 0 {
            if b - n != 0 { 0 } else { a }
        } else {
            b
        };

        // Spec: if index out of bounds, behavior is undefined. We therefore do not add
        // defined error handling; we just cast and index (may panic if out of bounds).
        let idx = idx_i32 as usize;

        // C logic:
        // if (p != m && *p) n = 0; else ++*p;
        if idx != 0 && used[idx] != 0 {
            n = 0; // mark infeasible
        } else {
            // Mark used (counting beyond 1 is irrelevant for the decision).
            used[idx] = used[idx].saturating_add(1);
        }
    }

    // C prints: puts(n ? "IMPOSSIBLE" : "POSSIBLE");
    // Note: This is inverted relative to the SPEC's wording. We follow the C behavior
    // as the source-of-truth for the final decision mapping.
    println!("{}", if n != 0 { "IMPOSSIBLE" } else { "POSSIBLE" });
}
