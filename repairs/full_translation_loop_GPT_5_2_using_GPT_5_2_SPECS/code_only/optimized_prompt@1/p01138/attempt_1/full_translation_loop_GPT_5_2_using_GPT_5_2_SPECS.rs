use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style tokenization).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn skip_ws(bytes: &[u8], idx: &mut usize) {
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
    }

    fn read_i32(bytes: &[u8], idx: &mut usize) -> Option<i32> {
        skip_ws(bytes, idx);
        if *idx >= bytes.len() {
            return None;
        }
        let mut sign: i32 = 1;
        if bytes[*idx] == b'-' {
            sign = -1;
            *idx += 1;
        }
        let mut val: i32 = 0;
        let mut any = false;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c.is_ascii_digit() {
                any = true;
                val = val.saturating_mul(10).saturating_add((c - b'0') as i32);
                *idx += 1;
            } else {
                break;
            }
        }
        if any { Some(val * sign) } else { None }
    }

    fn expect_byte(bytes: &[u8], idx: &mut usize, b: u8) {
        // The C code assumes the input matches "%d:%d:%d" exactly.
        // We mirror that by consuming the expected ':' if present; otherwise we still advance
        // only if it matches (best-effort without panicking on malformed input).
        if *idx < bytes.len() && bytes[*idx] == b {
            *idx += 1;
        }
    }

    fn read_hms(bytes: &[u8], idx: &mut usize) -> Option<(i32, i32, i32)> {
        let h = read_i32(bytes, idx)?;
        expect_byte(bytes, idx, b':');
        let m = read_i32(bytes, idx)?;
        expect_byte(bytes, idx, b':');
        let s = read_i32(bytes, idx)?;
        Some((h, m, s))
    }

    let mut out = String::new();
    let mut time = vec![0i32; 24 * 3600];

    loop {
        let n = match read_i32(bytes, &mut idx) {
            Some(v) => v,
            None => break, // no more input
        };
        if n == 0 {
            break;
        }

        // memset(time, 0, sizeof(time));
        time.fill(0);

        for _ in 0..n {
            let (hour, min, sec) = read_hms(bytes, &mut idx).unwrap_or((0, 0, 0));
            let depature = hour * 3600 + min * 60 + sec;

            let (hour2, min2, sec2) = read_hms(bytes, &mut idx).unwrap_or((0, 0, 0));
            let arrival = hour2 * 3600 + min2 * 60 + sec2;

            // for (j = depature; j < arrival; j++) time[j]++;
            // In the original C, out-of-range indices would be undefined behavior.
            // We preserve intended behavior for valid inputs; for invalid indices we clamp
            // to avoid Rust panics while keeping a reasonable approximation.
            let start = depature.max(0).min((24 * 3600) as i32);
            let end = arrival.max(0).min((24 * 3600) as i32);
            for j in start..end {
                time[j as usize] += 1;
            }
        }

        let mut maxv = 0i32;
        for &v in &time {
            if maxv < v {
                maxv = v;
            }
        }
        out.push_str(&format!("{maxv}\n"));
    }

    print!("{out}");
}
