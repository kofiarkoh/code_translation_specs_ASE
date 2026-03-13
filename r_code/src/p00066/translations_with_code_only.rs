// translation.rs
// Translated from the provided C program.
//
// Notes on semantics preservation:
// - The original C code used a fixed char str[10] buffer initialized to zeros
//   and then repeatedly called scanf("%s", str). That means bytes beyond the
//   written null terminator (if any) keep their previous values. We emulate
//   this by maintaining a fixed [u8; 10] buffer, copying the token bytes into
//   it, writing a null (0) at the first position after the token (or at index 9
//   if the token is too long). We intentionally do NOT overwrite bytes beyond
//   that null so that leftover values persist across iterations, like in C.
// - Input tokens are taken as whitespace-separated strings (scanf("%s") behavior).
// - All character comparisons are done on ASCII bytes to match the C `char` semantics.
// - Output matches the C program exactly: one of "d\n", "o\n", or "x\n".
use std::io::{self, Read};

fn main() {
    // fixed 10-byte buffer, initialized to zeros as in `char str[10] = {0};`
    let mut buf: [u8; 10] = [0; 10];

    // Read entire stdin and split on whitespace like scanf("%s", ...)
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for token in input.split_whitespace() {
        let bytes = token.as_bytes();
        let len = bytes.len();

        if len < 10 {
            // copy token bytes into buffer and place a null terminator at index `len`.
            for (i, &b) in bytes.iter().enumerate() {
                buf[i] = b;
            }
            // null terminator (like scanf writes)
            buf[len] = 0;
            // leave bytes beyond `len` unchanged to emulate C behavior
        } else {
            // token too long to fit entirely in buf: emulate a conservative behavior
            // similar to limiting to 9 chars + null within the buffer.
            // (The original C code would overflow the array in this case; to remain safe
            //  we place a terminator at index 9 and keep bytes beyond index 9 untouched.)
            for i in 0..9 {
                buf[i] = bytes[i];
            }
            buf[9] = 0;
        }

        // Helper to get a byte at an index
        let at = |i: usize| -> u8 { buf[i] };

        // Determine i according to the same branching logic as the C program.
        let i_val: i32 = if at(0) != b's' && at(0) == at(1) && at(1) == at(2) {
            0
        } else if at(0) != b's' && at(0) == at(3) && at(3) == at(6) {
            0
        } else if at(0) != b's' && at(0) == at(4) && at(4) == at(8) {
            0
        } else {
            if at(1) != b's' && at(1) == at(4) && at(4) == at(7) {
                1
            } else {
                if at(2) != b's' && at(2) == at(5) && at(5) == at(8) {
                    2
                } else if at(2) != b's' && at(2) == at(4) && at(4) == at(6) {
                    2
                } else {
                    if at(3) != b's' && at(3) == at(4) && at(4) == at(5) {
                        3
                    } else {
                        if at(6) != b's' && at(6) == at(7) && at(7) == at(8) {
                            6
                        } else {
                            9
                        }
                    }
                }
            }
        };

        if i_val == 9 {
            // draw / default case
            println!("d");
        } else {
            let idx = i_val as usize;
            if at(idx) == b'o' {
                println!("o");
            } else {
                println!("x");
            }
        }
    }
}
