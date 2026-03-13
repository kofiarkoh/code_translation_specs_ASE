use std::io::{self, BufRead};

fn classify(x: f64, y: f64) -> &'static str {
    // Follow the exact ordered, strict (<) comparisons from the specification.
    if x < 35.5 && y < 71.0 {
        "AAA"
    } else if x < 37.5 && y < 77.0 {
        "AA"
    } else if x < 40.0 && y < 83.0 {
        "A"
    } else if x < 43.0 && y < 89.0 {
        "B"
    } else if x < 50.0 && y < 105.0 {
        "C"
    } else if x < 55.0 && y < 116.0 {
        "D"
    } else if x < 70.0 && y < 148.0 {
        "E"
    } else {
        "NA"
    }
}

fn main() {
    let stdin = io::stdin();
    let mut pending: Option<f64> = None;

    // Read input line by line to keep memory usage bounded.
    for line_res in stdin.lock().lines() {
        // On I/O error, stop processing (spec: input stream is available/readable;
        // behavior on errors is not explicitly defined).
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };

        for tok in line.split_whitespace() {
            // Attempt to parse each token as an f64. If parsing fails, stop processing further.
            // The specification does not require explicit error messages; behavior for malformed
            // input is not guaranteed, so we choose to stop gracefully.
            let val = match tok.parse::<f64>() {
                Ok(v) => v,
                Err(_) => return, // exit immediately; avoid panic so process exit code remains 0?
                                  // Note: returning from main exits with code 0.
            };

            if let Some(x) = pending {
                let y = val;
                println!("{}", classify(x, y));
                pending = None;
            } else {
                pending = Some(val);
            }
        }
    }

    // If input ends with an incomplete pair (pending.is_some()), we do not process it.
    // This matches the specification which does not guarantee behavior for incomplete pairs.
}
