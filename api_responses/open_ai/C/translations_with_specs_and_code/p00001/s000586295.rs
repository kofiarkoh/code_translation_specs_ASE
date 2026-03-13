// top_three.rs
//
// Translation of the provided C program (per SPEC).
// Reads 10 integers from stdin and prints the three largest values
// (one per line) in descending order. Initial baselines for the top-3
// are zero.
//
// Notes on semantics:
// - The SPEC treats scanf failures as unchecked / undefined behavior.
//   To preserve that behavior in a safe Rust program, this translation
//   uses unwrap() on iterator/parse operations so that malformed or
//   insufficient input will cause a runtime panic (analogous to an
//   unhandled error in the C version).
// - The original C used scanf("%d\n", ...) where the trailing '\n' can
//   change blocking behavior (scanf may wait for a non-whitespace).
//   Here we read stdin into a string and split on whitespace. This
//   differs slightly in blocking semantics from scanf("%d\n"), but it
//   preserves the SPEC's requirement that the program reads ten
//   whitespace-separated integer tokens. This mismatch is intentional
//   and documented.
//
// Implementation notes:
// - Uses safe, idiomatic Rust. No unsafe code is required.
// - Uses i32 to represent C int (typical mapping). If the C platform's
//   int is wider, adjust types accordingly.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string. This will block until EOF when
    // running interactively unless the user provides EOF; that's acceptable
    // per the SPEC which allows blocking behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    // Baseline top-three values are zero.
    let mut rank: [i32; 3] = [0, 0, 0];

    // Read exactly 10 integers. Per SPEC/C original, we do not check
    // for parse errors or insufficient tokens beyond causing a panic.
    for _ in 0..10 {
        let token = tokens.next().unwrap();
        let work: i32 = token.parse().unwrap();

        if work > rank[0] {
            rank[2] = rank[1];
            rank[1] = rank[0];
            rank[0] = work;
        } else if work > rank[1] {
            rank[2] = rank[1];
            rank[1] = work;
        } else if work > rank[2] {
            rank[2] = work;
        }
    }

    // Print the three values, one per line, largest first.
    println!("{}", rank[0]);
    println!("{}", rank[1]);
    println!("{}", rank[2]);
}
