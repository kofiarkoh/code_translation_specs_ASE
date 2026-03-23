use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume tokens separated by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // C uses signed long long; in Rust we use i64.
    // Division in C for signed integers truncates toward zero; Rust matches that.
    let sum: i64;
    if m == 1 {
        sum = 0;
    } else if (m / 2 - n) < 0 {
        sum = m / 2;
    } else if (m / 2 - n) < 2 {
        sum = n;
    } else {
        sum = n + (m / 2 - n) / 2;
    }

    // Match C printf("%lld", sum): no trailing newline.
    print!("{}", sum);
}
