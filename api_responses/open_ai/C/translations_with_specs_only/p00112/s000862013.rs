// translator.rs
// Translation of the given specification into Rust.
//
// Notes on semantic choices made to stay faithful to the specification:
// - Fixed-capacity arrays of 10001 elements for times and wtime are used.
// - Sorting is implemented as selection sort (O(n^2)) to match the original behavior.
// - Arithmetic that accumulates waiting times uses wrapping arithmetic (i64::wrapping_add and wrapping_mul)
//   to avoid panics on overflow and to emulate the low-level behavior where overflow was unchecked.
// - If a test header n == 0 is encountered the program stops without printing for that terminating 0.
// - If n < 0 (negative) the program does not read any times (the spec indicates only n > 0 leads to reads)
//   and prints 0 for that test case, which mirrors the described/observed behavior.
// - If n > CAPACITY (10001) we intentionally panic to emulate the undefined/crash-like behavior
//   described in the specification (out-of-bounds in the original C code).
//
// The program reads all whitespace-separated tokens from stdin, then consumes them in order,
// following the scanf-like semantics described (no return-value checks). If the input is
// incomplete or malformed the program will stop processing further test cases.

use std::io::{self, Read};

const CAPACITY: usize = 10001;

fn main() {
    // Read entire stdin into a string of whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Spec says input reading is assumed to succeed; if it doesn't, we simply exit.
        return;
    }

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut pos: usize = 0;

    // Fixed-size arrays as per specification.
    let mut times: [i64; CAPACITY] = [0i64; CAPACITY];
    let mut wtime: [i64; CAPACITY] = [0i64; CAPACITY];

    while pos < tokens.len() {
        // Parse n as 32-bit signed integer (scanf("%d",&n) semantics).
        let n_i32: i32 = match tokens[pos].parse() {
            Ok(v) => v,
            Err(_) => {
                // Spec: malformed input leads to unspecified behavior; here we stop processing.
                break;
            }
        };
        pos += 1;

        if n_i32 == 0 {
            // Terminating 0: stop processing and do not produce output for this header.
            break;
        }

        if n_i32 < 0 {
            // Negative n: per specification, do not read times, loops will effectively be skipped,
            // resulting in an output of 0. Emulate that behavior.
            println!("0");
            continue;
        }

        // Now n_i32 > 0
        let n_usize = n_i32 as usize;

        // Emulate capacity invariant: original program assumes n <= 10001.
        if n_usize > CAPACITY {
            // The C program would index out of bounds (undefined behavior).
            // We emulate this by aborting (panic), which is a reasonable stand-in for a crash.
            panic!("n ({}) exceeds internal capacity ({}), emulating undefined behavior", n_usize, CAPACITY);
        }

        // Read exactly n_usize times from tokens (spec assumes they are present).
        if pos + n_usize > tokens.len() {
            // Incomplete input; spec says behavior unspecified. We stop processing.
            break;
        }

        for i in 0..n_usize {
            // Each processing time is a signed 64-bit integer (scanf("%lld")).
            match tokens[pos].parse::<i64>() {
                Ok(v) => times[i] = v,
                Err(_) => {
                    // Malformed time; spec doesn't check scanf returns; we stop processing.
                    return;
                }
            }
            pos += 1;
        }

        // Selection sort (ascending) to match O(n^2) behavior described.
        for i in 0..n_usize {
            let mut min_idx = i;
            let mut j = i + 1;
            while j < n_usize {
                if times[j] < times[min_idx] {
                    min_idx = j;
                }
                j += 1;
            }
            if min_idx != i {
                times.swap(i, min_idx);
            }
        }

        // Compute waiting times wtime as cumulative sums, with wtime[0] = 0 invariant.
        if n_usize == 0 {
            // No tasks: output nothing? But n>0 here, so this branch won't occur.
            println!("0");
            continue;
        }

        wtime[0] = 0i64;
        // Use wrapping_add to emulate unchecked overflow behavior.
        let mut j = 1usize;
        while j < n_usize {
            // wtime[j] = wtime[j-1] + times[j-1]
            wtime[j] = wtime[j - 1].wrapping_add(times[j - 1]);
            j += 1;
        }

        // Sum up the waiting times for tasks j = 1..n-1
        let mut total: i64 = 0i64;
        let mut k = 1usize;
        while k < n_usize {
            total = total.wrapping_add(wtime[k]);
            k += 1;
        }

        // Print the total as a signed 64-bit integer on its own line.
        println!("{}", total);
    }
}
