use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Use wrapping arithmetic to emulate typical "long" overflow behavior.
    let sum = a.wrapping_add(b);

    if (sum & 1) == 0 {
        // For even sums, integer division by 2 is exact.
        let avg = sum / 2;
        print!("{avg}");
    } else {
        print!("IMPOSSIBLE");
    }
}
