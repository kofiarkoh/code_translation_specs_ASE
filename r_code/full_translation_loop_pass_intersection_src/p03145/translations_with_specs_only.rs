use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Behavior is unspecified if parsing fails or fewer than three integers exist.
    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();
    let _c: i128 = it.next().unwrap().parse().unwrap(); // explicitly ignored

    // Preconditions state the product is representable (no overflow in the intended domain).
    let prod: i128 = a * b;

    // Compute round((a*b)/2) with ties away from zero, matching typical "round" semantics.
    // For integer prod:
    // - if prod is even: exact division
    // - if prod is odd: add/subtract 1 before dividing to round away from zero
    let result: i128 = if prod % 2 == 0 {
        prod / 2
    } else if prod > 0 {
        (prod + 1) / 2
    } else {
        (prod - 1) / 2
    };

    print!("{result}");
}
