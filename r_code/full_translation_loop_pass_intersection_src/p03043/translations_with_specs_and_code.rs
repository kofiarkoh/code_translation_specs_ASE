use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers N and K (well-formed per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    // Preconditions (N > 0, no overflow) are assumed per spec; no explicit error handling.
    let n_usize = n as usize;

    let mut multi: i32 = 1;
    let mut denomi: Vec<i32> = vec![0; n_usize];

    // Construct denominators for i = N down to 1, storing at index i-1.
    let mut i = n;
    while i > 0 {
        while k > multi * i {
            multi *= 2;
        }
        denomi[(i - 1) as usize] = multi * n;
        i -= 1;
    }

    // Compute S = sum_{j=0..N-1} denomi[0] / denomi[j] in long double terms.
    // In C, denomi[0]/denomi[j] is integer division, then promoted to long double.
    let denom0 = denomi[0];
    let mut nume: f64 = 0.0;
    for j in 0..n_usize {
        nume += (denom0 / denomi[j]) as f64; // integer division preserved
    }

    // Print nume/denomi[0] with exactly 12 digits after decimal, no trailing newline.
    let result = nume / (denom0 as f64);
    print!("{:.12}", result);
}
