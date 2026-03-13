// translated from the provided C program
// - Uses i32 to mirror C 'int' on typical platforms.
// - Reads exactly 10 integers from stdin (whitespace-separated).
//   The code uses `expect`/`parse().expect(...)` to mirror the original
//   program's assumption that valid integers are provided; if not,
//   the program will panic (similar to the original undefined / unchecked behavior).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    // Read all stdin (blocking until EOF), then split on whitespace.
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut tokens = input.split_whitespace();
    let mut rank: [i32; 3] = [0, 0, 0];

    for _ in 0..10 {
        // The original C used scanf without checking return value; here we
        // assume input contains at least 10 valid integers and unwrap accordingly.
        let work_str = tokens.next().expect("expected 10 integers");
        let work: i32 = work_str.parse().expect("invalid integer");

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

    // Print the three ranks, each followed by a newline, matching the C program.
    for i in 0..3 {
        println!("{}", rank[i]);
    }
}
