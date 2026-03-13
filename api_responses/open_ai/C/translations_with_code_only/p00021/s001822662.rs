// Translated from the given C program.
// Preserves floating-point semantics (including Inf/NaN and == behavior) by using f64.
// I/O formatting matches the original: prints "YES" or "NO" each on its own line.
use std::io::{self, Read};

fn main() {
    // Read entire stdin and split on whitespace like scanf does.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut iter = input.split_whitespace();

    // Read n as a 32-bit signed integer to mimic C's `int`.
    let n: i32 = match iter.next() {
        Some(tok) => tok.parse().expect("failed to parse n"),
        None => return, // no input -> nothing to do
    };

    // In C, if n <= 0 the for-loop runs zero times. Mimic that behavior.
    let count = if n > 0 { n as usize } else { 0usize };

    for _ in 0..count {
        // Read eight doubles: x1,y1,x2,y2,x3,y3,x4,y4
        let x1: f64 = iter.next().expect("missing value").parse().expect("parse fail");
        let y1: f64 = iter.next().expect("missing value").parse().expect("parse fail");
        let x2: f64 = iter.next().expect("missing value").parse().expect("parse fail");
        let y2: f64 = iter.next().expect("missing value").parse().expect("parse fail");
        let _x3: f64 = iter.next().expect("missing value").parse().expect("parse fail"); // unused in C
        let _y3: f64 = iter.next().expect("missing value").parse().expect("parse fail"); // unused in C
        let x4: f64 = iter.next().expect("missing value").parse().expect("parse fail");
        let y4: f64 = iter.next().expect("missing value").parse().expect("parse fail");

        // Compute slopes exactly as in C: (y2-y1)/(x2-x1) and (y4-y3)/(x4-x3)
        // Note: we read x3,y3 into _x3/_y3 to preserve input consumption even though original C didn't use them.
        let sl1 = (y2 - y1) / (x2 - x1);
        // For sl2 we need y3 and x3 values; original C used x3,y3 but they were read earlier.
        // We must use the same variables. We loaded them into _x3/_y3 above.
        let sl2 = (y4 - _y3) / (x4 - _x3);

        // Compare using f64 == to preserve IEEE-754 behavior (NaN comparisons are false, +0.0 == -0.0 is true).
        if sl1 == sl2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
