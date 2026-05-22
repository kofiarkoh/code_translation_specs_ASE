use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let _c: i32 = it.next().unwrap().parse().unwrap(); // read but unused, as in C

    // In C: a*b is computed in int (with potential signed overflow UB), then converted to double.
    // We emulate typical two's-complement wraparound for overflow using wrapping_mul.
    let prod: i32 = a.wrapping_mul(b);
    let ans: f64 = 0.5f64 * (prod as f64);

    // C printf("%.0lf", ans): round to nearest with ties-to-even (IEEE-754) and no newline.
    print!("{:.0}", ans);
}
