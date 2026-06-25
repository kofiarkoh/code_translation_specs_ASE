use std::io::{self, Read};

fn main() {
    // Read all input (behavior unspecified on invalid/missing input per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();

    // In the C code, variable `odd` actually tracks 2^E (E = count of even numbers).
    // Start at 1 = 2^0.
    let mut two_pow_e: i32 = 1;

    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        if a % 2 == 0 {
            two_pow_e *= 2;
        }
    }

    // Compute 3^N starting from 1.
    let mut three_pow_n: i32 = 1;
    for _ in 0..n {
        three_pow_n *= 3;
    }

    // Output exactly one integer, no trailing newline (matches C printf("%d", ...)).
    print!("{}", three_pow_n - two_pow_e);
}
