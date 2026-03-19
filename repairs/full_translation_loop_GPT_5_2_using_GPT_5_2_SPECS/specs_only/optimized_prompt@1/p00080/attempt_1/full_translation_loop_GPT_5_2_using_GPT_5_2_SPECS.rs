use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for tok in input.split_whitespace() {
        let q_i64: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => {
                // Behavior for non-integer data is not inferable; choose to stop quietly.
                return;
            }
        };

        // Termination value: not specified; choose the narrowest common sentinel.
        // Many such problems use 0 as the terminator, and the spec allows a special
        // termination value that may appear. This also aligns with the precondition
        // that non-terminating inputs are strictly positive.
        if q_i64 == 0 {
            break;
        }

        // For q <= 0 (non-terminating), behavior is not reliably defined by the spec.
        // We do not attempt to define it; proceed with computation which may yield
        // nonsensical results, matching "not reliably defined".
        let q = q_i64 as f64;

        // Compute r such that |r^3 - q| < 1e-5 * q.
        // Use Newton-Raphson on f(r) = r^3 - q.
        // Start with a simple positive guess for q > 0.
        let mut r = if q > 1.0 { q } else { 1.0 };

        // Fixed tolerance scaled by q, as specified.
        let tol = 1e-5 * q;

        // Iterate with a conservative cap to avoid infinite loops on ill-defined inputs.
        // For valid q > 0, Newton's method converges quickly.
        for _ in 0..10_000 {
            let r3 = r * r * r;
            let err = (r3 - q).abs();

            if err < tol {
                break;
            }

            // Newton update: r_{n+1} = r - (r^3 - q) / (3 r^2) = (2r + q/r^2)/3
            // If r is 0, this would divide by zero; for valid q>0 and our initialization,
            // r should not become 0, but for ill-defined inputs we allow IEEE behavior.
            let r2 = r * r;
            r = (2.0 * r + q / r2) / 3.0;
        }

        // Output rounded to six digits after the decimal point.
        println!("{:.6}", r);
    }
}
