// judge.rs
//
// Translated from C to Rust according to the provided specification.
// Notes:
// - The SPEC is authoritative. It requires processing only complete pairs of floats
//   and using strict '<' comparisons in the given order to print exactly one label per pair.
// - The original C used `while(scanf("%f%f",&x,&y)!=EOF)` which can behave oddly if only one
//   float is read (scanf returns the number of items assigned, not EOF). To follow the SPEC,
//   this Rust program processes only complete pairs and stops silently on EOF or on a token
//   that cannot be parsed as a float (no error messages), matching the SPEC's error-handling guidance.
//
// Behavior:
// - Reads whitespace-separated tokens from stdin, interprets them as f32 values in pairs (x,y).
// - For each complete parsed pair, prints exactly one of: "AAA", "AA", "A", "B", "C", "D", "E", or "NA"
//   followed by a newline. Processing stops at EOF or on parse failure.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string. On read error, exit silently (per SPEC preconditions).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    // Iterate over whitespace-separated tokens.
    let mut tokens = input.split_whitespace();

    // Threshold constants as f32 to match C `float` semantics.
    const T1_X: f32 = 35.5;
    const T1_Y: f32 = 71.0;
    const T2_X: f32 = 37.5;
    const T2_Y: f32 = 77.0;
    const T3_X: f32 = 40.0;
    const T3_Y: f32 = 83.0;
    const T4_X: f32 = 43.0;
    const T4_Y: f32 = 89.0;
    const T5_X: f32 = 50.0;
    const T5_Y: f32 = 105.0;
    const T6_X: f32 = 55.0;
    const T6_Y: f32 = 116.0;
    const T7_X: f32 = 70.0;
    const T7_Y: f32 = 148.0;

    loop {
        // Attempt to get the next two tokens. If fewer than two remain, stop.
        let x_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let y_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse as f32. On parse failure, stop processing (no error output).
        let x: f32 = match x_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let y: f32 = match y_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Evaluate conditions in the specified order with strict '<' comparisons.
        if x < T1_X && y < T1_Y {
            println!("AAA");
        } else if x < T2_X && y < T2_Y {
            println!("AA");
        } else if x < T3_X && y < T3_Y {
            println!("A");
        } else if x < T4_X && y < T4_Y {
            println!("B");
        } else if x < T5_X && y < T5_Y {
            println!("C");
        } else if x < T6_X && y < T6_Y {
            println!("D");
        } else if x < T7_X && y < T7_Y {
            println!("E");
        } else {
            println!("NA");
        }
    }
}
