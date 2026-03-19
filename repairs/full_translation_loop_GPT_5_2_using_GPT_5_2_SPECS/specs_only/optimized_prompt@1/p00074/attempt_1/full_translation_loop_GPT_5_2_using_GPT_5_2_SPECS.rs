use std::io::{self, Read};

fn format_hms(total_seconds: i64) -> String {
    // Preserve signed arithmetic and truncation toward zero like C integer division.
    let h = total_seconds / 3600;
    let m = (total_seconds / 60) % 60;
    let s = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let h = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break, // non-integer prevents reading a complete triple
            },
            None => break, // input exhausted
        };
        let m = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let s = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        if h == -1 && m == -1 && s == -1 {
            break;
        }

        // Total seconds for input time; no range validation per spec.
        let t = h * 3600 + m * 60 + s;

        // Remaining time until 2-hour mark (7200 seconds).
        let rem = 7200 - t;
        out.push_str(&format_hms(rem));
        out.push('\n');

        // Three times the remaining duration.
        let thrice = rem * 3;
        out.push_str(&format_hms(thrice));
        out.push('\n');
    }

    print!("{out}");
}
