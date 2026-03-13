use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (spec assumes well-formed input; behavior on errors is unspecified).
        return;
    }

    if input.is_empty() {
        return;
    }

    // Split on literal commas to mirror the scanf format that expects commas between numbers.
    let tokens: Vec<&str> = input.split(',').collect();

    // We process only complete records of 6 tokens each. If there's an incomplete tail, stop.
    let full_groups = tokens.len() / 6;

    // Helper to parse a token into f64 with rules that emulate scanf("%lf," ...):
    // - Leading whitespace is allowed (scanf's %lf skips leading whitespace).
    // - For tokens that are not the last field of a record (i.e., followed by a comma),
    //   trailing or internal whitespace is not allowed (scanf would require the comma to immediately follow the number).
    // - For the last token in a record, trailing whitespace is allowed (scanf does not require a following literal).
    //
    // If parsing fails, we stop processing further records (spec assumes well-formed input).
    for gi in 0..full_groups {
        let mut vals = [0f64; 6];
        let mut ok = true;
        for j in 0..6 {
            let tok = tokens[gi * 6 + j];
            // Allow leading whitespace
            let s_leading_stripped = tok.trim_start();
            if j != 5 {
                // For fields followed by a comma, there must be no whitespace remaining
                // (i.e., no trailing or internal whitespace). This mirrors scanf's requirement
                // that the comma immediately follows the numeric token.
                if s_leading_stripped.chars().any(|c| c.is_whitespace()) {
                    ok = false;
                    break;
                }
                match s_leading_stripped.parse::<f64>() {
                    Ok(v) => vals[j] = v,
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            } else {
                // Last token in the group: allow trailing whitespace (e.g., newline at end of file)
                let s_trimmed = s_leading_stripped.trim_end();
                if s_trimmed.is_empty() {
                    ok = false;
                    break;
                }
                match s_trimmed.parse::<f64>() {
                    Ok(v) => vals[j] = v,
                    Err(_) => {
                        ok = false;
                        break;
                    }
                }
            }
        }

        if !ok {
            // Stop processing further records on parse failure (behavior for malformed input is unspecified).
            break;
        }

        // Unpack values into points
        let p1x = vals[0];
        let p1y = vals[1];
        let p2x = vals[2];
        let p2y = vals[3];
        let qx = vals[4];
        let qy = vals[5];

        let (rx, ry) = if p1x != p2x && p1y != p2y {
            // General case: line with non-zero finite slope (neither vertical nor horizontal).
            let m = (p2y - p1y) / (p2x - p1x);
            // Compute foot O of perpendicular from Q onto line P1-P2, following original formula.
            let denom = m * m + 1.0;
            let ox = (m / denom) * (m * p1x + qx / m + qy - p1y);
            let oy = m * (ox - p1x) + p1y;
            (2.0 * ox - qx, 2.0 * oy - qy)
        } else if p1x != p2x && p1y == p2y {
            // Horizontal line y = p1y
            (qx, 2.0 * p1y - qy)
        } else if p1x == p2x && p1y != p2y {
            // Vertical line x = p1x
            (2.0 * p1x - qx, qy)
        } else {
            // Degenerate case: P1 == P2, reflect Q about this single point
            (2.0 * p1x - qx, 2.0 * p1y - qy)
        };

        // Print with the same default precision as printf("%lf %lf\n") which prints with 6 decimals.
        // Each output line: "<R.x> <R.y>\n"
        println!("{:.6} {:.6}", rx, ry);
    }
}
