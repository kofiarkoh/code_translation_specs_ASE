use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated i64 values.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    // The C code uses 1-based indexing with arrays of size 3; we just use scalars.
    let t1 = it.next().unwrap();
    let t2 = it.next().unwrap();
    let mut a1 = it.next().unwrap();
    let mut a2 = it.next().unwrap();
    let mut b1 = it.next().unwrap();
    let mut b2 = it.next().unwrap();

    // If total contributions are equal, output "infinity".
    if a1 * t1 + a2 * t2 == b1 * t1 + b2 * t2 {
        print!("infinity\n");
        return;
    }

    // Ensure a1 <= b1 by swapping A and B if needed (matches the C swap logic).
    if a1 > b1 {
        std::mem::swap(&mut a1, &mut b1);
        std::mem::swap(&mut a2, &mut b2);
    }

    let x = (b1 - a1) * t1;
    let y = (a2 - b2) * t2;

    if x > y {
        print!("0\n");
    } else {
        let denom = y - x;
        // C uses % and / on signed long long; Rust i64 matches truncation toward zero.
        if x % denom == 0 {
            print!("{}\n", (x / denom) * 2);
        } else {
            print!("{}\n", (x / denom) * 2 + 1);
        }
    }
}
