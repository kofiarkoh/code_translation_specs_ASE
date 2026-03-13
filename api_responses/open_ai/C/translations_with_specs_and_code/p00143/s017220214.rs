// altair_and_vega.rs
// Translated from the provided C program according to the SPEC.
// Behavior: For each test case, reads triangle vertices and two points K and S (all i32).
// Prints "OK" if exactly one of K or S is inside-or-on-boundary of the triangle, else "NG".
//
// Notes on translation choices:
// - Integer type: i32 is used to mirror C's `int` semantics as specified.
// - Arithmetic uses wrapping operations to avoid debug-build panics on overflow and to
//   approximate the unspecified/architecture-dependent overflow behavior of C.
//   This preserves the program's tendency to rely solely on integer arithmetic and sign tests.

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
    // Compute cross-like values using wrapping arithmetic to avoid panics on overflow.
    // The C code computed:
    // sa = (x2 - x1) * (ya - y1) + (y2 - y1) * (x1 - xa);
    // sb = (x2 - x1) * (yb - y1) + (y2 - y1) * (x1 - xb);
    //
    // which algebraically equals the standard cross products
    // (x2-x1)*(ya-y1) - (y2-y1)*(xa-x1), etc.
    let dx = x2.wrapping_sub(x1);
    let dy = y2.wrapping_sub(y1);

    let t1 = dx.wrapping_mul(ya.wrapping_sub(y1));
    let t2 = dy.wrapping_mul(x1.wrapping_sub(xa));
    let sa = t1.wrapping_add(t2);

    let u1 = dx.wrapping_mul(yb.wrapping_sub(y1));
    let u2 = dy.wrapping_mul(x1.wrapping_sub(xb));
    let sb = u1.wrapping_add(u2);

    // Return true iff sa*sb >= 0 (same sign or either zero).
    sa.wrapping_mul(sb) >= 0
}

fn main() {
    // Read all whitespace-separated tokens from stdin and parse as i32.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let vals: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Follow the C program's behavior: first value is n, then exactly 10*n integers follow.
    // If input is malformed or has insufficient tokens, parsing/indexing will panic (matches SPEC's undefined behavior).
    let mut idx: usize = 0;
    let n = vals[idx];
    idx += 1;

    let mut remaining = n;
    while remaining > 0 {
        // Read 10 integers per test case in order.
        // x1,y1, x2,y2, x3,y3, xk,yk, xs,ys
        let x1 = vals[idx]; let y1 = vals[idx + 1];
        let x2 = vals[idx + 2]; let y2 = vals[idx + 3];
        let x3 = vals[idx + 4]; let y3 = vals[idx + 5];
        let xk = vals[idx + 6]; let yk = vals[idx + 7];
        let xs = vals[idx + 8]; let ys = vals[idx + 9];
        idx += 10;

        let kin = at_same_side(x1, y1, x2, y2, x3, y3, xk, yk)
            && at_same_side(x2, y2, x3, y3, x1, y1, xk, yk)
            && at_same_side(x3, y3, x1, y1, x2, y2, xk, yk);

        let sin = at_same_side(x1, y1, x2, y2, x3, y3, xs, ys)
            && at_same_side(x2, y2, x3, y3, x1, y1, xs, ys)
            && at_same_side(x3, y3, x1, y1, x2, y2, xs, ys);

        if kin != sin {
            println!("OK");
        } else {
            println!("NG");
        }

        remaining -= 1;
    }
}
