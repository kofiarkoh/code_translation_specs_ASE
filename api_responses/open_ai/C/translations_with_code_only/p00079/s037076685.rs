// main.rs
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

struct Scanner {
    s: String,
    pos: usize,
}

impl Scanner {
    fn new(s: String) -> Self {
        Scanner { s, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.s.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.s[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        if self.pos >= self.s.len() {
            None
        } else {
            let ch = self.s[self.pos..].chars().next().unwrap();
            self.pos += ch.len_utf8();
            Some(ch)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.bump_char();
            } else {
                break;
            }
        }
    }

    // Try to parse a floating point number starting at current pos.
    // On success returns Some((value, new_pos)).
    // On failure returns None and does not advance pos.
    fn parse_float_at(&self, start_pos: usize) -> Option<(f64, usize)> {
        let s = &self.s[start_pos..];
        let mut i = 0;
        let mut chars = s.chars();
        // optional sign
        if let Some('+') | Some('-') = chars.clone().next() {
            i += chars.next().unwrap().len_utf8();
        }
        let mut has_digits = false;
        // digits before decimal
        while let Some(ch) = chars.clone().next() {
            if ch.is_ascii_digit() {
                has_digits = true;
                i += ch.len_utf8();
                chars.next();
            } else {
                break;
            }
        }
        // fractional part
        if let Some('.') = chars.clone().next() {
            i += 1;
            chars.next();
            let mut frac_digits = false;
            while let Some(ch) = chars.clone().next() {
                if ch.is_ascii_digit() {
                    frac_digits = true;
                    i += ch.len_utf8();
                    chars.next();
                } else {
                    break;
                }
            }
            has_digits = has_digits || frac_digits;
        }
        if !has_digits {
            return None;
        }
        // exponent
        if let Some(ch) = chars.clone().next() {
            if ch == 'e' || ch == 'E' {
                i += 1;
                chars.next();
                if let Some(signch) = chars.clone().next() {
                    if signch == '+' || signch == '-' {
                        i += signch.len_utf8();
                        chars.next();
                    }
                }
                let mut exp_digits = false;
                while let Some(dch) = chars.clone().next() {
                    if dch.is_ascii_digit() {
                        exp_digits = true;
                        i += dch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !exp_digits {
                    // malformed exponent -> treat as no exponent (so stop before 'e')
                    // so we should backtrack to before 'e'
                    // find position of 'e' relative to start_pos
                    let consumed = &s[..i];
                    // find last occurrence of 'e' or 'E' in consumed
                    if let Some(e_idx) = consumed.rfind(|c| c == 'e' || c == 'E') {
                        i = e_idx; // cut off exponent
                    }
                }
            }
        }
        let end_pos = start_pos + i;
        // Convert substring to f64
        // Note: from_str will accept the substring if it's a valid float representation.
        match f64::from_str(&self.s[start_pos..end_pos]) {
            Ok(v) => Some((v, end_pos)),
            Err(_) => None,
        }
    }

    // This function emulates (enough for our use) the C scanf("%lf,%lf", &p->x, &p->y).
    // Return value:
    //  - -1 to indicate EOF encountered before any conversion (matches scanf EOF)
    //  - otherwise returns number of successfully assigned items (0,1,2).
    //
    // The implementation focuses on correctly parsing well-formed inputs like "x,y"
    // as produced by typical uses. It attempts to follow scanf behavior regarding EOF:
    // if stream is exhausted before any conversion, return -1.
    fn input_point(&mut self, p: &mut Point) -> i32 {
        // Save original pos to allow non-destructive failures where appropriate.
        // Step 1: skip leading whitespace
        self.skip_whitespace();
        if self.is_eof() {
            // EOF before any conversion
            return -1;
        }
        let pos_before_first = self.pos;
        // Try parse first float
        // We skip whitespace already, so parse at current pos
        match self.parse_float_at(self.pos) {
            Some((v1, newpos1)) => {
                // consume parsed float
                self.pos = newpos1;
                // Now expect a literal comma immediately next (no skipping)
                if self.is_eof() {
                    // EOF after first conversion: scanf would return 1 (not EOF)
                    p.x = v1;
                    return 1;
                }
                // peek next char
                if let Some(ch) = self.peek_char() {
                    if ch == ',' {
                        // consume comma
                        self.bump_char();
                        // Now parse second float (skip no whitespace per scanf? %lf skips leading whitespace)
                        // In scanf, %lf does skip leading whitespace. So skip whitespace here.
                        self.skip_whitespace();
                        if self.is_eof() {
                            // EOF after comma without second number: scanf returns 1
                            p.x = v1;
                            return 1;
                        }
                        match self.parse_float_at(self.pos) {
                            Some((v2, newpos2)) => {
                                self.pos = newpos2;
                                p.x = v1;
                                p.y = v2;
                                return 2;
                            }
                            None => {
                                // failed to parse second float: scanf would return 1
                                p.x = v1;
                                return 1;
                            }
                        }
                    } else {
                        // literal comma didn't match: scanf would stop matching here and return 1
                        // It would also leave the non-matching character unread; to avoid
                        // an infinite loop on malformed input we do NOT consume it here,
                        // mirroring scanf's behavior as far as return value (1).
                        p.x = v1;
                        return 1;
                    }
                } else {
                    // shouldn't reach: handled is_eof earlier
                    p.x = v1;
                    return 1;
                }
            }
            None => {
                // Couldn't parse first float.
                // If at EOF (no characters), return -1; otherwise return 0 (no assignments).
                if self.is_eof() {
                    -1
                } else {
                    // In C, scanf on matching failure leaves the offending character unread and returns 0.
                    // To avoid potential infinite loops on malformed input, advance by one character here.
                    // This is a pragmatic choice: typical correct inputs won't hit this branch.
                    self.bump_char();
                    0
                }
            }
        }
    }
}

fn len(a: &Point, b: &Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

fn area(x: &Point, y: &Point, z: &Point) -> f64 {
    let a = len(x, y);
    let b = len(y, z);
    let c = len(z, x);
    let s = (a + b + c) / 2.0;
    // Mirror C's behavior: use sqrt of potentially negative small numbers -> NaN if negative.
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    // Read all stdin into a string for sequential parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut scanner = Scanner::new(input);

    // Mirror C: fixed-size array of 36 points (initialized to zeros here).
    let mut v: [Point; 36] = [Point::default(); 36];

    let mut n: usize = 0;

    // The original C loop was: n = 0; while (~input_point(v + n++));
    // The loop continues while input_point(...) != EOF (-1). Note that n is incremented
    // on every call, even when EOF is returned (because of the post-increment in the argument).
    //
    // We emulate that: call input_point with v[n] (if n < 36 we write into array),
    // then increment n, and break if input_point returned -1.
    loop {
        // Prepare a temporary point to pass when n >= 36 to avoid out-of-bounds write.
        // The original C would write out-of-bounds if more than 36 inputs were provided;
        // here we avoid UB by not writing beyond the array. This preserves behavior for
        // typical inputs <= 36.
        let ret;
        if n < 36 {
            ret = scanner.input_point(&mut v[n]);
        } else {
            // Provide a dummy Point to fill; this mirrors that program still advances n
            // but avoids memory corruption in Rust.
            let mut tmp = Point::default();
            ret = scanner.input_point(&mut tmp);
        }
        n = n.wrapping_add(1);
        if ret == -1 {
            break;
        }
        // continue otherwise
    }

    let mut S: f64 = 0.0;
    // The C code then did: for (i = 2; i < n; i++) S += area(v, v + i - 1, v + i);
    // Note that due to the post-increment in the read loop, n in C ends up being
    // number_of_successful_reads + 1. We replicate the same bounds here.
    // We must be careful with indexing to avoid panics: only perform the area addition
    // when all three indices are within 0..36. This avoids undefined behavior while
    // preserving the intended iterative pattern for typical inputs.
    let n_usize = n;
    let start = 2usize;
    let end = n_usize; // i < n
    for i in start..end {
        // indices used: 0, i-1, i
        if i < 36 && (i - 1) < 36 {
            S += area(&v[0], &v[i - 1], &v[i]);
        } else {
            // If indices would go out of bounds, the original C program would exhibit UB.
            // We avoid panic and stop accumulating further to keep program robust.
            break;
        }
    }

    // Print with the same formatting as printf("%f\n", S) -> default 6 decimal places.
    println!("{:.6}", S);
}
