use std::cell::RefCell;
use std::io::{self, Read};

thread_local! {
    // C's static double sdRate;
    static SD_RATE: RefCell<f64> = RefCell::new(0.0);
}

/// Read next line from `lines` (like fgets into a 1024-byte buffer).
/// - Returns `None` on EOF (C's fgets would return NULL; the C code doesn't check).
fn next_line(lines: &mut std::str::Lines<'_>) -> Option<String> {
    lines.next().map(|s| {
        // `fgets` keeps the trailing '\n' if present; `lines()` strips it.
        // The C code uses sscanf on the line, so newline presence doesn't matter.
        // Still, we can mimic a typical line by re-adding '\n' (harmless for sscanf-like parsing).
        let mut t = String::with_capacity(s.len() + 1);
        t.push_str(s);
        t.push('\n');
        t
    })
}

/// Equivalent to C's fGetVal(int piPos).
/// Note: This is naive recursion exactly like the C code (no memoization),
/// and can overflow the stack for large |piPos|, matching the C behavior.
fn f_get_val(pi_pos: i32) -> f64 {
    if pi_pos % 2 == 0 {
        // C: return (double)(piPos / 2) / sdRate;
        // Integer division truncates toward zero in C for C99+; Rust matches.
        let half = (pi_pos / 2) as f64;
        SD_RATE.with(|r| half / *r.borrow())
    } else {
        // C:
        // double ldVal = 1.0;
        // ldVal += fGetVal(piPos - 1) * sdRate;
        // ldVal += fGetVal(piPos + 1) * (1.0 - sdRate);
        SD_RATE.with(|r| {
            let rate = *r.borrow();
            let mut ld_val = 1.0f64;
            ld_val += f_get_val(pi_pos - 1) * rate;
            ld_val += f_get_val(pi_pos + 1) * (1.0 - rate);
            ld_val
        })
    }
}

fn main() {
    // In non-test mode, C uses stdin and prints to stdout.
    // We'll read all stdin, then parse line-by-line similarly to fgets+sscanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read position line
    let pos_line = next_line(&mut lines).unwrap_or_else(|| "\n".to_string());
    let li_pos: i32 = pos_line
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0);

    // Read rate line
    let rate_line = next_line(&mut lines).unwrap_or_else(|| "\n".to_string());
    let mut sd_rate: f64 = rate_line
        .split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0);
    sd_rate /= 100.0;

    SD_RATE.with(|r| *r.borrow_mut() = sd_rate);

    // Compute expected value
    let ld_val = f_get_val(li_pos);

    // Output: C uses sprintf("%.7lf\n", ldVal) then printf("%s", buf)
    // Rust formatting matches: 7 digits after decimal, newline.
    print!("{:.7}\n", ld_val);
}
