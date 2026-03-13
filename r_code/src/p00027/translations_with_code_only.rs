use std::io::{self, Read};

/// Translated from the C function:
/// int dayOfWeek(int year, int month, int day)
fn day_of_week(mut year: i32, mut month: i32, day: i32) -> i32 {
    // Preserve the exact arithmetic and control flow from the C code.
    if month == 1 || month == 2 {
        year -= 1;
        month += 12;
    }
    // All operations are on signed 32-bit integers like typical C 'int'.
    // For the inputs used here (positive year/month/day), this mirrors C behaviour.
    (year + year / 4 - year / 100 + year / 400 + (13 * month + 8) / 5 + day) % 7
}

fn main() {
    // Day names, in the same order as the C array passed to puts(...)
    const DAYS: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Read all stdin and split on whitespace to emulate scanf("%d%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to read two tokens (m and d). If not available, stop.
        let m_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let d_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse integers. If parsing fails, stop (similar to failing scanf).
        let m: i32 = match m_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let d: i32 = match d_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Stop condition exactly as in C: if m == 0 || d == 0 break;
        if m == 0 || d == 0 {
            break;
        }

        let idx = day_of_week(2004, m, d) as usize;
        // Use println! to print the day name followed by a newline (like puts).
        println!("{}", DAYS[idx]);
    }
}
