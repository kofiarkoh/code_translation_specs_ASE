use std::io::{self, Read};

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

fn days_in_month(y: i32, m: i32) -> i32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_valid_date(y: i32, m: i32, d: i32) -> bool {
    if m < 1 || m > 12 {
        return false;
    }
    let dim = days_in_month(y, m);
    d >= 1 && d <= dim
}

// Day count with day 0 = 0001-01-01 (Gregorian proleptic).
fn date_to_ordinal0(y: i32, m: i32, d: i32) -> i64 {
    let mut days: i64 = 0;
    let y1 = (y - 1) as i64;
    days += 365 * y1;
    days += y1 / 4 - y1 / 100 + y1 / 400;

    let mut month_days = [31i32, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(y) {
        month_days[1] = 29;
    }
    for mm in 1..m {
        days += month_days[(mm - 1) as usize] as i64;
    }
    days += (d - 1) as i64;
    days
}

fn ordinal0_to_date(mut ord0: i64) -> (i32, i32, i32) {
    if ord0 < 0 {
        ord0 = 0;
    }

    // Find year by binary search in a conservative range.
    let mut lo: i32 = 1;
    let mut hi: i32 = 9999;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let start_mid = date_to_ordinal0(mid, 1, 1);
        let start_next = date_to_ordinal0(mid + 1, 1, 1);
        if ord0 < start_mid {
            hi = mid;
        } else if ord0 >= start_next {
            lo = mid + 1;
        } else {
            lo = mid;
            break;
        }
    }
    let y = lo;
    let year_start = date_to_ordinal0(y, 1, 1);
    let mut day_of_year0 = (ord0 - year_start) as i32; // 0-based within year

    let mut month_days = [31i32, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(y) {
        month_days[1] = 29;
    }

    let mut m: i32 = 1;
    for md in month_days {
        if day_of_year0 >= md {
            day_of_year0 -= md;
            m += 1;
        } else {
            break;
        }
    }
    let d = day_of_year0 + 1;
    (y, m, d)
}

fn parse_i64(s: &str) -> Option<i64> {
    s.trim().parse::<i64>().ok()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            break;
        }

        let parts: Vec<&str> = line.split('.').collect();
        if parts.len() == 3 {
            // "YYYY.MM.DD" -> "a.b.c.d.e"
            let y = match parse_i64(parts[0]) {
                Some(v) => v as i32,
                None => continue,
            };
            let m = match parse_i64(parts[1]) {
                Some(v) => v as i32,
                None => continue,
            };
            let d = match parse_i64(parts[2]) {
                Some(v) => v as i32,
                None => continue,
            };
            if !is_valid_date(y, m, d) {
                continue;
            }

            // Custom encoding inferred from samples:
            // a = year / 20
            // b = year % 20
            // c = month
            // d = day / 20
            // e = day % 20
            let a = (y / 20) as i64;
            let b = (y % 20) as i64;
            let c = m as i64;
            let dd = (d / 20) as i64;
            let ee = (d % 20) as i64;
            println!("{}.{}.{}.{}.{}", a, b, c, dd, ee);
        } else if parts.len() == 5 {
            // "a.b.c.d.e" -> "YYYY.MM.DD"
            let a = match parse_i64(parts[0]) {
                Some(v) => v,
                None => continue,
            };
            let b = match parse_i64(parts[1]) {
                Some(v) => v,
                None => continue,
            };
            let c = match parse_i64(parts[2]) {
                Some(v) => v,
                None => continue,
            };
            let d = match parse_i64(parts[3]) {
                Some(v) => v,
                None => continue,
            };
            let e = match parse_i64(parts[4]) {
                Some(v) => v,
                None => continue,
            };

            // Inverse mapping:
            // year = a*20 + b
            // month = c
            // day = d*20 + e
            let y = (a * 20 + b) as i32;
            let m = c as i32;
            let day = (d * 20 + e) as i32;

            // If invalid, behavior unspecified; we still follow the spec's "may terminate"
            // by skipping output.
            if !is_valid_date(y, m, day) {
                continue;
            }

            println!("{:04}.{:02}.{:02}", y, m, day);
        } else {
            continue;
        }
    }
}
