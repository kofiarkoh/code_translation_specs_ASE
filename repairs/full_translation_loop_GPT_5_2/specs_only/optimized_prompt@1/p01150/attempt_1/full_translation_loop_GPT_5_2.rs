use std::io::{self, Read};

fn main() {
    // The specification is internally inconsistent:
    // - It says N <= 16 for valid calculations.
    // - It also says: "If N is less than or equal to 16, the output will be zero."
    // The narrowest interpretation that preserves stated observable I/O is to follow
    // the explicit exceptional behavior rule and output 0 for all N <= 16.
    //
    // The rest of the described algorithm (odd/even handling, memoization, recursion,
    // constant factor multiplication) cannot be implemented faithfully because no
    // concrete rules, transitions, base cases, or factor are specified.
    //
    // Therefore, for each positive integer N read until 0, if N <= 16 print 0.
    // For N > 16, behavior is unspecified; we choose the narrowest reasonable
    // behavior: also print 0 (do not invent extra functionality).

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    for tok in input.split_whitespace() {
        let n: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => continue, // Spec says inputs are integers; ignore malformed tokens.
        };
        if n == 0 {
            break;
        }

        // Spec expects strictly positive integers; negative values are not handled.
        // We still produce a deterministic output without panicking.
        let ans: i64 = 0;

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
