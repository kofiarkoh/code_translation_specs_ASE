use std::io::{self, Read};

fn is_leap(year: i64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    // Constants as in the C code (some are unused there too; keep only what matters).
    let m12: i64 = 365 * 13 + 4 - 11; // 4738
    let month_len: [i64; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // Read all input, split by whitespace into tokens (matches scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        if tok.as_bytes().first() == Some(&b'#') {
            break;
        }

        // Parse dot-separated non-negative integers.
        // Spec says malformed input is undefined; we mimic C's assumption and unwrap.
        let parts: Vec<u64> = tok.split('.').map(|p| p.parse::<u64>().unwrap()).collect();

        if parts.len() == 3 {
            // Gregorian (year.month.day) -> mixed radix (5 parts)
            let mut year = parts[0] as i64;
            let month = parts[1] as i64;
            let day = parts[2] as i64;

            let mut dt: u64 = 0;

            // Days within year before this month
            for i in 1..month {
                dt += month_len[i as usize] as u64;
            }
            // Leap day adjustment if after Feb in leap year
            if month > 2 && is_leap(year) {
                dt += 1;
            }
            // Day-of-month (0-based)
            dt += (day as u64).wrapping_sub(1);

            // Year offset from 2001 (C mutates d[0] -= 2001)
            year -= 2001;
            // Add days for years since 2001 plus constant offset (366 - m12)
            // Note: spec precondition says years earlier than 2001 not supported (underflow in C).
            let y = year as i64;
            let add = 365i64 * y + y / 4 - y / 100 + y / 400 + 366 - m12;
            dt = dt.wrapping_add(add as u64);

            // Convert to mixed radix: bases 13,20,20,18,20 (msb->lsb)
            let a4 = (dt % 20) as u64;
            dt /= 20;
            let a3 = (dt % 18) as u64;
            dt /= 18;
            let a2 = (dt % 20) as u64;
            dt /= 20;
            let a1 = (dt % 20) as u64;
            dt /= 20;
            let a0 = (dt % 13) as u64;

            out.push_str(&format!("{a0}.{a1}.{a2}.{a3}.{a4}\n"));
        } else {
            // Mixed radix (assumed 5 parts) -> Gregorian (year.month.day)
            // Spec: if component count other than 3, treat as 5-component form.
            // The C code reads d[0..4] regardless; here we require at least 5 parts.
            let d0 = *parts.get(0).unwrap_or(&0);
            let d1 = *parts.get(1).unwrap_or(&0);
            let d2 = *parts.get(2).unwrap_or(&0);
            let d3 = *parts.get(3).unwrap_or(&0);
            let d4 = *parts.get(4).unwrap_or(&0);

            let mut dt: u64 = d0;
            dt = dt * 20 + d1;
            dt = dt * 20 + d2;
            dt = dt * 18 + d3;
            dt = dt * 20 + d4;

            // C: dt += 4738 + 1; where 4738 == m12
            dt = dt.wrapping_add((m12 + 1) as u64);

            // Convert absolute day count to Gregorian by iterating years from 2000 upward.
            // This matches the C code's linear year-advance behavior.
            let mut year: i64 = 2000;
            let mut t_prev: u64 = dt;
            while dt > 365 {
                t_prev = dt;
                dt -= 365;
                if is_leap(year) {
                    dt = dt.wrapping_sub(1);
                }
                year += 1;
            }
            if dt == 0 {
                dt = t_prev;
                year -= 1;
            }

            // Determine month/day within year
            let mut month: i64 = 1;
            while month < 13 {
                let ml = month_len[month as usize] as u64;
                if dt <= ml {
                    break;
                }
                dt -= ml;

                if month == 2 && is_leap(year) {
                    dt = dt.wrapping_sub(1);
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
                month += 1;
            }

            out.push_str(&format!("{}.{}.{}\n", year, month, dt));
        }
    }

    print!("{out}");
}
