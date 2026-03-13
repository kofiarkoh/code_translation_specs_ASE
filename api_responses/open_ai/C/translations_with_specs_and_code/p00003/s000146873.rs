// pythag_triples.rs
//
// Translated from the provided C program according to the SPEC.
// Behavior:
//  - Read an integer N, then N triplets of signed integers (a, b, c).
//  - For each triplet, treat the largest of the three as the hypotenuse
//    and print "YES" if hypotenuse^2 == sum of squares of the other two,
//    otherwise print "NO".
// Notes on translation choices:
//  - Uses i32 to match typical C `int` sizing in the SPEC.
//  - Uses wrapping arithmetic (wrapping_mul / wrapping_add) to avoid
//    panics on overflow and to emulate non-checked integer behavior of C.
//    The SPEC says overflow behavior is unspecified; wrapping avoids Rust
//    debug-build panics and yields deterministic two's-complement-like behavior.
//  - Parsing unwraps results (via `.unwrap()`), mirroring the original C
//    program's lack of scanf return-value checks; invalid or insufficient input
//    will cause a panic (undefined in the original).
//  - No `unsafe` is used.

use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Read N; unwrap to mirror C program's unchecked scanf behavior.
    let n: i32 = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => {
            // If N is missing, behavior in the original is undefined.
            // Here we cause a panic to reflect unchecked input parsing.
            panic!("Missing N");
        }
    };

    if n <= 0 {
        // Per SPEC, if N <= 0, no triplets are processed and nothing is printed.
        return;
    }

    // Process N triplets. If fewer tokens are available, `.unwrap()` will panic,
    // matching the original program's unchecked behavior.
    for _ in 0..(n as usize) {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();
        let c: i32 = it.next().unwrap().parse().unwrap();

        // Use wrapping arithmetic to avoid panics on overflow.
        let aa = a.wrapping_mul(a);
        let bb = b.wrapping_mul(b);
        let cc = c.wrapping_mul(c);

        // Determine the maximum value and perform the corresponding check.
        // Using >= tie-breaks; choosing any maximum when there are ties yields
        // the same check semantics as the SPEC requires (largest treated as hypotenuse).
        let is_pythag: bool;
        if a >= b && a >= c {
            // a is hypotenuse
            is_pythag = aa == bb.wrapping_add(cc);
        } else if b >= a && b >= c {
            // b is hypotenuse
            is_pythag = bb == aa.wrapping_add(cc);
        } else {
            // c is hypotenuse
            is_pythag = cc == aa.wrapping_add(bb);
        }

        if is_pythag {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
