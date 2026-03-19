use std::io::{self, Read};

const EPS: f64 = 1e-5;

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (as scanf("%d") would).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // Spec precondition: input is valid integer; undefined otherwise.
        let q: i32 = tok.parse().unwrap();
        if q == -1 {
            break;
        }

        // C code uses Newton-Raphson starting from q/2.
        // Note: The C stopping condition is `fabs(x^3 - q) >= EPS * q`.
        // This is problematic for q <= 0 (negative threshold), but the spec says
        // the cube root is always real; we preserve the C behavior as written.
        let qf = q as f64;

        let mut px: f64 = qf / 2.0;
        let x: f64;

        loop {
            // x = px - (px^3 - q) / (3*px^2)
            // This matches the C formula exactly.
            let next = px - (px * px * px - qf) / (3.0 * px * px);
            px = next;

            // do-while: check after updating
            if (next * next * next - qf).abs() < EPS * qf {
                x = next;
                break;
            }
        }

        println!("{:.6}", x);
    }
}
