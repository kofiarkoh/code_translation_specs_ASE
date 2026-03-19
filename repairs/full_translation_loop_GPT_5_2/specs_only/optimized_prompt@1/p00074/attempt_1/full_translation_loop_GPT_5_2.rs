use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    loop {
        let h = match it.next() {
            Some(t) => match t.parse::<i32>() {
                Ok(v) => v,
                Err(_) => continue, // ignore non-integer token, keep waiting for valid input
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

        // Remaining time until the next two-hour mark from the input time.
        // Interpret "two-hour mark" as the next time where hour is even and minutes/seconds are 0.
        // If already exactly on such a mark, remaining time is 0.
        let h_u = h as i64;
        let m_u = m as i64;
        let s_u = s as i64;

        let current_sec = h_u * 3600 + m_u * 60 + s_u;

        let next_even_hour = if (h_u % 2 == 0) && m_u == 0 && s_u == 0 {
            h_u
        } else if h_u % 2 == 0 {
            h_u + 2
        } else {
            h_u + 1
        };

        // Wrap within 24 hours.
        let next_even_hour_mod = next_even_hour % 24;
        let target_sec = next_even_hour_mod * 3600;

        let mut remaining = target_sec - current_sec;
        if remaining < 0 {
            remaining += 24 * 3600;
        }

        let triple = remaining * 3;

        fn format_hms(total_seconds: i64) -> String {
            let h = total_seconds / 3600;
            let m = (total_seconds % 3600) / 60;
            let s = total_seconds % 60;
            format!("{:02}:{:02}:{:02}", h, m, s)
        }

        println!("{} {}", format_hms(remaining), format_hms(triple));
    }
}
