// circles_classify.rs
//
// Translates the provided specification into a Rust program.
// Behavior notes:
// - Assumes well-formed input as per the specification.
// - Uses strict `>` comparisons so equality cases follow the non-strict branch behavior described.
// - Parsing uses `unwrap()` to mirror the specification's lack of input validation (parsing failures will panic).
// - If radii are equal, circle A is treated as the larger-or-equal circle (ra >= rb).

use std::io::{self, Read};

fn main() {
    // Read entire stdin and tokenize by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut toks = input.split_whitespace();

    // Parse datasets (assumed to be a non-negative integer in the input per specification).
    // Using usize parsing and unwrap: if input is malformed or negative (like "-1"), unwrap will panic,
    // which is consistent with the specification's assumption that parsing succeeds.
    let datasets: usize = toks.next().unwrap().parse().unwrap();

    for _ in 0..datasets {
        // Read circle A: x, y, r (doubles)
        let ax: f64 = toks.next().unwrap().parse().unwrap();
        let ay: f64 = toks.next().unwrap().parse().unwrap();
        let ra: f64 = toks.next().unwrap().parse().unwrap();

        // Read circle B: x, y, r (doubles)
        let bx: f64 = toks.next().unwrap().parse().unwrap();
        let by: f64 = toks.next().unwrap().parse().unwrap();
        let rb: f64 = toks.next().unwrap().parse().unwrap();

        // Euclidean distance between centers
        let dx = ax - bx;
        let dy = ay - by;
        let d = (dx * dx + dy * dy).sqrt();

        // Classification per specification:
        // if d > ra + rb -> 0
        // else if d > |ra - rb| -> 1
        // else -> 2 if A is larger-or-equal (ra >= rb), else -2
        if d > ra + rb {
            println!("0");
        } else if d > (ra - rb).abs() {
            println!("1");
        } else if ra >= rb {
            println!("2");
        } else {
            println!("-2");
        }
    }

    // Normal termination with exit code 0 (implicit).
}
