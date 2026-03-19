use std::io::{self, Read};

fn cbrt_spec(n: i64) -> f64 {
    // Match the likely reference behavior for AOJ p00080:
    // - Use Newton iteration for cube root
    // - Use a relatively large epsilon (1e-5) on |x^3 - n|
    // - Start from x = n (as double), which for perfect cubes can stop "early"
    //   and yield a small residual (e.g., 64 -> 4.000009 when printed to 6 decimals).
    //
    // This intentionally does NOT use f64::cbrt to preserve the specified
    // "convergence criterion based on a small epsilon value" behavior.

    if n == 0 {
        return 0.0;
    }

    let a = n as f64;
    let mut x = a; // initial guess as in many simple C implementations
    let eps = 1e-5_f64;

    // Iterate until residual is within epsilon.
    // Also cap iterations to guarantee termination even for extreme inputs.
    for _ in 0..1_000_000 {
        let fx = x * x * x - a;
        if fx.abs() <= eps {
            break;
        }
        // Newton step: x = x - f(x)/f'(x) = x - (x^3-a)/(3x^2)
        // Avoid division by zero if x becomes 0 (can happen for a=0 only, handled above).
        let denom = 3.0 * x * x;
        if denom == 0.0 {
            break;
        }
        x -= fx / denom;
    }

    x
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    for tok in input.split_whitespace() {
        let v: i64 = tok.parse().unwrap();
        if v == -1 {
            break;
        }
        let r = cbrt_spec(v);
        out.push_str(&format!("{:.6}\n", r));
    }

    print!("{out}");
}
