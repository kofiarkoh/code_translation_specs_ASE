use std::io::{self, Read};

fn main() {
    // Read all input from stdin into a string. This mirrors typical
    // scanf-based behavior where whitespace-separated tokens are consumed.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit normally with code 0 per specification (no diagnostics).
        return;
    }

    // Iterate over whitespace-separated tokens.
    let mut iter = input.split_whitespace();

    loop {
        // Collect exactly 8 f64 values for one record.
        let mut vals = [0f64; 8];
        let mut i = 0usize;
        while i < 8 {
            match iter.next() {
                Some(tok) => {
                    // Try to parse token as f64. If parsing fails, mimic scanf behavior:
                    // stop processing further records and exit normally.
                    match tok.parse::<f64>() {
                        Ok(v) => {
                            vals[i] = v;
                            i += 1;
                        }
                        Err(_) => {
                            // Malformed token: stop processing (no further output).
                            return;
                        }
                    }
                }
                None => {
                    // EOF before completing a full record: exit normally.
                    return;
                }
            }
        }

        // Unpack values: x0, y0, x1, y1, x2, y2, x3, y3
        let x0 = vals[0];
        let y0 = vals[1];
        let x1 = vals[2];
        let y1 = vals[3];
        let x2 = vals[4];
        let y2 = vals[5];
        let x3 = vals[6];
        let y3 = vals[7];

        // Inclusive interval checks: b <= a && a <= c
        let in_x2 = x0 <= x2 && x2 <= x1;
        let in_x3 = x0 <= x3 && x3 <= x1;
        let in_y2 = y0 <= y2 && y2 <= y1;
        let in_y3 = y0 <= y3 && y3 <= y1;

        let predicate = (in_x2 && (in_y2 || in_y3)) || (in_x3 && (in_y2 || in_y3));

        if predicate {
            // Exactly one of "YES\n" or "NO\n" is printed per record.
            print!("YES\n");
        } else {
            print!("NO\n");
        }
    }
}
