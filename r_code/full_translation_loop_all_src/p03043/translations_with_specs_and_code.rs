use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut k: i32 = it.next().unwrap().parse().unwrap();

    // C uses int; keep i32 for the same overflow behavior domain (though Rust panics on debug overflow).
    // For the floating computations, use f64 as the closest common Rust equivalent to C's long double.
    let mut multi: i32 = 1;
    let mut i: i32 = n;

    // Invariant: array size is always N
    let mut denomi: Vec<i32> = vec![0; n as usize];

    while i > 0 {
        let idx = (i - 1) as usize;
        denomi[idx] = 0;

        // Preserve C loop condition and update order
        while k > multi * i {
            multi *= 2;
        }
        denomi[idx] += multi * n;

        i -= 1;
    }

    let mut nume: f64 = 0.0;
    for j in 0..(n as usize) {
        // In C: denomi[0]/denomi[j] is integer division (both ints), then promoted to long double.
        let term_int: i32 = denomi[0] / denomi[j];
        nume += term_int as f64;
    }

    // In C: nume/denomi[0] where denomi[0] is int, promoted to long double.
    let result = nume / (denomi[0] as f64);

    // Exactly 12 digits after decimal, no trailing newline (matches C printf behavior here)
    print!("{:.12}", result);
}
