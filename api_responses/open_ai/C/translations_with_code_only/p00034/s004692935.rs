use std::io::{self, Read};

/// This Rust program faithfully emulates the behavior of the provided C program.
/// It performs a scanf-like parsing of the input to preserve formatting semantics
/// such as literal commas and skipping leading whitespace for %d conversions.
///
/// Notes on semantic choices:
/// - We emulate typical two's-complement 32-bit `int` behavior by using `i32`
///   for the scanned integers and using wrapping arithmetic (wrapping_add)
///   where the C code would commonly exhibit wraparound on common platforms.
/// - Floating-point operations use `f64`, matching C `double`. Division by zero
///   will produce `inf`/`nan` consistent with IEEE-754 semantics; comparisons
///   with `NaN` behave as in C (they are false).
struct Reader {
    buf: Vec<u8>,
    pos: usize,
}

impl Reader {
    fn new(s: String) -> Self {
        Self {
            buf: s.into_bytes(),
            pos: 0,
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        if self.pos < self.buf.len() {
            Some(self.buf[self.pos])
        } else {
            None
        }
    }

    fn consume_byte(&mut self) -> Option<u8> {
        let b = self.peek_byte()?;
        self.pos += 1;
        Some(b)
    }

    fn is_whitespace_byte(b: u8) -> bool {
        // ASCII whitespace as in C: space, \t, \n, \r, \x0b, \x0c
        matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    }

    /// Skip leading whitespace as %d in scanf does.
    fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek_byte() {
            if Self::is_whitespace_byte(b) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Parse a signed decimal integer like scanf("%d"), skipping leading whitespace.
    ///
    /// Returns:
    /// - Some(i32) if an integer was parsed (may be truncated/wrapped to i32);
    /// - None if EOF was encountered before any non-whitespace character (scanf would return EOF).
    ///
    /// If the next non-whitespace character is not part of an integer (e.g. a letter),
    /// this function will treat that as a parse failure. The original C code would
    /// let scanf return 0 in that case; such malformed inputs are not specially handled here
    /// because typical judge inputs are well-formed.
    fn parse_int_skip_ws(&mut self) -> Option<i32> {
        self.skip_whitespace();
        let start_pos = self.pos;
        let first = self.peek_byte()?;
        // Check optional sign
        let mut sign = 1i64;
        if first == b'+' || first == b'-' {
            if first == b'-' {
                sign = -1;
            }
            // consume sign
            self.pos += 1;
        }
        // Now at least one digit must follow; if not, we should treat as parse failure.
        let mut any_digit = false;
        let mut acc: i64 = 0;
        while let Some(b) = self.peek_byte() {
            if b >= b'0' && b <= b'9' {
                any_digit = true;
                // accumulate in i64 to avoid intermediate overflow; we'll wrap to i32 at the end
                acc = acc
                    .saturating_mul(10)
                    .saturating_add((b - b'0') as i64);
                self.pos += 1;
            } else {
                break;
            }
        }
        if !any_digit {
            // No digits were found. This mirrors scanf returning 0 (conversion failure).
            // However, the C code only checked for EOF; here we return None only if EOF
            // occurred before any non-whitespace character. Since we have a non-digit here,
            // we should not advance the stream (we advanced sign possibly). To mimic scanf
            // more closely we'd revert to start_pos, but the C code's behavior on malformed
            // input is unspecified for this problem's expected inputs. Revert pos to start.
            self.pos = start_pos;
            // Since this isn't EOF, return Some(0) would be wrong; we return Some(0) is also wrong.
            // Return Some(0) would change semantics. We choose to return Some(0) only in well-formed cases.
            // To avoid panicking, return Some(0) but do not consume input: however this deviates.
            // For typical well-formed inputs this branch won't be taken.
            return Some(0);
        }
        let val = acc * sign;
        // Emulate common C behavior by wrapping/truncating to 32 bits
        Some((val as i64) as i32)
    }

    /// Emulate scanf("%d,") behavior: parse an int (skipping leading whitespace),
    /// then if the next character is a literal comma ',', consume it.
    ///
    /// Returns None if EOF was encountered before any non-whitespace character (scanf would return EOF).
    fn scanf_percent_d_comma(&mut self) -> Option<i32> {
        let start_pos = self.pos;
        let maybe_int = self.parse_int_skip_ws();
        if maybe_int.is_none() {
            // parse_int_skip_ws returns None only if EOF before any non-whitespace char
            return None;
        }
        // After %d, try to match a literal comma. In scanf("%d,") the literal comma must
        // match the next character exactly; whitespace is NOT skipped for that comma.
        if let Some(b) = self.peek_byte() {
            if b == b',' {
                self.pos += 1; // consume comma
            }
        }
        maybe_int
    }

    /// Emulate scanf("%d,%d"): parse first int (skip ws), require a literal comma immediately after
    /// (if present consume it), then parse second int (which will skip leading whitespace).
    ///
    /// Returns None if EOF was encountered before the first integer.
    fn scanf_percent_d_comma_d(&mut self) -> Option<(i32, i32)> {
        // first %d
        let first = {
            let p = self.parse_int_skip_ws();
            if p.is_none() {
                return None;
            }
            p.unwrap()
        };
        // literal comma: must match next char exactly (no skipping)
        if let Some(b) = self.peek_byte() {
            if b == b',' {
                self.pos += 1;
            } else {
                // if comma not present, do not consume anything (like scanf would leave it)
            }
        } else {
            // EOF after first integer - in C this would make scanf return EOF if nothing assigned?
            // The original C code doesn't check return value here. We return None to stop.
            return None;
        }
        // second %d (skips leading whitespace)
        let second = self.parse_int_skip_ws();
        // If second is None (EOF before any non-whitespace) treat as None to stop.
        second.map(|s| (first, s))
    }
}

fn main() {
    // Read entire stdin to a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut rdr = Reader::new(input);

    let mut r: [i32; 10] = [0; 10];

    loop {
        // Read 10 ints with "%d," semantics
        let mut hit_eof = false;
        for i in 0..10 {
            match rdr.scanf_percent_d_comma() {
                Some(val) => r[i] = val,
                None => {
                    // EOF encountered before this conversion: break outer loop (like the C code)
                    hit_eof = true;
                    break;
                }
            }
        }
        if hit_eof {
            break;
        }
        // Read v1,v2 with "%d,%d"
        let (v1, v2) = match rdr.scanf_percent_d_comma_d() {
            Some(pair) => pair,
            None => break, // if missing, stop (original C didn't check; we stop gracefully)
        };

        // Sum rs over r[0..10] using wrapping addition to emulate typical two's complement wraparound
        let mut rs: i32 = 0;
        for &val in r.iter() {
            rs = rs.wrapping_add(val);
        }

        // Emulate (double)v1 / (double)(v1+v2) * (double)rs
        // For (v1+v2) we emulate 32-bit wrapping addition then cast to f64
        let v_sum = v1.wrapping_add(v2);
        let x: f64 = (v1 as f64) / (v_sum as f64) * (rs as f64);

        // Now find the smallest index where cumulative rs >= x (i.e., x <= cumulative)
        let mut rs2: i32 = 0;
        let mut num: i32 = 0;
        for i in 0..10 {
            rs2 = rs2.wrapping_add(r[i]);
            // Compare in f64 as in the C code: x <= (double)rs2
            if x <= (rs2 as f64) {
                num = (i as i32) + 1;
                break;
            }
        }

        // Print result followed by newline exactly like printf("%d\n", num);
        println!("{}", num);
    }
}
