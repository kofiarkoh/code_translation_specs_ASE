use std::io::{self, Read};

fn is_leap(y: i64) -> bool {
    (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
}

fn parse_components(token: &str) -> Vec<u64> {
    // Emulates a simple left-to-right scan splitting on '.' and accumulating base-10 digits.
    // Non-digit/non-dot characters are "unspecified" per spec; we keep a narrow interpretation:
    // treat any non-digit as a separator-like boundary (ending the current number).
    let mut comps: Vec<u64> = Vec::new();
    let mut cur: u64 = 0;
    let mut in_num = false;

    for b in token.bytes() {
        match b {
            b'0'..=b'9' => {
                in_num = true;
                cur = cur.wrapping_mul(10).wrapping_add((b - b'0') as u64);
            }
            b'.' => {
                comps.push(cur);
                cur = 0;
                in_num = false;
            }
            _ => {
                // Unspecified; end current component if we were in one.
                if in_num {
                    comps.push(cur);
                    cur = 0;
                    in_num = false;
                }
            }
        }
    }
    // Always push the last component (even if empty -> 0), matching typical dot-splitting behavior.
    comps.push(cur);
    comps
}

fn gregorian_to_mixed(y_u: u64, m_u: u64, d_u: u64) -> (u64, u64, u64, u64, u64) {
    // Month lengths for non-leap years.
    const ML: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let y = y_u as i64;
    let m = m_u as usize;
    let d = d_u;

    // dt is an unsigned 64-bit value; use wrapping arithmetic to emulate C unsigned behavior.
    let mut dt: u64 = 0;

    // days before month M in year Y: sum months 1..(M-1)
    if m >= 1 {
        for i in 1..m {
            if i <= 12 {
                dt = dt.wrapping_add(ML[i - 1]);
            }
        }
    }

    // (D - 1) as 0-based offset within month
    dt = dt.wrapping_add(d.wrapping_sub(1));

    // If M > 2 and leap year, add 1
    if m_u > 2 && is_leap(y) {
        dt = dt.wrapping_add(1);
    }

    // Year-based offset relative to 2001 using k = Y - 2001 (C integer arithmetic).
    let k: i64 = y - 2001;
    let year_off: i64 = 365 * k + (k / 4) - (k / 100) + (k / 400);
    dt = dt.wrapping_add(year_off as u64);

    // Add fixed epoch-alignment constant: 366 - m12, where m12 = 365*13 + 4 - 11.
    let m12: i64 = 365 * 13 + 4 - 11;
    let addc: i64 = 366 - m12;
    dt = dt.wrapping_add(addc as u64);

    // Decompose into mixed-radix digits (least significant first).
    let p4 = dt % 20;
    dt /= 20;
    let p3 = dt % 18;
    dt /= 18;
    let p2 = dt % 20;
    dt /= 20;
    let p1 = dt % 20;
    dt /= 20;
    let p0 = dt % 13;

    (p0, p1, p2, p3, p4)
}

fn mixed_to_gregorian(d0: u64, d1: u64, d2: u64, d3: u64, d4: u64) -> (i64, u64, u64) {
    // Combine digits with radices 13,20,20,18,20:
    // dt = ((((d0)*20 + d1)*20 + d2)*18 + d3)*20 + d4
    let mut dt: u64 = d0
        .wrapping_mul(20)
        .wrapping_add(d1)
        .wrapping_mul(20)
        .wrapping_add(d2)
        .wrapping_mul(18)
        .wrapping_add(d3)
        .wrapping_mul(20)
        .wrapping_add(d4);

    // Add fixed constant offset 4739.
    dt = dt.wrapping_add(4739);

    // Convert dt into Gregorian date.
    let mut y: i64 = 2000;
    let mut t: u64 = dt;

    // Repeatedly advance year while dt > 365.
    while dt > 365 {
        t = dt;
        dt = dt.wrapping_sub(365);
        if is_leap(y) {
            dt = dt.wrapping_sub(1);
        }
        y += 1;
    }

    // If loop ends with dt == 0, restore dt from t and decrement year.
    if dt == 0 {
        dt = t;
        y -= 1;
    }

    // Determine month by subtracting month lengths until dt fits.
    const ML: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m: u64 = 1;

    for (idx, &len) in ML.iter().enumerate() {
        if dt > len {
            dt = dt.wrapping_sub(len);
            m += 1;

            // February in leap years: when passing February, decrement dt once.
            if idx == 1 && is_leap(y) {
                dt = dt.wrapping_sub(1);
                // If that decrement makes dt become 0, set day=29 and month=February.
                if dt == 0 {
                    return (y, 2, 29);
                }
            }
        } else {
            break;
        }
    }

    // Remaining dt is 1-based day-of-month.
    (y, m, dt)
}

fn main() {
    // Read all input; tokens are whitespace-delimited.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // Stop when first character is '#'
        if tok.as_bytes().first() == Some(&b'#') {
            break;
        }

        let comps = parse_components(tok);

        if comps.len() == 3 {
            let (p0, p1, p2, p3, p4) = gregorian_to_mixed(comps[0], comps[1], comps[2]);
            out.push_str(&format!("{p0}.{p1}.{p2}.{p3}.{p4}\n"));
        } else {
            // Spec: uses first five parsed integers; fewer than five is undefined.
            // Narrowest reasonable Rust behavior: treat missing components as 0.
            let d0 = *comps.get(0).unwrap_or(&0);
            let d1 = *comps.get(1).unwrap_or(&0);
            let d2 = *comps.get(2).unwrap_or(&0);
            let d3 = *comps.get(3).unwrap_or(&0);
            let d4 = *comps.get(4).unwrap_or(&0);

            let (y, m, d) = mixed_to_gregorian(d0, d1, d2, d3, d4);
            out.push_str(&format!("{y}.{m}.{d}\n"));
        }
    }

    print!("{out}");
}
