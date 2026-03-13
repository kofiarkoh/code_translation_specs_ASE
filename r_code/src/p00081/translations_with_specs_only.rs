use std::io::{self, Read};

fn main() {
    // Read entire stdin. The original specification describes a C-style program
    // using scanf with the exact pattern "%lf,%lf,%lf,%lf,%lf,%lf".
    // We emulate the observable behavior: expect comma-separated numeric tokens
    // (six per record). If parsing fails or there are insufficient tokens,
    // we stop processing (behavior is unspecified for malformed input).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (mirrors unspecified/undefined behavior in spec).
        return;
    }

    // Split on literal commas. This follows the spec's requirement of
    // comma-delimited values. We trim each token so that typical line endings
    // don't prevent parsing; this is a narrow, reasonable interpretation.
    let parts: Vec<&str> = input.split(',').collect();

    let mut idx = 0;
    while idx + 6 <= parts.len() {
        // Parse six consecutive tokens as f64. If any parse fails, stop.
        let mut vals = [0f64; 6];
        let mut ok = true;
        for j in 0..6 {
            let tok = parts[idx + j].trim();
            match tok.parse::<f64>() {
                Ok(v) => vals[j] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            // Malformed token: specification leaves behavior unspecified.
            // We stop processing further records.
            break;
        }
        idx += 6;

        let p1x = vals[0];
        let p1y = vals[1];
        let p2x = vals[2];
        let p2y = vals[3];
        let qx = vals[4];
        let qy = vals[5];

        let (rx, ry) = if p1x == p2x && p1y == p2y {
            // Degenerate case: P1 == P2, reflect Q about the point P1.
            (2.0 * p1x - qx, 2.0 * p1y - qy)
        } else if p1y == p2y && p1x != p2x {
            // Horizontal line y = p1y
            (qx, 2.0 * p1y - qy)
        } else if p1x == p2x && p1y != p2y {
            // Vertical line x = p1x
            (2.0 * p1x - qx, qy)
        } else {
            // Generic case: line through P1 and P2, neither horizontal nor vertical.
            // Use line in form a*x + b*y + c = 0 where:
            // a = y1 - y2, b = x2 - x1, c = x1*y2 - x2*y1
            // Reflection formula for point (x0,y0):
            // d = (a*x0 + b*y0 + c) / (a*a + b*b)
            // xr = x0 - 2*a*d
            // yr = y0 - 2*b*d
            let a = p1y - p2y;
            let b = p2x - p1x;
            let c = p1x * p2y - p2x * p1y;
            let denom = a * a + b * b;
            // According to preconditions in the specification, denom should be non-zero
            // in this branch. We perform the arithmetic straightforwardly.
            let d = (a * qx + b * qy + c) / denom;
            let xr = qx - 2.0 * a * d;
            let yr = qy - 2.0 * b * d;
            (xr, yr)
        };

        // Print R.x and R.y separated by single space and terminated by newline.
        // We use Rust's default formatting for f64, which is a reasonable
        // approximation of C's default printf("%f")/("%g") behavior.
        println!("{} {}", rx, ry);
    }
}
