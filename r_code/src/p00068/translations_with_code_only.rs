// translated from the provided C program
// Preserves semantics of floating comparisons and the scanf "%lf,%lf" behavior
// (the parser expects the comma immediately after the first number, like scanf).
// Uses f64 and large finite sentinels 1e200/-1e200 to match the C code.

use std::io::Read;
use std::str;

#[derive(Clone, Copy)]
struct XY {
    x: f64,
    y: f64,
}

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        Scanner {
            buf: input.into_bytes(),
            pos: 0,
        }
    }

    fn is_ws(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0b | 0x0c)
    }

    fn skip_ws(&mut self) {
        while self.pos < self.buf.len() && Self::is_ws(self.buf[self.pos]) {
            self.pos += 1;
        }
    }

    // Parse an integer like scanf("%d"), skipping leading whitespace.
    // Returns None on EOF or if no integer found.
    fn read_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.pos >= self.buf.len() {
            return None;
        }
        let start = self.pos;
        // optional sign
        if self.buf[self.pos] == b'+' || self.buf[self.pos] == b'-' {
            self.pos += 1;
        }
        let mut digits = 0;
        while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_ascii_digit() {
            self.pos += 1;
            digits += 1;
        }
        if digits == 0 {
            // no integer found
            return None;
        }
        let s = match str::from_utf8(&self.buf[start..self.pos]) {
            Ok(s) => s,
            Err(_) => return None,
        };
        match s.parse::<i32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    // Parse a floating number similar to strtod, starting at current pos.
    // Does NOT skip leading whitespace; caller should call skip_ws() if needed.
    // Returns Some(f64) and advances pos, or None on parse failure.
    fn parse_number(&mut self) -> Option<f64> {
        if self.pos >= self.buf.len() {
            return None;
        }
        let start = self.pos;
        // optional sign
        if self.buf[self.pos] == b'+' || self.buf[self.pos] == b'-' {
            self.pos += 1;
            if self.pos >= self.buf.len() {
                return None;
            }
        }
        let mut digits_before = 0usize;
        while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_ascii_digit() {
            self.pos += 1;
            digits_before += 1;
        }
        let mut digits_after = 0usize;
        if self.pos < self.buf.len() && self.buf[self.pos] == b'.' {
            self.pos += 1;
            while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_ascii_digit() {
                self.pos += 1;
                digits_after += 1;
            }
        }
        if digits_before == 0 && digits_after == 0 {
            return None;
        }
        // optional exponent
        if self.pos < self.buf.len() && (self.buf[self.pos] == b'e' || self.buf[self.pos] == b'E') {
            let exp_pos = self.pos;
            self.pos += 1;
            if self.pos < self.buf.len() && (self.buf[self.pos] == b'+' || self.buf[self.pos] == b'-') {
                self.pos += 1;
            }
            let mut exp_digits = 0usize;
            while self.pos < self.buf.len() && (self.buf[self.pos] as char).is_ascii_digit() {
                self.pos += 1;
                exp_digits += 1;
            }
            if exp_digits == 0 {
                // rollback to before 'e' (invalid exponent)
                self.pos = exp_pos;
            }
        }
        let s = match str::from_utf8(&self.buf[start..self.pos]) {
            Ok(s) => s,
            Err(_) => return None,
        };
        match s.parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    // Parse one point using scanf("%lf,%lf") semantics:
    // - skip leading whitespace
    // - parse first double (skipping leading whitespace)
    // - immediately expect a comma (no intervening whitespace allowed)
    // - after comma, skip leading whitespace and parse second double
    // Returns None if parsing fails or EOF is encountered.
    fn read_point(&mut self) -> Option<XY> {
        self.skip_ws();
        // parse first number
        let start_pos = self.pos;
        let x = match self.parse_number() {
            Some(v) => v,
            None => {
                // If no number at this position, mimic scanf failing by returning None.
                self.pos = start_pos;
                return None;
            }
        };
        // Next character must be a comma exactly (scanf "%lf,%lf" expects comma immediately)
        if self.pos >= self.buf.len() || self.buf[self.pos] != b',' {
            // mismatch: restore and fail
            return None;
        }
        // consume comma
        self.pos += 1;
        // skip whitespace before second number (scanf's %lf skips leading whitespace)
        self.skip_ws();
        let y = match self.parse_number() {
            Some(v) => v,
            None => return None,
        };
        Some(XY { x, y })
    }
}

