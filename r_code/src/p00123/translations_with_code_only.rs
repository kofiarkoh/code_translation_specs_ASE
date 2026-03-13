// Translated from the provided C program.
// Behavior notes:
// - Reads whitespace-separated floats from stdin.
// - Emulates the C loop `while (scanf("%f%f",&x,&y) != EOF)`:
//     * If no token is available at the start of an iteration -> stop (scanf would return EOF).
//     * If exactly one token is available -> update x, leave y as previous value, still process (scanf would return 1).
//     * If two tokens are available -> update x and y and process.
// - For simplicity and deterministic behavior, x and y are initialized to 0.0f32. This mirrors the C program for well-formed inputs
//   (the original C has undefined behavior if input has an odd number of values and uses an uninitialized `y` on the first iteration).
// - Comparisons use f32 to match the original C `float` semantics.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split into whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut i = 0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    while i < tokens.len() {
        // Attempt to parse the first float for this iteration.
        // If parsing fails, we break to avoid infinite loops; this is a pragmatic choice
        // since faithfully reproducing scanf's behavior on malformed input is complex
        // and the original program likely expects numeric input.
        match tokens[i].parse::<f32>() {
            Ok(v) => x = v,
            Err(_) => break,
        }
        i += 1;

        // Try to parse the second float. If none available, emulate scanf returning 1:
        // update x only and leave y as the previous value.
        if i < tokens.len() {
            match tokens[i].parse::<f32>() {
                Ok(v) => {
                    y = v;
                    i += 1;
                }
                Err(_) => break,
            }
        } else {
            // no second token: proceed with current x and existing y
        }

        // Perform the same comparisons as the C code (all f32 comparisons).
        if x < 35.5_f32 && y < 71.0_f32 {
            println!("AAA");
        } else if x < 37.5_f32 && y < 77.0_f32 {
            println!("AA");
        } else if x < 40.0_f32 && y < 83.0_f32 {
            println!("A");
        } else if x < 43.0_f32 && y < 89.0_f32 {
            println!("B");
        } else if x < 50.0_f32 && y < 105.0_f32 {
            println!("C");
        } else if x < 55.0_f32 && y < 116.0_f32 {
            println!("D");
        } else if x < 70.0_f32 && y < 148.0_f32 {
            println!("E");
        } else {
            println!("NA");
        }
    }
}
