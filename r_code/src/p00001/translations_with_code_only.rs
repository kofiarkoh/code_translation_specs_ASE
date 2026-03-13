use std::io::{self, Read};

fn main() {
    // This program mirrors the given C program:
    // - reads 10 integers from stdin (whitespace separated)
    // - keeps the top-3 values in rank[0..2], initialized to 0
    // - prints the three ranks each on its own line
    //
    // Note about "%d\n" in the original C code:
    // the trailing '\n' in scanf's format string causes scanf to skip
    // any amount of whitespace after the number, which can block waiting
    // for non-whitespace. Here we read the whole stdin and split on
    // whitespace, which matches the typical usage where 10 numbers are
    // provided. This preserves the observable behavior for normal inputs.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let mut rank: [i32; 3] = [0; 3];

    for _ in 0..10 {
        // If the input doesn't provide 10 integers, we propagate a panic,
        // similar to how the original C program would have undefined behavior.
        let token = iter.next().expect("expected 10 integers on stdin");
        let work: i32 = token.parse().expect("invalid integer");

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

    for i in 0..3 {
        println!("{}", rank[i]);
    }
}
