use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let _c: i32 = it.next().unwrap().parse().unwrap(); // read but unused, as in C

    // In C: 0.5*(a*b) where a*b is computed in int first (may overflow/UB in C).
    // We emulate typical two's-complement wrapping behavior for i32 multiplication.
    let prod = a.wrapping_mul(b);
    let ans = 0.5f64 * (prod as f64);

    // printf("%.0lf", ans): round to nearest integer with ties away from zero.
    // Rust's format rounds ties to even, so implement C-like rounding explicitly.
    let rounded: f64 = if ans.is_nan() {
        f64::NAN
    } else if ans.is_infinite() {
        ans
    } else if ans >= 0.0 {
        (ans + 0.5).floor()
    } else {
        (ans - 0.5).ceil()
    };

    print!("{:.0}", rounded);
}
