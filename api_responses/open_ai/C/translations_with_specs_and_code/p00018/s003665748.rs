use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    // We unwrap here: under the SPEC the program assumes stdin contains at least five integers.
    // If that precondition holds, unwrap will not fail. This mirrors the C program which does
    // not check scanf return values.
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    // Use i32 as the natural Rust analogue for C's `int` on common platforms.
    // (If the platform's C `int` has a different range, that is a platform-level mismatch;
    // the SPEC expects values representable as C int.)
    let mut a: [i32; 5] = [0; 5];

    // Read exactly five integer tokens. Per SPEC we do not perform extra validation.
    for i in 0..5 {
        // parse and unwrap: matches the SPEC's assumption of valid input
        a[i] = iter.next().unwrap().parse::<i32>().unwrap();
    }

    // Same sorting logic as the C code: nested loops, swap when a[i] < a[j]
    // to produce non-increasing (descending) order.
    for i in 0..5 {
        for j in i..5 {
            if a[i] < a[j] {
                let t = a[i];
                a[i] = a[j];
                a[j] = t;
            }
        }
    }

    // Print five integers separated by single spaces, with a trailing space after the last one.
    // No terminating newline is printed, matching the SPEC.
    for i in 0..5 {
        print!("{} ", a[i]);
    }
}
