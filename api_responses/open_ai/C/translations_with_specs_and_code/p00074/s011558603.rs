// translator.rs
//
// Translated from the provided C program according to the SPEC.
// Notes:
// - Uses i64 for arithmetic to avoid Rust-debug overflow panics while preserving
//   integer division and remainder semantics (truncation toward zero, remainder
//   sign matches dividend). The original C used `int` (signed overflow is UB);
//   this is a deliberate, documented deviation to keep well-defined behavior.
// - Input: whitespace-separated integer triples read from stdin.
// - Processing stops when the exact triple -1 -1 -1 is encountered.
// - For each triple (t,h,s) != (-1,-1,-1):
//     input_seconds = t*3600 + h*60 + s
//     seconds1 = 7200 - input_seconds
//     seconds2 = 7200 - (input_seconds / 3)   (integer division truncating toward zero)
//   Each seconds value is printed as HH:MM:SS with fields zero-padded to at least width 2.
// - If input is malformed or incomplete, the program will stop (behavior is unspecified per SPEC).

use std::io::{self, Read};

fn print_time(time: i64) {
    // Match the C implementation:
    // printf("%02d:", time/3600);
    // time %= 3600;
    // printf("%02d:", time/60);
    // time %= 60;
    // printf("%02d\n", time);
    let hh = time / 3600;
    let mut rem = time % 3600;
    let mm = rem / 60;
    let ss = rem % 60;
    // {:02} ensures at least two digits, zero-padded. If the number requires more digits
    // (or contains a sign), the full representation is printed (matching the SPEC).
    println!("{:02}:{:02}:{:02}", hh, mm, ss);
}

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (behavior unspecified in SPEC).
        return;
    }

    // Parse tokens into integers. On parse error, stop (unspecified by SPEC).
    let mut nums: Vec<i64> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i64>() {
            Ok(n) => nums.push(n),
            Err(_) => return,
        }
    }

    // Process triples. If the input has an incomplete final triple, stop.
    let mut i = 0usize;
    while i + 2 < nums.len() {
        let t = nums[i];
        let h = nums[i + 1];
        let s = nums[i + 2];
        i += 3;

        if t == -1 && h == -1 && s == -1 {
            // sentinel: terminate normally
            return;
        }

        let input_seconds = t.wrapping_mul(3600).wrapping_add(h.wrapping_mul(60)).wrapping_add(s);
        // Using wrapping_* to avoid panic on overflow in extreme cases; this keeps behavior
        // defined in Rust. The SPEC notes signed overflow in C is undefined; this is a
        // reasonable choice to keep the program robust in Rust.
        let seconds1 = 7200i64.wrapping_sub(input_seconds);
        let seconds2 = 7200i64.wrapping_sub(input_seconds / 3);
        print_time(seconds1);
        print_time(seconds2);
    }
}
