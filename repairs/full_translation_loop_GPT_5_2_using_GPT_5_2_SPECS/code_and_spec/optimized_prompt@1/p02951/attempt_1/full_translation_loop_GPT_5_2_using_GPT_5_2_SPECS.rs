use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers.
    // Per spec: behavior is only defined when parsing succeeds.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    // Follow the specification (which matches the C loop's net effect):
    // D = A - B
    // If D <= 0: C' = C
    // If D > 0 and C > 0: C' = max(0, C - D)
    // If D > 0 and C <= 0: C' = C - D
    let d = a - b;
    let c_prime = if d <= 0 {
        c
    } else if c > 0 {
        // Decrease by up to D steps, stopping early at zero.
        let dec = std::cmp::min(d, c);
        c - dec
    } else {
        // No early stop when C is non-positive.
        c - d
    };

    print!("{c_prime}\n");
}
