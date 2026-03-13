use std::io::{self, Read};

fn main() {
    // Read all stdin and split into whitespace-separated tokens, parse as i32.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().filter_map(|s| s.parse::<i32>().ok());

    while let Some(a) = tokens.next() {
        // If there's no second token, mimic EOF behavior by exiting.
        let b = match tokens.next() {
            Some(v) => v,
            None => break,
        };

        if a == 0 && b == 0 {
            break;
        }

        let mut judge = false;

        // Iterate from a to b inclusive. The original C code used a for-loop
        // that incremented `a` in-place. To avoid signed-integer overflow
        // and undefined behavior while preserving the intended semantics
        // (iterate all values from a to b inclusive when a <= b), we iterate
        // using a separate variable `cur` and stop after reaching `b`.
        let mut cur = a;
        if cur <= b {
            loop {
                // Leap year logic from the C code:
                // print years divisible by 400, or divisible by 4 but not by 100.
                if cur % 400 == 0 {
                    println!("{}", cur);
                    judge = true;
                } else if cur % 100 == 0 {
                    // skip
                } else if cur % 4 == 0 {
                    println!("{}", cur);
                    judge = true;
                }

                if cur == b {
                    break;
                }
                // Use wrapping_add to avoid panicking on overflow in debug builds.
                // We ensure we never actually reach overflow because we break at cur == b,
                // so this is just defensive.
                cur = cur.wrapping_add(1);
            }
        }

        if !judge {
            println!("NA");
        }
        // Print the extra blank line after each case, as in the C code.
        println!();
    }
}
