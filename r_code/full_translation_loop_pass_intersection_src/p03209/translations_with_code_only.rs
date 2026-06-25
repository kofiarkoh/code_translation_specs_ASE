use std::io::{self, Read};

type LL = i64;

fn f(n: LL, x: LL) -> LL {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    // In C: (1LL<<n) is undefined if n is negative or >= 64.
    // We emulate typical two's-complement behavior by using wrapping shifts.
    let p: LL = ((1i64).wrapping_shl(n as u32)).wrapping_sub(1);

    if x <= 2i64.wrapping_mul(p) {
        f(n - 1, x - 1)
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: LL = it.next().unwrap().parse().unwrap();
    let k: LL = it.next().unwrap().parse().unwrap();

    print!("{}", f(n, k));
}
