use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed 64-bit integer (like scanf("%lld", &x)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let x: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // In C, x/11 truncates toward zero and x%11 has the sign of x.
    // Rust's / and % on i64 match these semantics.
    let r = x % 11;

    let ans: i64 = if r == 0 {
        2 * (x / 11)
    } else if r > 6 {
        2 * (x / 11) + 2
    } else {
        2 * (x / 11) + 1
    };

    // Match C's printf("%lld\n", ...)
    print!("{ans}\n");
}
