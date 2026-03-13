// Translated from the C implementation described in the SPEC.
// Preserves the semantics: precompute minimal costs for totals 0..50 (units),
// then read integers from stdin until a zero sentinel and print the precomputed
// cost for floor(n/100) for each non-zero n.
//
// Note: The original C had a conditional expression that, due to operator
// precedence, looked ambiguous. The SPEC clarifies the intended behavior:
// update tbl[v] only when v <= 50 and (tbl[v] == 0 || tbl[v] > cost).
// This Rust code implements that intended logic.

use std::io::{self, Read};

fn combi(
    i: usize,
    v: i32,
    cost: i32,
    tbl: &mut [i32; 60],
    a: &[i32; 3],
    s: &[i32; 3],
    c: &[i32; 3],
    f: &[i32; 3],
) {
    if i == 3 {
        if v <= 50 {
            let idx = v as usize;
            if tbl[idx] == 0 || tbl[idx] > cost {
                tbl[idx] = cost;
            }
        }
        return;
    }

    // iterate j = 0, a[i], 2*a[i], ... while v + j <= 50
    let step = a[i] as usize;
    // Compute the maximum j such that v + j <= 50
    let max_add = 50 - v;
    if max_add < 0 {
        return;
    }
    let max_add_usize = max_add as usize;
    let mut j_usize = 0usize;
    while j_usize <= max_add_usize {
        let j = j_usize as i32;
        // cc = (number of full bundles) * f[i] + (remaining base-units) * s[i]
        let cc = (j / c[i]) * f[i] + ((j % c[i]) / a[i]) * s[i];
        combi(i + 1, v + j, cost + cc, tbl, a, s, c, f);

        // advance j by a[i]
        j_usize = j_usize.saturating_add(step);
        // loop will exit when j_usize > max_add_usize
    }
}

fn main() {
    // Arrays as in the C program
    let a: [i32; 3] = [2, 3, 5];
    let s: [i32; 3] = [380, 550, 850];
    let c: [i32; 3] = [10, 12, 15];
    let f: [i32; 3] = [1520, 1870, 2244];

    let mut tbl: [i32; 60] = [0; 60];

    // Precompute
    combi(0, 0, 0, &mut tbl, &a, &s, &c, &f);

    // Read all input, parse whitespace-separated integers, process in order.
    // For each non-zero n, print tbl[floor(n/100)] on its own line. Stop at first zero.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        // If parsing fails, skip the token (scanf in C would have different behavior;
        // the SPEC delegates undefined behavior for invalid input).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if n == 0 {
            break;
        }
        let idx = (n / 100) as usize;
        // This may panic if idx is out of 0..59, mirroring the SPEC/C program's
        // undefined behavior for out-of-range queries.
        println!("{}", tbl[idx]);
    }
}
