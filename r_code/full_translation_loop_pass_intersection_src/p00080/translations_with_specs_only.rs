use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let eps: f64 = 1e-5;

    for tok in input.split_whitespace() {
        let q_i64: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified beyond termination on input failure
        };

        if q_i64 == -1 {
            break;
        }

        // Specification intends q > 0; behavior for q <= 0 is not defined.
        let q: f64 = q_i64 as f64;

        // p0 = q/2 (as a double)
        let mut p: f64 = q / 2.0;

        // Iterate Newton–Raphson for x^3 = q until |x^3 - q| < eps * q.
        // Print the final x produced when the loop terminates.
        let x_final: f64 = loop {
            // x = p - (p^3 - q) / (3 p^2)
            // Algebraically: x = (2p + q/p^2) / 3, but we keep the given form.
            let x = p - (p * p * p - q) / (3.0 * p * p);
            p = x;

            if (x * x * x - q).abs() < eps * q {
                break x;
            }
        };

        println!("{:.6}", x_final);
    }
}
