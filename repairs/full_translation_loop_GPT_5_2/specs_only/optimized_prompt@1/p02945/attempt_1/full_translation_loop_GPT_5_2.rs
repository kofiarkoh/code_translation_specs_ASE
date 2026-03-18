use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Use wrapping arithmetic to emulate typical fixed-width integer behavior
    // in the presence of overflow, rather than panicking.
    let sum = a.wrapping_add(b);
    let diff = a.wrapping_sub(b);
    let prod = a.wrapping_mul(b);

    let mut max_val = sum;
    if diff > max_val {
        max_val = diff;
    }
    if prod > max_val {
        max_val = prod;
    }

    print!("{}", max_val);
}
