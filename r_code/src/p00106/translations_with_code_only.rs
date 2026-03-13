// Translated from the given C program.
// Preserves semantics of the original, using i32 for C 'int' and keeping the same combination logic.
//
// Notes on semantic preservation:
// - The original C code used a global int tbl[60] initialized to 0. We use [i32; 60] initialized to 0.
// - The conditional in the base case: (v <= 50 && tbl[v] == 0 || tbl[v] > cost)
//   is preserved exactly as ((v <= 50 && tbl[v] == 0) || tbl[v] > cost).
// - Integer divisions are for non-negative values in the algorithm, matching C's behavior here.
// - This translation does not add bounds checks beyond what Rust already enforces. If inputs cause
//   an out-of-range index as in the original C (undefined behavior), this program may panic on conversion/indexing.
// - No unsafe is needed.

use std::io::{self, Read};

fn combi(
    i: usize,
    v: usize,
    cost: i32,
    tbl: &mut [i32; 60],
    a: &[i32; 3],
    s: &[i32; 3],
    c: &[i32; 3],
    f: &[i32; 3],
) {
    if i == 3 {
        // Preserve the original conditional: (v <= 50 && tbl[v] == 0 || tbl[v] > cost)
        if (v <= 50 && tbl[v] == 0) || tbl[v] > cost {
            tbl[v] = cost;
        }
        return;
    }

    let step = a[i] as usize;
    let mut j: usize = 0;
    while v + j <= 50 {
        let j_i32 = j as i32;
        let cc = (j_i32 / c[i]) * f[i] + ((j_i32 % c[i]) / a[i]) * s[i];
        combi(i + 1, v + j, cost + cc, tbl, a, s, c, f);
        j += step;
    }
}

fn main() {
    // Arrays as in the C code
    let a: [i32; 3] = [2, 3, 5];
    let s: [i32; 3] = [380, 550, 850];
    let c: [i32; 3] = [10, 12, 15];
    let f: [i32; 3] = [1520, 1870, 2244];

    let mut tbl: [i32; 60] = [0; 60];

    combi(0, 0, 0, &mut tbl, &a, &s, &c, &f);

    // Read all input integers and process them in order, stopping at 0.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");
    for token in input.split_whitespace() {
        let n: i32 = match token.parse() {
            Ok(x) => x,
            Err(_) => continue,
        };
        if n == 0 {
            break;
        }
        // Preserve integer division n/100 as in C.
        let idx = (n / 100) as usize;
        // Print exactly "%d\n" per input as in the C program.
        println!("{}", tbl[idx]);
    }
}
