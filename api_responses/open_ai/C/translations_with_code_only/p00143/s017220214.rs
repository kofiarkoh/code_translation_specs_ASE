// Translated from C: Aizu Vol-2 0143   Altair and Vega
// Preserves semantics of original C code. Arithmetic uses wrapping
// operations to emulate typical 32-bit two's-complement behavior
// (C signed overflow is undefined, but this preserves the usual wraparound).

use std::io::{self, Read};

fn at_same_side(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    xa: i32,
    ya: i32,
    xb: i32,
    yb: i32,
) -> bool {
    // Use wrapping arithmetic to emulate C's typical 32-bit overflow behavior.
    let dx = x2.wrapping_sub(x1);
    let dy = y2.wrapping_sub(y1);
    let ya_y1 = ya.wrapping_sub(y1);
    let yb_y1 = yb.wrapping_sub(y1);
    let x1_xa = x1.wrapping_sub(xa);
    let x1_xb = x1.wrapping_sub(xb);

    let sa = dx.wrapping_mul(ya_y1).wrapping_add(dy.wrapping_mul(x1_xa));
    let sb = dx.wrapping_mul(yb_y1).wrapping_add(dy.wrapping_mul(x1_xb));

    // In C: return (sa * sb >= 0);
    // Use wrapping_mul for product to preserve wraparound, then check sign.
    let prod = sa.wrapping_mul(sb);
    prod >= 0
}

fn main() {
    // Read entire stdin and parse as i32 tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Read n
    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    for _ in 0..n {
        // Read 10 integers per test case
        let x1: i32 = it.next().unwrap().parse().unwrap();
        let y1: i32 = it.next().unwrap().parse().unwrap();
        let x2: i32 = it.next().unwrap().parse().unwrap();
        let y2: i32 = it.next().unwrap().parse().unwrap();
        let x3: i32 = it.next().unwrap().parse().unwrap();
        let y3: i32 = it.next().unwrap().parse().unwrap();
        let xk: i32 = it.next().unwrap().parse().unwrap();
        let yk: i32 = it.next().unwrap().parse().unwrap();
        let xs: i32 = it.next().unwrap().parse().unwrap();
        let ys: i32 = it.next().unwrap().parse().unwrap();

        let kin = at_same_side(x1, y1, x2, y2, x3, y3, xk, yk)
            && at_same_side(x2, y2, x3, y3, x1, y1, xk, yk)
            && at_same_side(x3, y3, x1, y1, x2, y2, xk, yk);

        let sin = at_same_side(x1, y1, x2, y2, x3, y3, xs, ys)
            && at_same_side(x2, y2, x3, y3, x1, y1, xs, ys)
            && at_same_side(x3, y3, x1, y1, x2, y2, xs, ys);

        // In C: puts(kin ^ sin ? "OK" : "NG");
        // kin and sin are booleans here; ^ is boolean XOR.
        if kin ^ sin {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