fn main() {
    let mut sc = Scanner::new();

    // large finite sentinels to emulate the C code's use of 1e+200 as INF surrogates
    const POS_INF_S: f64 = 1e200;
    const NEG_INF_S: f64 = -1e200;

    loop {
        let n_opt = sc.read_i32();
        let zahyou_num = match n_opt {
            Some(v) => v as usize,
            None => break, // EOF or parse error -> exit
        };
        if zahyou_num == 0 {
            break;
        }

        // read points
        let mut zahyou: Vec<XY> = Vec::with_capacity(zahyou_num);
        for _ in 0..zahyou_num {
            match sc.read_point() {
                Some(p) => zahyou.push(p),
                None => {
                    // If parsing a point fails, mimic scanf behavior by stopping.
                    // But to preserve semantics for valid inputs, we assume input is correct.
                    return;
                }
            }
        }

        // sort by x, then y, exactly like the C comparator
        zahyou.sort_by(|a, b| {
            // Use direct comparisons so that NaN behaves similarly to C (comparisons false -> treated as equal)
            if a.x > b.x {
                std::cmp::Ordering::Greater
            } else if a.x < b.x {
                std::cmp::Ordering::Less
            } else if a.y > b.y {
                std::cmp::Ordering::Greater
            } else if a.y < b.y {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        // lower hull (th_down)
        let mut th_down: Vec<XY> = Vec::with_capacity(zahyou_num);
        // initialize with first point
        th_down.push(zahyou[0]);
        for i in 1..zahyou_num {
            // replicate the C logic: while th_down_n > 1 {...}
            while th_down.len() > 1 {
                let last = th_down[th_down.len() - 1];
                let second_last = th_down[th_down.len() - 2];
                let cur = zahyou[i];
                let k1: f64;
                if cur.x == last.x {
                    // INF cases handled with large sentinels, matching C code
                    if cur.y < last.y {
                        k1 = NEG_INF_S;
                    } else if cur.y > last.y {
                        k1 = POS_INF_S;
                    } else {
                        k1 = 0.0;
                    }
                } else {
                    k1 = (cur.y - last.y) / (cur.x - last.x);
                }
                let k2: f64;
                if last.x == second_last.x {
                    if last.y < second_last.y {
                        k2 = NEG_INF_S;
                    } else if last.y > second_last.y {
                        k2 = POS_INF_S;
                    } else {
                        k2 = 0.0;
                    }
                } else {
                    k2 = (last.y - second_last.y) / (last.x - second_last.x);
                }
                if k1 >= k2 {
                    break;
                }
                th_down.pop();
            }
            th_down.push(zahyou[i]);
        }

        // upper hull (th_up)
        let mut th_up: Vec<XY> = Vec::with_capacity(zahyou_num);
        // initialize with last point
        th_up.push(zahyou[zahyou_num - 1]);
        // iterate from zahyou_num-2 down to 0 inclusive
        if zahyou_num >= 2 {
            // i as isize for reverse iteration convenience
            let mut i: isize = (zahyou_num as isize) - 2;
            while i >= 0 {
                let idx = i as usize;
                while th_up.len() > 1 {
                    let last = th_up[th_up.len() - 1];
                    let second_last = th_up[th_up.len() - 2];
                    let cur = zahyou[idx];
                    let k1: f64;
                    if cur.x == last.x {
                        // Note the flipped signs compared to lower hull, matching C code
                        if cur.y < last.y {
                            k1 = POS_INF_S;
                        } else if cur.y > last.y {
                            k1 = NEG_INF_S;
                        } else {
                            k1 = 0.0;
                        }
                    } else {
                        k1 = (cur.y - last.y) / (cur.x - last.x);
                    }
                    let k2: f64;
                    if last.x == second_last.x {
                        if last.y < second_last.y {
                            k2 = POS_INF_S;
                        } else if last.y > second_last.y {
                            k2 = NEG_INF_S;
                        } else {
                            k2 = 0.0;
                        }
                    } else {
                        k2 = (last.y - second_last.y) / (last.x - second_last.x);
                    }
                    if k1 >= k2 {
                        break;
                    }
                    th_up.pop();
                }
                th_up.push(zahyou[idx]);
                i -= 1;
            }
        }

        let th_down_n = th_down.len();
        let th_up_n = th_up.len();
        // result: zahyou_num - (th_down_n + th_up_n - 2)
        let result = (zahyou_num as isize) - ((th_down_n + th_up_n) as isize - 2);
        println!("{}", result);
    }
}
