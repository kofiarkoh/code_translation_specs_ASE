use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    // Specification: inputs are non-negative integers.
    // Use u128 to avoid overflow in intermediate arithmetic for typical constraints.
    // (Spec does not define overflow behavior; choosing a wide type is the narrowest
    // reasonable way to preserve intended math semantics.)
    let h: u128 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: u128 = it.next().unwrap().parse().unwrap();
    let k: u128 = it.next().unwrap().parse().unwrap();

    // Invariant: i, j are non-negative integers.
    // Bound: i <= h, j <= w (as stated in the specification).
    for i in 0..=h {
        for j in 0..=w {
            let expr = w * i + h * j - 2u128 * i * j;
            if expr == k {
                print!("Yes");
                return;
            }
        }
    }

    print!("No");
}
