// translated from the provided C program.
// Behavior: read integers from stdin until EOF; for each integer `n` print the number
// of 4-tuples (a,b,c,d) with 0<=a,b,c,d<=9 such that a+b+c+d == n, followed by a newline.
//
// Notes on translation choices:
// - We read all whitespace-separated tokens from stdin and attempt to parse them as i32.
//   The original C used scanf("%d",&n) which would behave differently on non-numeric input
//   (it would return 0 and could cause a different control flow). Typical use expects only
//   integers, so here we parse integers and ignore tokens that fail to parse.
// - All arithmetic is done with i32 to mirror typical 32-bit `int` behavior in C for this program.
// - Implementation is purely safe Rust.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, just exit (no output), similar to C program not entering loop.
        return;
    }

    for token in input.split_whitespace() {
        // Parse token as i32; skip tokens that are not valid integers.
        if let Ok(n) = token.parse::<i32>() {
            let mut count: i32 = 0;
            for a in 0..=9 {
                for b in 0..=9 {
                    for c in 0..=9 {
                        for d in 0..=9 {
                            if a + b + c + d == n {
                                count = count.wrapping_add(1);
                                // Use wrapping_add to mirror possible wraparound semantics of C's signed int
                                // (though for this problem the count will not overflow a 32-bit int).
                            }
                        }
                    }
                }
            }
            println!("{}", count);
        }
    }
}
