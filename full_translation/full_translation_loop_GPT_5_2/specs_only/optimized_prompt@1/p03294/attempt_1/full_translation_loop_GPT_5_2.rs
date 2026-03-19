use std::io::{self, Read};

fn main() {
    // Behavior is undefined by the specification for invalid/non-integer input or non-positive count.
    // We therefore parse strictly and let parsing failures panic (undefined behavior equivalent).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    if n <= 0 {
        // Undefined behavior per spec; choose to panic rather than invent output.
        panic!("non-positive count");
    }

    // Use i128 for accumulation to avoid introducing overflow behavior not specified.
    // The spec only requires "valid range for integer representation" but does not define overflow.
    let mut sum: i128 = 0;

    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        sum += (x as i128) - 1;
    }

    println!("{}", sum);
}
