use std::io::{self, Read};

fn main() {
    // Specification is ambiguous about the exact condition/state transition.
    // Narrowest reasonable interpretation: read the first integer N, then read N pairs,
    // but since no evaluation rule is specified, we cannot derive "POSSIBLE"/"IMPOSSIBLE"
    // from the pairs. To avoid inventing functionality, we perform only input consumption
    // and produce no output (observable behavior beyond I/O is unspecified).
    //
    // Error handling is undefined for malformed input; we follow that by using unwrap-like
    // parsing behavior (panic on invalid input).

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => return,
    };

    for _ in 0..n {
        // Consume pairs if present; if missing, parsing will panic, matching undefined behavior.
        let _a: i64 = it.next().unwrap().parse().unwrap();
        let _b: i64 = it.next().unwrap().parse().unwrap();
    }

    // No output because the decision rule is not specified.
}
