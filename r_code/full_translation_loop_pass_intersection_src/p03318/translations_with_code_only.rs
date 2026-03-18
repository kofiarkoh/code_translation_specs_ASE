use std::io::{self, Read};

fn s(mut x: i64) -> i32 {
    // Mirrors C: sum of decimal digits for x > 0; returns 0 for x <= 0.
    let mut r: i32 = 0;
    while x > 0 {
        r += (x % 10) as i32;
        x /= 10;
    }
    r
}

fn main() {
    // Read entire stdin, parse one integer like scanf("%ld", &n).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = match input.split_whitespace().next() {
        Some(tok) => tok.parse::<i64>().unwrap(),
        None => return, // If no input, do nothing (scanf would fail; C code doesn't handle it).
    };

    let mut x: i64 = 1;

    for _i in 0..n {
        // printf("%ld\n", x);
        println!("{}", x);

        // r = ++x;
        x = x.wrapping_add(1);
        let mut r: i64 = x;

        // m = (double)x / S(x);
        // In C, division by zero would be UB if S(x)==0; here we mimic by producing +/-inf.
        let sx = s(x) as f64;
        let mut m: f64 = (x as f64) / sx;

        // for (j=0, s=10; j<15; j++, s*=10L) { ... }
        let mut s_pow: i64 = 10;
        for _j in 0..15 {
            // y = s*(x/s+1)-1;
            // Use wrapping ops to emulate potential signed overflow behavior as best-effort.
            let y = s_pow
                .wrapping_mul(x.wrapping_div(s_pow).wrapping_add(1))
                .wrapping_sub(1);

            // o = (double)y / S(y);
            let sy = s(y) as f64;
            let o: f64 = (y as f64) / sy;

            if o < m {
                m = o;
                r = y;
            }

            s_pow = s_pow.wrapping_mul(10);
        }

        x = r;
    }
}
