use std::io::{self, Read};

fn main() {
    // This Rust program intentionally mirrors the semantics of the provided C program.
    // - It treats integers as 32-bit signed ints and uses wrapping arithmetic (two's complement)
    //   to emulate typical C "int" wraparound behavior on overflow.
    // - It reads whitespace-separated tokens from stdin (like scanf("%s", ...)) and scans
    //   each token byte-by-byte for ASCII digit runs, converting each contiguous run of
    //   ASCII digits ('0'..'9') into an i32 (with wrapping) and adding to the running sum.
    //
    // The program prints the final sum as a decimal number followed by a newline, exactly
    // as the original C program does.

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave similarly to C which would likely exit;
        // here we just print 0 (sum initial value) to match that there is nothing to process.
        println!("0");
        return;
    }

    let mut sum: i32 = 0;

    // split_whitespace approximates scanf("%s") tokenization.
    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        let mut i: usize = 0;
        while i < bytes.len() {
            let b = bytes[i];
            if b >= b'0' && b <= b'9' {
                // start of a number
                let mut num: i32 = 0;
                while i < bytes.len() {
                    let bb = bytes[i];
                    if bb < b'0' || bb > b'9' {
                        break;
                    }
                    // emulate 32-bit signed int wraparound as in typical C implementations
                    num = num.wrapping_mul(10).wrapping_add((bb - b'0') as i32);
                    i += 1;
                }
                sum = sum.wrapping_add(num);
            } else {
                i += 1;
            }
        }
    }

    println!("{}", sum);
}
