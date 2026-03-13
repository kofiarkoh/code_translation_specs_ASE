use std::io::{self, Read};

/// Compute number of days since 1970-01-01 (the Unix epoch) for a given
/// Gregorian date (year, month, day).
///
/// This uses Howard Hinnant's "days_from_civil" algorithm which works for the
/// extended proleptic Gregorian calendar. It matches the normalization and
/// leap-year logic that mktime would apply to the calendar date (ignoring
/// time-zone / DST effects).
fn days_from_civil(y: i64, m: i32, d: i32) -> i64 {
    // Convert inputs into forms used by the algorithm.
    // m is 1-based (January = 1).
    let mut year = y - if m <= 2 { 1 } else { 0 };
    let month = if m > 2 { m - 3 } else { m + 9 }; // [0,11]
    // era and year of era
    let era = if year >= 0 { year / 400 } else { (year - 399) / 400 };
    let yoe = (year - era * 400) as i64; // [0,399]
    // day of year starting Mar 1 = 0
    let doy = ((153 * (month as i64) + 2) / 5) + (d as i64) - 1; // [0,365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468 // 719468 = days from 0000-03-01 to 1970-01-01
}

fn main() {
    // Read entire stdin (behavior like repeated scanf calls reading whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse all integers (ignore parse errors by skipping tokens that don't parse).
    let nums: Vec<i64> = input
        .split_whitespace()
        .filter_map(|tok| tok.parse::<i64>().ok())
        .collect();

    let mut i = 0;
    while i + 5 < nums.len() {
        let year1 = nums[i];
        let month1 = nums[i + 1] as i32;
        let day1 = nums[i + 2] as i32;
        let year2 = nums[i + 3];
        let month2 = nums[i + 4] as i32;
        let day2 = nums[i + 5] as i32;

        // Break if any of the six values is -1 (matches the C code's condition).
        if year1 == -1 || month1 == -1 || day1 == -1 || year2 == -1 || month2 == -1 || day2 == -1 {
            break;
        }

        // Compute days since epoch for each date.
        let days1 = days_from_civil(year1, month1, day1);
        let days2 = days_from_civil(year2, month2, day2);

        // Convert to seconds (same scaling as mktime + difftime with zeroed time fields,
        // but deterministic and independent of local timezone).
        let secs1 = days1 * 86_400i64;
        let secs2 = days2 * 86_400i64;

        // Compute difference in seconds as f64 and apply the same rounding as the C code:
        // printf("%d\n",(int)(diff/60/60/24+0.5));
        //
        // Note: This reproduces the C rounding behavior including its asymmetry for
        // negative values (because casting from f64 to integer truncates toward zero).
        let diff_secs_f = (secs2 - secs1) as f64;
        let days_rounded = (diff_secs_f / 86_400.0 + 0.5) as i32;

        println!("{}", days_rounded);

        i += 6;
    }
}
