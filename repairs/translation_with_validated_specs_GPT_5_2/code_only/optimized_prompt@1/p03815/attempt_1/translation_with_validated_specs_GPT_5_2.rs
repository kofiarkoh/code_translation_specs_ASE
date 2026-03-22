use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed 64-bit integer (matches C's long long on typical targets).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let x: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // In C, signed overflow is undefined; we use wrapping ops to emulate typical two's-complement behavior.
    let r = x % 11;
    let q = x / 11;

    let ans = if r == 0 {
        q.wrapping_mul(2)
    } else if r > 6 {
        q.wrapping_mul(2).wrapping_add(2)
    } else {
        q.wrapping_mul(2).wrapping_add(1)
    };

    print!("{}\n", ans);
}
