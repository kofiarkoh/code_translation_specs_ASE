use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse three integers (third is unused per spec)
    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let _c: i64 = it.next().unwrap().parse().unwrap();

    // Spec: output is half the product of the first two integers,
    // rounded to the nearest whole number (matches C's printf("%.0lf") rounding).
    let ans = 0.5f64 * ((a * b) as f64);
    print!("{:.0}", ans);
}
