use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace-separated parsing)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Use i64 to match C's typical "long long int" width.
    // Integer division in Rust truncates toward zero, matching C99+ semantics.
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
