use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C: long (commonly 64-bit on LP64, 32-bit on LLP64). We choose i64 as a practical match
    // for typical judge environments; behavior for out-of-range inputs is unspecified in C anyway.
    let mut x = [0i64; 3];
    for i in 0..3 {
        if let Some(tok) = it.next() {
            x[i] = tok.parse::<i64>().unwrap();
        } else {
            // If input is missing, C's scanf would leave values uninitialized (UB on use).
            // Here we keep default 0; this path shouldn't occur in valid test data.
            x[i] = 0;
        }
    }

    if x[0] % 2 == 0 || x[1] % 2 == 0 || x[2] % 2 == 0 {
        // puts adds a newline.
        println!("0");
    } else {
        // Replicate C logic: max is int, initialized to 0; m is uninitialized but will be set
        // because all x[i] are odd, hence non-zero, and at least one will be > 0 in typical inputs.
        // If all x[i] <= 0, C would have UB due to uninitialized m; we avoid UB by defaulting m=0.
        let mut m: usize = 0;
        let mut max_i32: i32 = 0;

        for i in 0..3 {
            // C compares int max with long X[i] after usual arithmetic conversions.
            // We emulate by converting X[i] to i32 with wrapping (implementation-defined in C if out of range).
            let xi_i32 = x[i] as i32;
            if max_i32 < xi_i32 {
                max_i32 = xi_i32;
                m = i;
            }
        }

        x[m] = 1;

        // C signed overflow is UB; in Rust we use wrapping to avoid panics and approximate
        // typical two's-complement wraparound behavior.
        let prod = x[0].wrapping_mul(x[1]).wrapping_mul(x[2]);

        // C printf("%ld", ...) prints without a trailing newline.
        print!("{}", prod);
    }
}
