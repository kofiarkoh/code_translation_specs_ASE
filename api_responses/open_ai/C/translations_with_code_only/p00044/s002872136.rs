use std::io::{self, Read};

fn main() {
    // Translate the C program behavior:
    // - Build an array `p` of primes in the same way as the C code.
    // - Append a sentinel value 60000 at p[n].
    // - For each input integer a, find smallest n such that p[n] > a (C used while(a >= p[n]) n++;).
    // - If p[n-1] == a print p[n-2], p[n] else print p[n-1], p[n].
    //
    // Notes on translation choices:
    // - The original C code used a local array `int p[80000];` (uninitialized except some entries).
    //   Reproducing uninitialized stack memory behavior is undefined in Rust. We allocate and
    //   initialize the array with zeros. This diverges from C only in corner cases where the C
    //   program would read uninitialized memory (e.g., inputs < 2 or >= 60000). For typical
    //   valid inputs (2..59999) behavior matches the C program.
    //
    // - All arithmetic and comparisons use i32 which matches typical C `int` ranges for this code.
    //
    // - I/O formatting (space and newline) is preserved exactly.

    // Prepare prime array p with the same logic as the C code.
    let mut p = vec![0i32; 80000];
    p[0] = 2;
    let mut n = 1usize;
    // a from 3 to 59999 inclusive (C: for(a=3;a<60000;a++))
    for a in 3..60000 {
        for b in 0..n {
            if a % p[b] == 0 {
                // composite, break out and do not add
                break;
            }
            // integer division semantics preserved by using i32
            if a / p[b] < p[b] {
                p[n] = a;
                n += 1;
                break;
            }
        }
    }
    // sentinel as in C: p[n] = 60000;
    p[n] = 60000;

    // Read all integers from stdin and process until EOF.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");
    let mut out = String::new();
    for token in input.split_whitespace() {
        // parse token to i32; if parsing fails, skip (mirrors scanf which would ignore non-integers)
        let a: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Find smallest index idx such that p[idx] > a
        let mut idx: usize = 0;
        // This mirrors the C loop: while(a >= p[n]) n++;
        while a >= p[idx] {
            idx += 1;
        }

        // Now mirror the C printing logic.
        // Note: In the original C code corner cases (idx == 0 or idx == 1) could produce
        // out-of-bounds access. Here we rely on normal inputs; indexing will panic on invalid
        // corner cases rather than producing undefined behavior.
        if p[idx - 1] == a {
            // print p[idx-2], p[idx]
            out.push_str(&format!("{} {}\n", p[idx - 2], p[idx]));
        } else {
            // print p[idx-1], p[idx]
            out.push_str(&format!("{} {}\n", p[idx - 1], p[idx]));
        }
    }

    // Write all output at once to match stdout behavior.
    print!("{}", out);
}
