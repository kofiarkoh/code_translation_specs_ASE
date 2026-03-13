use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Run {
    p: i32,
    t: f64,
}

fn main() {
    // Read all tokens from stdin, whitespace-separated, same as scanf("%d %lf", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // g[3][8]
    let mut g = [[Run { p: 0, t: 0.0 }; 8]; 3];

    // Fill g from tokens. This mirrors the nested scanf loops in the C code.
    for i in 0..3 {
        for j in 0..8 {
            // Parse p (int) then t (double)
            // If parsing fails, unwrap() will panic (this roughly corresponds to
            // the C program exhibiting undefined/erroneous behavior on bad input).
            let p_tok = toks.next().unwrap();
            let t_tok = toks.next().unwrap();
            let p: i32 = p_tok.parse().unwrap();
            let t: f64 = t_tok.parse().unwrap();
            g[i][j] = Run { p, t };
        }
    }

    // Bubble sort each row by t ascending, using the same comparison semantics as C's ">" on double.
    // In Rust, the '>' operator on f64 uses partial comparison and yields false if NaN is involved,
    // which matches C's behavior where comparisons with NaN are false.
    for i in 0..3 {
        for j in 0..7 {
            for k in (j + 1..8).rev() {
                if g[i][k - 1].t > g[i][k].t {
                    let tmp = g[i][k - 1];
                    g[i][k - 1] = g[i][k];
                    g[i][k] = tmp;
                }
            }
        }
    }

    // Print first two entries of each row: printf("%d %.2f\n", g[i][j].p, g[i][j].t);
    for i in 0..3 {
        for j in 0..2 {
            println!("{} {:.2}", g[i][j].p, g[i][j].t);
        }
    }

    // The C code does:
    // min1.t = 100;
    // (min1.p is left uninitialized in C). To avoid undefined behavior in Rust,
    // initialize min1.p and min2 with reasonable defaults (0). This preserves
    // the intended algorithm while avoiding UB.
    let mut min1 = Run { p: 0, t: 100.0 };
    let mut min2 = Run { p: 0, t: 0.0 };

    // Search among positions j = 2..7 in each row for the smallest two as in the C code.
    for i in 0..3 {
        for j in 2..8 {
            if g[i][j].t < min1.t {
                min2 = min1;
                min1 = g[i][j];
            }
        }
    }

    // Print min1 then min2 with the same formatting as the C program.
    println!("{} {:.2}", min1.p, min1.t);
    println!("{} {:.2}", min2.p, min2.t);
}
