use std::io::{self, Read};

fn is_leap_year(y: i64) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

fn main() {
    // Read entire stdin, then split into whitespace-delimited tokens (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Constants exactly as in the C program/spec.
    let m12: i64 = 365 * 13 + 4 - 11;
    let month_len: [i64; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // Sentinel: stop when first character is '#'
        if tok.as_bytes().first() == Some(&b'#') {
            break;
        }

        // Parse token into dot-separated non-negative integers, scanning left-to-right.
        // Spec assumes only digits and '.'; we follow that assumption.
        let mut d: Vec<i64> = Vec::new();
        let mut n: i64 = 0;
        let mut dot_count: i32 = 0;

        for &b in tok.as_bytes() {
            if b == b'.' {
                dot_count += 1;
                d.push(n);
                n = 0;
            } else {
                n = n * 10 + (b - b'0') as i64;
            }
        }
        d.push(n);

        if dot_count == 2 {
            // Gregorian Y.M.D -> mixed radix p0..p4
            // Use i64/u64 arithmetic to mirror C's behavior for typical valid inputs.
            let y = d[0];
            let m = d[1];
            let day = d[2];

            let mut dt: i64 = 0;

            // days before month
            for i in 1..m {
                // If m is out of range, C would read out of bounds; spec says no validation.
                // We keep behavior defined by clamping to available month_len indices.
                // However, to preserve intended semantics for valid inputs, this is fine.
                if (i as usize) < month_len.len() {
                    dt += month_len[i as usize];
                }
            }

            if m > 2 && is_leap_year(y) {
                dt += 1;
            }

            dt += day - 1;

            // year offset relative to 2001
            let k = y - 2001;
            dt += 365 * k + k / 4 - k / 100 + k / 400;

            // epoch alignment constant: 366 - m12
            dt += 366 - m12;

            // Decompose into mixed radix digits (least significant first)
            let mut tmp = dt.rem_euclid(i64::MAX); // keep non-negative for typical valid inputs
            // In the original C, dt is unsigned long long; for valid dates this is non-negative.
            // We assume dt >= 0 per intended domain.
            let mut u = tmp as u64;

            let p4 = (u % 20) as u64;
            u /= 20;
            let p3 = (u % 18) as u64;
            u /= 18;
            let p2 = (u % 20) as u64;
            u /= 20;
            let p1 = (u % 20) as u64;
            u /= 20;
            let p0 = (u % 13) as u64;

            out.push_str(&format!("{p0}.{p1}.{p2}.{p3}.{p4}\n"));
        } else {
            // Mixed radix (use first five components) -> Gregorian Y.M.D
            // Spec: if fewer than five components, C has undefined behavior.
            // In Rust we must be defined; we mimic "use first five" by treating missing as 0.
            // Note: This is a mismatch with C's undefined behavior on too-few components.
            let d0 = *d.get(0).unwrap_or(&0) as u64;
            let d1 = *d.get(1).unwrap_or(&0) as u64;
            let d2 = *d.get(2).unwrap_or(&0) as u64;
            let d3 = *d.get(3).unwrap_or(&0) as u64;
            let d4 = *d.get(4).unwrap_or(&0) as u64;

            let mut dt: u64 = d0;
            dt = dt * 20 + d1;
            dt = dt * 20 + d2;
            dt = dt * 18 + d3;
            dt = dt * 20 + d4;

            dt += 4739; // 4738 + 1

            // Convert day count to Gregorian date
            let mut y: i64 = 2000;
            let mut t: u64 = 0;

            while dt > 365 {
                t = dt;
                dt -= 365;
                if is_leap_year(y) {
                    dt -= 1;
                }
                y += 1;
            }

            if dt == 0 {
                dt = t;
                y -= 1;
            }

            // Determine month/day
            let mut month: i64 = 1;
            let mut rem = dt as i64;

            for i in 1..13 {
                month = i;
                let ml = month_len[i as usize];
                if rem <= ml {
                    break;
                }
                rem -= ml;

                if i == 2 && is_leap_year(y) {
                    rem -= 1;
                    if rem == 0 {
                        rem = 29;
                        month = 2;
                        break;
                    }
                }
            }

            out.push_str(&format!("{y}.{month}.{rem}\n"));
        }
    }

    print!("{out}");
}
