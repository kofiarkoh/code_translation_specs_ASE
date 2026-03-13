use std::io::{self, Read};

fn main() {
    // Read all input from stdin. The original C program used scanf without
    // checking return values; here we use unwrap()/expect() which will panic
    // on malformed or missing input, mirroring the unspecified behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut toks = input.split_whitespace();

    // Parse the number of test cases. If parsing fails, this will panic (like
    // the original C program would exhibit undefined behavior).
    let n: i32 = toks.next().unwrap().parse().unwrap();

    // If n <= 0, do no processing (same behavior as the C for-loop).
    if n <= 0 {
        return;
    }

    for _ in 0..(n as usize) {
        // Read eight doubles for the two segments. Unwrap/parse unwrap to
        // preserve the unspecified/malformatted-input behavior of the C program.
        let x1: f64 = toks.next().unwrap().parse().unwrap();
        let y1: f64 = toks.next().unwrap().parse().unwrap();
        let x2: f64 = toks.next().unwrap().parse().unwrap();
        let y2: f64 = toks.next().unwrap().parse().unwrap();
        let x3: f64 = toks.next().unwrap().parse().unwrap();
        let y3: f64 = toks.next().unwrap().parse().unwrap();
        let x4: f64 = toks.next().unwrap().parse().unwrap();
        let y4: f64 = toks.next().unwrap().parse().unwrap();

        // Compute slopes as double division. This preserves IEEE-754 semantics:
        // division by zero yields +/-Inf or NaN as appropriate, and comparisons
        // use exact floating-point equality (NaN != NaN).
        let sl1 = (y2 - y1) / (x2 - x1);
        let sl2 = (y4 - y3) / (x4 - x3);

        if sl1 == sl2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
