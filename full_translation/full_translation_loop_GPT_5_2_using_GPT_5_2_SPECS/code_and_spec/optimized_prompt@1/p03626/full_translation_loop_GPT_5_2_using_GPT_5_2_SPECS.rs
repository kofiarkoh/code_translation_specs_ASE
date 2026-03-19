use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d %s %s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s1 = it.next().unwrap();
    let s2 = it.next().unwrap();

    // Work with bytes; inputs are assumed to have length at least N per spec.
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    // Previous unit type: true = vertical, false = horizontal.
    // Initialize based on first unit.
    let mut ans: i64;
    let mut prev_vertical: bool;

    if n == 0 {
        // Spec doesn't define N=0 explicitly; C would read strings and then access [0] (UB).
        // We keep behavior unspecified; choose a benign output.
        println!("0");
        return;
    }

    if b1[0] == b2[0] {
        ans = 3;
        prev_vertical = true;
    } else {
        ans = 6;
        prev_vertical = false;
    }

    // Process units left-to-right.
    // Note: The original C code checks both conditions independently (not else-if),
    // relying on the precondition that the encoding is a valid tiling decomposition.
    // We preserve that structure.
    for i in 1..n {
        if b1[i] == b2[i] {
            // current is vertical
            if prev_vertical {
                ans = (ans * 2) % MOD; // vertical after vertical
            } // vertical after horizontal: factor 1 (no change)
            prev_vertical = true;
        }

        // Horizontal unit starts at i if s1[i] == s1[i+1].
        // In a valid tiling, this will only be evaluated meaningfully when i+1 < N.
        // The C code reads S1[i+1] even when i == N-1 (out of bounds), which is UB.
        // Per spec, inputs encode a valid tiling; thus horizontal cannot start at N-1.
        if i + 1 < n && b1[i] == b1[i + 1] {
            // current is horizontal
            if prev_vertical {
                ans = (ans * 2) % MOD; // horizontal after vertical
            } else {
                ans = (ans * 3) % MOD; // horizontal after horizontal
            }
            prev_vertical = false;
        }
    }

    println!("{ans}");
}
