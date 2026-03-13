// translation.rs
// Translated from the provided C program and SPEC.
// Behavior:
//  - Read test cases from stdin.
//  - Each test case begins with an integer n. If n == 0, terminate (do not produce output for that terminating 0).
//  - If n > 0, read n i64 processing times, sort them in non-decreasing order, compute the total waiting time
//    (sum of waiting times for tasks scheduled in ascending processing time), and print that sum as i64 on its own line.
// Notes on translation choices:
//  - Uses safe, idiomatic Rust with Vec<i64> and the standard sort (ascending).
//    The SPEC requires sorting in non-decreasing order; the original C used selection sort (O(n^2)) but that is a
//    non-functional detail and not required to preserve.
//  - To avoid panics from signed integer overflow in debug builds and to mirror the C program's lack of overflow checks,
//    arithmetic uses wrapping_add so results wrap on overflow instead of panicking. (C signed overflow is undefined;
//    using wrapping_add is a practical, well-defined choice that avoids aborts in Rust.)
//  - If input ends or a token fails to parse, the program stops processing further test cases (this mirrors scanf-based
//    behavior which also stops on EOF or parse failure).
//  - Negative n: the SPEC assumes n >= 0. The original C would treat any non-zero n (including negative) as a test case
//    header; loops that depend on n would be skipped for negative n resulting in printing 0. This Rust program mirrors
//    that behaviour by treating negative n as a test case with n == 0 (i.e., prints 0) without consuming time values.
//  - The SPEC assumes n does not exceed 10001. This program does not explicitly enforce that limit; it will attempt to
//    read the specified number of times. If tokens are missing, the program will terminate early.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split on whitespace, which mimics scanf("%d"/"%lld") behavior.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (similar to unspecified behavior in the C program).
        return;
    }

    let mut tokens = input.split_whitespace();

    loop {
        // Read header n
        let n_token = match tokens.next() {
            Some(t) => t,
            None => break, // EOF / no more tokens: stop processing
        };

        // Parse n as i32 (C int). If parse fails, stop processing further (mirrors scanf not validated).
        let n_parsed: i32 = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: n == 0 -> stop without producing output for the terminating 0.
        if n_parsed == 0 {
            break;
        }

        // For negative n, mirror the C behavior where loops dependent on n are skipped and sum is 0.
        let n_usize: usize = if n_parsed < 0 { 0 } else { n_parsed as usize };

        // Read n_usize times. If tokens run out or parsing fails, stop processing.
        let mut times: Vec<i64> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let tkn = match tokens.next() {
                Some(t) => t,
                None => return, // premature EOF: exit (unspecified in original)
            };
            let val: i64 = match tkn.parse() {
                Ok(v) => v,
                Err(_) => return, // parse error: exit (unspecified in original)
            };
            times.push(val);
        }

        // Sort in non-decreasing order (ascending).
        times.sort();

        // Compute total waiting time.
        // Use wrapping_add to avoid panics on overflow and reflect the original program's lack of overflow checks.
        let mut prefix_sum: i64 = 0;
        let mut total: i64 = 0;
        for &t in &times {
            // For each task, add the waiting time (sum of previous processing times) to total,
            // then include this task's time in the prefix for subsequent tasks.
            total = total.wrapping_add(prefix_sum);
            prefix_sum = prefix_sum.wrapping_add(t);
        }

        // Print the result as a signed 64-bit integer followed by a newline.
        println!("{}", total);
    }
}
