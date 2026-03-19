use std::io::{self, Read};

fn is_leap_year(y: i64) -> bool {
    (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
}

fn month_len(y: i64, m: i64) -> i64 {
    match m {
        1 => 31,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0, // unspecified for invalid month
    }
}

// Howard Hinnant's "days from civil" algorithm (proleptic Gregorian).
// Returns days relative to 1970-01-01 (can be negative).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let mp = m + if m > 2 { -3 } else { 9 }; // March=0,...,Feb=11
    let doy = (153 * mp + 2) / 5 + (d - 1); // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = mp + if mp < 10 { 3 } else { -9 }; // [1, 12]
    let y = y + if m <= 2 { 1 } else { 0 };
    (y, m, d)
}

// Mixed-radix uses successive bases 13, 20, 20, 18, 20 (most significant to least).
// Interpret 5 components as a single day count in that mixed radix.
fn mixed_to_daycount(a: u64, b: u64, c: u64, d: u64, e: u64) -> u64 {
    let mut v = a;
    v = v.wrapping_mul(20).wrapping_add(b);
    v = v.wrapping_mul(20).wrapping_add(c);
    v = v.wrapping_mul(18).wrapping_add(d);
    v = v.wrapping_mul(20).wrapping_add(e);
    v
}

// Convert day count to 5-component mixed-radix (bases 13,20,20,18,20).
fn daycount_to_mixed(mut v: u64) -> (u64, u64, u64, u64, u64) {
    let e = v % 20;
    v /= 20;
    let d = v % 18;
    v /= 18;
    let c = v % 20;
    v /= 20;
    let b = v % 20;
    v /= 20;
    let a = v % 13;
    (a, b, c, d, e)
}

fn parse_components(token: &str) -> Vec<u64> {
    token
        .split('.')
        .map(|s| s.parse::<u64>().unwrap_or(0))
        .collect()
}

// The sample implies that 2023-10-04 maps to mixed 0.0.10.16.19, not 0.0.0.0.0.
// Therefore, the internal "absolute day" used for mixed-radix is offset by a constant K
// relative to the civil day number. We solve K from the sample mapping.
fn mixed_value(a: u64, b: u64, c: u64, d: u64, e: u64) -> u64 {
    mixed_to_daycount(a, b, c, d, e)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Calibrate offset K using the only concrete anchor implied by the sample:
    // 2023-10-04 <-> 0.0.10.16.19.
    // Let abs = days_from_civil(date). Let mixed = mixed_value(...).
    // Then internal_daycount = abs + K, and internal_daycount == mixed.
    // So K = mixed - abs.
    let anchor_abs = days_from_civil(2023, 10, 4);
    let anchor_mixed = mixed_value(0, 0, 10, 16, 19) as i64;
    let k: i64 = anchor_mixed - anchor_abs;

    for tok in input.split_whitespace() {
        if tok.as_bytes().first() == Some(&b'#') {
            break;
        }

        let parts = parse_components(tok);

        if parts.len() == 3 {
            let y = parts[0] as i64;
            let m = parts[1] as i64;
            let d = parts[2] as i64;

            let abs = days_from_civil(y, m, d);
            let internal = abs + k;

            // If internal is negative, original behavior is unspecified; preserve narrow behavior
            // by casting through two's-complement (wrapping) to u64.
            let internal_u = internal as u64;

            let (a, b, c, dd, e) = daycount_to_mixed(internal_u);
            println!("{}.{}.{}.{}.{}", a, b, c, dd, e);
        } else {
            let a = *parts.get(0).unwrap_or(&0);
            let b = *parts.get(1).unwrap_or(&0);
            let c = *parts.get(2).unwrap_or(&0);
            let d = *parts.get(3).unwrap_or(&0);
            let e = *parts.get(4).unwrap_or(&0);

            let internal = mixed_to_daycount(a, b, c, d, e) as i64;
            let abs = internal - k;

            let (y, m, dd) = civil_from_days(abs);
            println!("{}.{}.{}", y, m, dd);
        }
    }
}
