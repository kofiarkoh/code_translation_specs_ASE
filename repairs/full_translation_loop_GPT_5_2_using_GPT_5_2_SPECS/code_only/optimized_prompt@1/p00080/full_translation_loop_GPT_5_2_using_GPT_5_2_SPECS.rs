use std::io::{self, Read};

const EPS: f64 = 1e-5;

fn main() {
    // Read entire stdin, then parse ints like scanf("%d") would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    while let Some(tok) = it.next() {
        // If parsing fails, stop (scanf would fail to match and return 0, not EOF).
        let q_i32: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if q_i32 == -1 {
            break;
        }

        let q = q_i32 as f64;

        // Mirrors: px = (double)q / 2;
        let mut px = q / 2.0;
        let x: f64;

        // do { ... } while (...)
        loop {
            // x = px - (px*px*px - q) / (3*px*px);
            // Note: if px == 0.0, this will produce NaN/Inf similarly to C's IEEE-754 behavior.
            let x_new = px - (px * px * px - q) / (3.0 * px * px);
            px = x_new;

            // while (fabs(x*x*x - q) >= EPS * q);
            // Keep the same comparison structure; for negative q, EPS*q is negative,
            // so the condition is always true unless fabs(...) is NaN (then comparison is false),
            // matching typical C behavior with IEEE-754.
            if (x_new * x_new * x_new - q).abs() < EPS * q {
                x = x_new;
                break;
            }
        }

        // printf("%.6lf\n", x);
        println!("{:.6}", x);
    }
}
