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
                val = val.wrapping_mul(10).wrapping_add((c - b'0') as i32);
                *idx += 1;
            } else {
                break;
            }
        }
        if !any {
            None
        } else {
            Some(val.wrapping_mul(sign))
        }
    }

    fn expect_char(bytes: &[u8], idx: &mut usize, ch: u8) -> bool {
        if *idx < bytes.len() && bytes[*idx] == ch {
            *idx += 1;
            true
        } else {
            false
        }
    }

    fn read_time_seconds(bytes: &[u8], idx: &mut usize) -> Option<i32> {
        // Matches scanf("%d:%d:%d") behavior: leading whitespace is skipped for the first %d,
        // then literal ':' must appear between numbers.
        let hour = read_i32(bytes, idx)?;
        if !expect_char(bytes, idx, b':') {
            return None;
        }
        let min = read_i32(bytes, idx)?;
        if !expect_char(bytes, idx, b':') {
            return None;
        }
        let sec = read_i32(bytes, idx)?;
        Some(hour.wrapping_mul(3600).wrapping_add(min.wrapping_mul(60)).wrapping_add(sec))
    }

    let mut out = String::new();
    loop {
        let n = match read_i32(bytes, &mut idx) {
            Some(v) => v,
            None => break, // EOF / parse failure: C code would have undefined behavior; we stop.
        };
        if n == 0 {
            break;
        }

        let mut time = vec![0i32; 24 * 3600];

        for _ in 0..n {
            let dep = match read_time_seconds(bytes, &mut idx) {
                Some(v) => v,
                None => return,
            };
            let arr = match read_time_seconds(bytes, &mut idx) {
                Some(v) => v,
                None => return,
            };

            // C code assumes indices are within [0, 24*3600). If not, it would be UB.
            // We emulate the intended behavior by only updating within bounds.
            let start = dep.max(0) as usize;
            let end = arr.min((24 * 3600) as i32).max(0) as usize;
            for j in start..end {
                time[j] = time[j].wrapping_add(1);
            }
        }

        let mut maxv: i32 = 0;
        for &v in &time {
            if maxv < v {
                maxv = v;
            }
        }
        out.push_str(&format!("{maxv}\n"));
    }

    print!("{out}");
}
