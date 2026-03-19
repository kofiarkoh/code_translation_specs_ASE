use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Preserve C's signed integer division semantics: truncation toward zero.
    let half = m / 2;

    let sum: i64;
    if m == 1 {
        sum = 0;
    } else if (half - n) < 0 {
        sum = half;
    } else if (half - n) < 2 {
        sum = n;
    } else {
        sum = n + (half - n) / 2;
    }

    // Match C printf("%lld", sum): no trailing newline.
    print!("{}", sum);
}
