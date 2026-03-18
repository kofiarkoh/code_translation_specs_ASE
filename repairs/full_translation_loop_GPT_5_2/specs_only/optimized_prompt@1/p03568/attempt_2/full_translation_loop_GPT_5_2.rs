use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: u64 = it.next().unwrap().parse().unwrap();

    let mut even_count: u64 = 0;
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if x % 2 == 0 {
            even_count += 1;
        }
    }

    // The specification does not define overflow behavior.
    // Use wrapping arithmetic on a fixed-width signed integer to ensure defined behavior in Rust.
    // This matches common C-like behavior where overflow wraps in two's complement.
    let mut three_pow: i128 = 1;
    for _ in 0..n {
        three_pow = three_pow.wrapping_mul(3);
    }

    let mut two_pow: i128 = 1;
    for _ in 0..even_count {
        two_pow = two_pow.wrapping_mul(2);
    }

    let result = three_pow.wrapping_sub(two_pow);
    println!("{result}");
}
