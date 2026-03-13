use std::io::{self, Read};

const WEEKDAYS: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
];

/// Compute day of week for given (year, month, day).
/// Returns 0 = Sunday, 1 = Monday, ..., 6 = Saturday.
/// Follows the arithmetic from the original C code:
/// if month is Jan/Feb, treat as month+12 of previous year, then:
/// (year + year/4 - year/100 + year/400 + (13*month + 8)/5 + day) % 7
fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let mut y = year;
    let mut m = month;
    if m == 1 || m == 2 {
        y -= 1;
        m += 12;
    }
    let val = y + y / 4 - y / 100 + y / 400 + (13 * m + 8) / 5 + day;
    // Normalize to 0..6 like C's non-negative modulo for typical inputs.
    ((val % 7) + 7) % 7
}

fn main() {
    // Read all whitespace-separated tokens from stdin (behaves like scanf "%d%d" across whitespace/newlines).
    let mut input = String::new();
    // If read_to_string fails (I/O error), unwrap will panic; the original C code did not check scanf results,
    // so this is a reasonable (safer) choice in Rust for visibility of unexpected I/O failure.
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    loop {
        // Attempt to read two tokens. If tokens are exhausted (EOF), stop.
        let m_str = match iter.next() {
            Some(s) => s,
            None => break,
        };
        let d_str = match iter.next() {
            Some(s) => s,
            None => break,
        };

        // Parse tokens as integers. The original C used scanf without checking failures.
        // In Rust we choose to stop on parse errors (safer than undefined behavior in C).
        let m: i32 = match m_str.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let d: i32 = match d_str.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: if either is 0, stop and do not print for that pair.
        if m == 0 || d == 0 {
            break;
        }

        let w = day_of_week(2004, m, d) as usize;
        // Indexing is safe because day_of_week is normalized to 0..6.
        println!("{}", WEEKDAYS[w]);
    }
}
