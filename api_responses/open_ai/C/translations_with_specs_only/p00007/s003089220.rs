use std::io::{self, Read};

fn main() {
    // Read entire stdin and take the first whitespace-separated token as the integer n.
    // Per specification: if no valid integer is present, behavior is unspecified.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap();
    let n: i32 = token.parse().unwrap();

    // Interpret n as an int; negative n means zero iterations.
    let iterations: i32 = if n > 0 { n } else { 0 };

    // Use i32 for the amount to mirror the Specification's use of `int`.
    // To emulate typical C signed two's-complement wraparound on overflow
    // (the Specification states overflow is not handled and is implementation-defined),
    // we use wrapping arithmetic so behavior is deterministic and resembles common C implementations.
    let mut amount: i32 = 100_000;

    for _ in 0..iterations {
        // Increase by 5% using integer arithmetic similar to `amount * 105 / 100`.
        // Use wrapping_mul to emulate possible signed overflow wraparound.
        let increased = amount.wrapping_mul(105) / 100;

        // Round up to the next multiple of 1000 if not already a multiple.
        // The specification requires "round it up to the next multiple of 1000".
        // For negative values (possible after wraparound), "up" means the next greater-or-equal multiple.
        let rem = increased % 1000;
        let rounded = if rem == 0 {
            increased
        } else if rem > 0 {
            // positive remainder: add (1000 - rem)
            increased.wrapping_add(1000 - rem)
        } else {
            // negative remainder: subtract the negative remainder (i.e., add abs(rem))
            increased.wrapping_sub(rem)
        };

        amount = rounded;
        // Invariant (as intended): after each iteration amount is a multiple of 1000 (modulo wrap semantics).
    }

    // Output the final amount as a decimal integer followed by a newline.
    println!("{}", amount);
}
