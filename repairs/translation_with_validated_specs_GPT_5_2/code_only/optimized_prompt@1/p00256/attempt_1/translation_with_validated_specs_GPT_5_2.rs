use std::io::{self, Read};

fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    // Read entire stdin, then split by whitespace like scanf("%s", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let m12: i32 = 365 * 13 + 4 - 11;
    let m: [u64; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let bytes = tok.as_bytes();
        if !bytes.is_empty() && bytes[0] == b'#' {
            break; // matches: while(scanf("%s",s), s[0]-'#')
        }

        // Parse dot-separated decimal integers, emulating the C loop.
        let mut d: [i32; 5] = [0; 5];
        let mut c: i32 = 0;
        let mut n: i32 = 0;
        let mut r: usize = 0;

        for &ch in bytes {
            if ch == b'.' {
                c += 1;
                if r < 5 {
                    d[r] = n;
                }
                r += 1;
                n = 0;
            } else {
                // C uses signed int arithmetic; overflow would be UB in C.
                // We use wrapping ops to emulate typical two's-complement behavior.
                n = n.wrapping_mul(10);
                n = n.wrapping_add((ch - b'0') as i32);
            }
        }
        if r < 5 {
            d[r] = n;
        }

        if c == 2 {
            // Gregorian date -> custom base representation
            let year = d[0];
            let month = d[1];
            let day = d[2];

            let mut dt: u64 = 0;

            // Sum days in months before `month`
            for i in 1..month {
                if (i as usize) < m.len() {
                    dt = dt.wrapping_add(m[i as usize]);
                }
            }

            // Leap day adjustment if after Feb
            if month > 2 && is_leap(year) {
                dt = dt.wrapping_add(1);
            }

            // Day-of-month (0-based)
            dt = dt.wrapping_add((day as i64 - 1) as u64);

            // Year offset from 2001
            let yoff = year.wrapping_sub(2001);
            // dt += 365*yoff + yoff/4 - yoff/100 + yoff/400 + 366 - m12;
            // Use wrapping to mimic C's signed int behavior as closely as possible.
            let mut add: i64 = 0;
            add = add.wrapping_add(365i64.wrapping_mul(yoff as i64));
            add = add.wrapping_add((yoff / 4) as i64);
            add = add.wrapping_sub((yoff / 100) as i64);
            add = add.wrapping_add((yoff / 400) as i64);
            add = add.wrapping_add(366);
            add = add.wrapping_sub(m12 as i64);
            dt = dt.wrapping_add(add as u64);

            let mut a: [u64; 5] = [0; 5];
            a[4] = dt % 20;
            dt /= 20;
            a[3] = dt % 18;
            dt /= 18;
            a[2] = dt % 20;
            dt /= 20;
            a[1] = dt % 20;
            dt /= 20;
            a[0] = dt % 13;

            out.push_str(&format!("{}.{}.{}.{}.{}\n", a[0], a[1], a[2], a[3], a[4]));
        } else {
            // Custom base representation -> Gregorian date
            let mut dt: u64 = d[0] as u64;
            dt = dt.wrapping_mul(20).wrapping_add(d[1] as u64);
            dt = dt.wrapping_mul(20).wrapping_add(d[2] as u64);
            dt = dt.wrapping_mul(18).wrapping_add(d[3] as u64);
            dt = dt.wrapping_mul(20).wrapping_add(d[4] as u64);

            dt = dt.wrapping_add(4738 + 1);

            let mut year: i32 = 2000;
            let mut t: u64 = 0;

            while dt > 365 {
                t = dt;
                dt = dt.wrapping_sub(365);
                if is_leap(year) {
                    dt = dt.wrapping_sub(1);
                }
                year = year.wrapping_add(1);
            }

            if dt == 0 {
                dt = t;
                year = year.wrapping_sub(1);
            }

            // Determine month/day
            let mut month: i32 = 1;
            for i in 1..13 {
                if dt <= m[i as usize] {
                    month = i;
                    break;
                }
                dt = dt.wrapping_sub(m[i as usize]);
                if i == 2 && is_leap(year) {
                    dt = dt.wrapping_sub(1);
                    if dt == 0 {
                        dt = 29;
                        month = i;
                        break;
                    }
                }
                month = i + 1;
            }

            out.push_str(&format!("{}.{}.{}\n", year, month, dt));
        }
    }

    print!("{out}");
}
