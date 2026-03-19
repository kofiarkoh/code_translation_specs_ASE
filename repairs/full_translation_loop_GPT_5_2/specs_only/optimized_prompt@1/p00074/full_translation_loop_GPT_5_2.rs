use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    loop {
        let h = match it.next() {
            Some(t) => match t.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue, // ignore non-integer token
            },
            None => break,
        };
        let m = match it.next() {
            Some(t) => match t.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue,
            },
            None => break,
        };
        let s = match it.next() {
            Some(t) => match t.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue,
            },
            None => break,
        };

        // Termination condition: -1 provided for any component.
        if h == -1 || m == -1 || s == -1 {
            break;
        }

        // Invalid values: negative (other than -1) or out of bounds => no output.
        if h < 0 || m < 0 || s < 0 || h >= 24 || m >= 60 || s >= 60 {
            continue;
        }

        let h = h as i64;
        let m = m as i64;
        let s = s as i64;

        // Remaining time until the next two-hour mark from the input time.
        // "Two-hour mark" interpreted as the next time where hour is even and minutes/seconds are 0.
        let current_sec = h * 3600 + m * 60 + s;

        let next_even_hour = if (h % 2 == 0) && m == 0 && s == 0 {
            h
        } else if h % 2 == 0 {
            h + 2
        } else {
            h + 1
        };

        let next_even_hour_mod = next_even_hour % 24;
        let target_sec = next_even_hour_mod * 3600;

        let mut remaining = target_sec - current_sec;
        if remaining < 0 {
            remaining += 24 * 3600;
        }

        let triple = remaining * 3;

        fn format_hms(total_seconds: i64) -> String {
            let hh = total_seconds / 3600;
            let mm = (total_seconds % 3600) / 60;
            let ss = total_seconds % 60;
            format!("{:02}:{:02}:{:02}", hh, mm, ss)
        }

        // Output format: each time value on its own line.
        println!("{}", format_hms(remaining));
        println!("{}", format_hms(triple));
    }
}
