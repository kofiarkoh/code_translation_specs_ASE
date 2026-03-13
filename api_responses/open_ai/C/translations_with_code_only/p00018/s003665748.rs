// Translated from the given C program.
// Behavior:
// - Read up to 5 whitespace-separated integers from stdin.
// - Sort them in descending order using the same nested-loop selection-like sort.
// - Print each number followed by a single space (no trailing newline).
//
// Note about corner cases:
// The original C code used scanf in a loop without checking its return value,
// which could leave array elements uninitialized if fewer than 5 integers were
// provided. Rust does not allow uninitialized memory safely, so this translation
// initializes missing inputs to 0. For well-formed inputs of 5 integers this
// behaves the same as the C program.

use std::io::{self, Read};

fn main() {
    // Use i32 to represent C's typical 'int' (32-bit on common platforms).
    let mut a = [0i32; 5];

    // Read entire stdin and split on whitespace to get tokens similar to scanf("%d", ...).
    let mut input = String::new();
    // Ignore errors from read_to_string, similar to how C code ignored scanf return values.
    let _ = io::stdin().read_to_string(&mut input);

    let mut toks = input.split_whitespace();

    for i in 0..5 {
        if let Some(tok) = toks.next() {
            // Try to parse; if parse fails, fall back to 0 (safer than UB).
            if let Ok(v) = tok.parse::<i32>() {
                a[i] = v;
            } else {
                a[i] = 0;
            }
        } else {
            // If there are fewer than 5 tokens, leave remaining as 0.
            a[i] = 0;
        }
    }

    // The C code sorts in descending order with:
    // for(i=0;i<5;i++)
    //   for(j=i;j<5;j++)
    //     if(a[i]<a[j]) swap
    for i in 0..5 {
        for j in i..5 {
            if a[i] < a[j] {
                a.swap(i, j);
            }
        }
    }

    // Print each element followed by a space, no newline (matches printf("%d ")).
    for i in 0..5 {
        print!("{} ", a[i]);
    }
}
