use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let target: i128 = it.next().unwrap().parse().unwrap();
    let divisor: i128 = it.next().unwrap().parse().unwrap();
    let count: i128 = it.next().unwrap().parse().unwrap();

    // Specification states divisor must be non-zero; if zero, behavior is undefined.
    // We intentionally do not add explicit handling beyond letting Rust panic on division by zero
    // if it occurs in the computation.
    //
    // Specification is ambiguous about the exact "adjustment" operation. The narrowest
    // interpretation consistent with "adjusting the value according to specified divisors and a
    // count of operations" and "constant time" is to apply a single arithmetic adjustment that
    // incorporates both divisor and count, then take absolute value.
    //
    // We compute: abs(target + divisor * count)
    // This uses a fixed number of arithmetic operations regardless of input values.
    let adjusted = target + divisor * count;

    // Absolute value with i128::MIN corner case: abs(i128::MIN) is not representable.
    // The specification does not define overflow behavior; we choose a narrow, deterministic
    // behavior by saturating to i128::MAX in that single unrepresentable case.
    let result = if adjusted == i128::MIN {
        i128::MAX
    } else {
        adjusted.abs()
    };

    print!("{result}");
}
