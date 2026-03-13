use std::io::{self, Read};

fn pad_min_width_2(n: i128) -> String {
    // Convert number (with sign if negative) to string, and ensure at least width 2 by
    // left-padding with '0' when necessary. This follows the specification:
    // - print the full numeric representation (including sign) if it requires more digits;
    // - otherwise ensure minimum width of 2 using zero-padding.
    let s = n.to_string();
    if s.len() >= 2 {
        s
    } else {
        // s.len() is 0 or 1; for integers it's 1 (e.g., "0".."9"), so pad to "0X".
        format!("0{}", s)
    }
}

fn main() {
    // Read all standard input as whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, exit (behavior on malformed/partial input is unspecified per spec).
        return;
    }

    let mut iter = input.split_whitespace();

    loop {
        // Attempt to read three tokens. If any are missing or fail to parse as integers,
        // we stop. The Specification leaves behavior unspecified for malformed/partial input;
        // here we choose to terminate quietly.
        let t_tok = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let h_tok = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let s_tok = match iter.next() {
            Some(s) => s,
            None => break,
        };

        // Parse as signed integers. Use i128 for arithmetic to avoid inadvertent overflow
        // that would be undefined in C; this preserves division/remainder semantics.
        let t = match t_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let h = match h_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let s = match s_tok.parse::<i128>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Check sentinel (-1, -1, -1)
        if t == -1 && h == -1 && s == -1 {
            // Normal termination per specification: exit with status 0 (implicit).
            break;
        }

        // Compute input_seconds = t*3600 + h*60 + s
        let input_seconds = t * 3600 + h * 60 + s;

        // seconds1 = 7200 - input_seconds
        let seconds1 = 7200i128 - input_seconds;

        // seconds2 = 7200 - (input_seconds / 3), where division truncates toward zero.
        let seconds2 = 7200i128 - (input_seconds / 3);

        // Map seconds value S to HH:MM:SS as specified:
        // HH = S / 3600
        // MM = (S % 3600) / 60
        // SS = S % 60
        // Use i128 arithmetic; Rust's division and remainder for signed integers
        // truncate toward zero and remainder has sign of dividend, matching C semantics.
        for &sec in &[seconds1, seconds2] {
            let hh = sec / 3600;
            let mm = (sec % 3600) / 60;
            let ss = sec % 60;

            let hh_s = pad_min_width_2(hh);
            let mm_s = pad_min_width_2(mm);
            let ss_s = pad_min_width_2(ss);

            println!("{}:{}:{}", hh_s, mm_s, ss_s);
        }
    }
}
