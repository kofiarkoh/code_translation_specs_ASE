use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume formatted input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let _c: i32 = it.next().unwrap().parse().unwrap(); // read but unused, matching C

    // In C: a*b is computed in int, which can overflow (UB). We emulate typical
    // two's-complement wraparound by using wrapping_mul.
    let prod: i32 = a.wrapping_mul(b);

    // Then converted to double and multiplied by 0.5.
    let ans: f64 = 0.5f64 * (prod as f64);

    // printf("%.0lf", ans): round to nearest integer with ties to even.
    // Rust's format "{:.0}" uses the same rounding behavior for f64.
    print!("{:.0}", ans);
}
