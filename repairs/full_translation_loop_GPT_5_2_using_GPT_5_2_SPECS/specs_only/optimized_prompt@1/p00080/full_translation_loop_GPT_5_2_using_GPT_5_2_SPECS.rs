use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for tok in input.split_whitespace() {
        let q_i64: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => return,
        };

        // Termination value (from sample behavior): -1
        if q_i64 == -1 {
            break;
        }

        let q = q_i64 as f64;

        // Compute cube root approximation using Newton-Raphson, but match the
        // reference behavior by:
        // - using a fixed initial guess r=1
        // - iterating until |r^3 - q| < 1e-5 * q
        // - then printing r rounded to 6 decimals
        //
        // This choice also reproduces the slight overshoot seen in the sample output
        // (e.g., 64 -> 4.000009) which is typical of stopping based on cube-error
        // rather than minimizing absolute root error.
        let mut r = 1.0f64;
        let tol = 1e-5f64 * q;

        // For valid inputs q>0, this converges quickly; cap iterations to avoid
        // non-termination on ill-defined inputs.
        for _ in 0..1_000_000 {
            let r3 = r * r * r;
            if (r3 - q).abs() < tol {
                break;
            }
            // r_{n+1} = (2r + q/r^2)/3
            let r2 = r * r;
            r = (2.0 * r + q / r2) / 3.0;
        }

        println!("{:.6}", r);
    }
}
