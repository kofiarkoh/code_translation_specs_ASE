use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse a single integer.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Follow the specification/formula as implemented in the C code.
    let rem = n % 11;
    let res: i64 = if rem == 0 {
        0
    } else if (1..=6).contains(&rem) {
        1
    } else {
        2
    };

    let ans = (n / 11) * 2 + res;

    // Preserve exact output formatting: no trailing newline.
    print!("{ans}");
}
