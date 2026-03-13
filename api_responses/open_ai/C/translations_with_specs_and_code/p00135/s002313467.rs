use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit quietly (matches unspecified behavior on malformed input).
        return;
    }

    // Tokenize by whitespace. This aligns with scanf behavior for separating tokens;
    // each time is expected in the form "%d:%d" (no spaces around the colon).
    let mut tokens = input.split_whitespace();

    // Read the count `n`
    let n_token = match tokens.next() {
        Some(t) => t,
        None => return, // missing n -> unspecified; exit quietly
    };
    let n: i64 = match n_token.parse() {
        Ok(v) => v,
        Err(_) => return, // malformed n -> unspecified; exit quietly
    };

    if n <= 0 {
        // Nothing to process (matches spec: if n <= 0, no classification lines printed)
        return;
    }

    for _ in 0..n {
        // Get next token expected to be "h:m"
        let tok = match tokens.next() {
            Some(t) => t,
            None => break, // missing time token -> unspecified; stop processing
        };

        // Split on the first ':' to obtain h and m
        let mut parts = tok.splitn(2, ':');
        let h_str = parts.next().unwrap(); // safe: splitn always yields at least one element
        let m_str = match parts.next() {
            Some(s) => s,
            None => break, // missing ':' or minute part -> unspecified; stop processing
        };

        let h: i64 = match h_str.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed hour -> unspecified; stop processing
        };
        let m: i64 = match m_str.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed minute -> unspecified; stop processing
        };

        // Compute angles using f64, mirroring the original C expressions:
        // angle = h * 360 / 12 + 360 / 12 * m / 60.0;
        // angle -= (360 / 60 * m);
        let mut angle: f64 = (h as f64) * 360.0 / 12.0 + 360.0 / 12.0 * (m as f64) / 60.0;
        angle -= 360.0 / 60.0 * (m as f64);

        // Normalize only if angle > 180 (per spec)
        if angle > 180.0 {
            angle -= 360.0;
        }

        // Classification with strict comparisons (boundaries excluded, per spec)
        if -30.0 < angle && angle < 30.0 {
            println!("alert");
        } else if -90.0 < angle && angle < 90.0 {
            println!("warning");
        } else {
            println!("safe");
        }
    }
}
