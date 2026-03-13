// triangulation_fan.rs
//
// Translates the provided Specification into Rust while preserving the described
// observable behaviors and corner-cases as closely as reasonable in safe Rust.
//
// Notes on emulation choices (see comments in code):
// - We read entire stdin into a String and implement a simple scanner that
//   mimics the relevant aspects of scanf("%lf,%lf"):
//     * Leading whitespace is skipped before the first %lf.
//     * If no characters remain at that point, the scanner reports EOF.
//     * If a double cannot be parsed at that point, the scanner reports a 0-match
//       and does NOT consume input (this can cause an infinite loop like the C
//       program would on malformed tokens).
//     * If the first double parses but the required comma is missing, the scanner
//       reports 1 matched item and consumes only the parsed double (leaving the
//       rest). If comma is found but the second double fails, scanner returns 1
//       after consuming the comma (matching typical scanf semantics).
// - We maintain a fixed-size array of 36 points initialized to NaN. Writes past
//   index >= 36 are ignored (we cannot safely emulate memory corruption in Rust).
//   Reads for indices >= 36 or for slots never written yield (NaN, NaN), which
//   lets NaNs propagate into area computations, matching the Specification's
//   note about uninitialized/out-of-bounds data producing NaN.
// - The input counter `idx` is incremented for every *successful* parse of a
//   point. Additionally, to emulate the off-by-one described in the
//   Specification, when EOF is encountered the counter is incremented once
//   extra before stopping. During area accumulation we iterate
//   for i in 2..idx { area += area(p[0], p[i-1], p[i]) } which reproduces the
//   described behavior that the last triangle may reference an uninitialized
//   / out-of-bounds point if idx was incremented once extra.
// - Triangle area is computed via Heron's formula using distances computed
//   with hypot, and sqrt is used; NaNs may therefore appear and propagate.

use std::io::{self, Read};
use std::f64;

#[derive(Debug)]
struct Scanner {
    input: String,
    pos: usize,
}

#[derive(Debug)]
enum ScanResult {
    Eof,
    Matched0,                  // scanf would return 0, no input consumed
    Matched1(f64),             // one double parsed (or parsed + comma consumed)
    Matched2(f64, f64),        // both doubles parsed and consumed
}

impl Scanner {
    fn new(input: String) -> Self {
        Scanner { input, pos: 0 }
    }

    // Skip ASCII whitespace (spaces, tabs, newlines, etc.)
    fn skip_ws(&self, mut p: usize) -> usize {
        let bytes = self.input.as_bytes();
        while p < bytes.len() {
            let c = bytes[p];
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == 0x0b || c == 0x0c {
                p += 1;
            } else {
                break;
            }
        }
        p
    }

    // Try to parse the longest prefix starting at `start` as an f64.
    // Returns (value, new_pos) if any substring from start..end parses as f64.
    // Otherwise returns None.
    //
    // This brute-force approach tries progressively longer substrings and keeps
    // the last successful parse, emulating strtod/scanf behavior where the
    // conversion consumes the longest valid prefix.
    fn parse_double_at(&self, start: usize) -> Option<(f64, usize)> {
        let s = &self.input;
        let len = s.len();
        if start >= len {
            return None;
        }
        let mut found: Option<(f64, usize)> = None;
        // We'll try end from start+1 ..= len, and keep the last success.
        for end in start + 1..=len {
            // substring slice
            let sub = &s[start..end];
            // parse::<f64> requires the entire substring to be a valid float
            // (no trailing characters), so this tests progressively longer prefixes.
            if let Ok(v) = sub.parse::<f64>() {
                found = Some((v, end));
            }
        }
        found
    }

    // Emulate scanf("%lf,%lf") semantics relevant for this program.
    //
    // Behavior:
    // - Leading whitespace is skipped before attempting the first %lf.
    // - If EOF (no chars) at that point -> returns Eof.
    // - If first double cannot be parsed -> return Matched0 and do NOT advance pos.
    // - If first double parsed:
    //     * advance pos to end of first double
    //     * skip whitespace (we allow optional whitespace before comma here to be a bit permissive)
    //     * if at EOF or next char != ',' -> return Matched1(value) (pos points after parsed double)
    //     * else consume comma and advance pos
    //     * skip whitespace
    //     * attempt to parse second double:
    //         - if fails -> return Matched1(first_value) (pos remains after comma, i.e., comma consumed)
    //         - if succeeds -> return Matched2(first, second)
    fn scan_two_doubles(&mut self) -> ScanResult {
        let len = self.input.len();
        let start = self.skip_ws(self.pos);
        if start >= len {
            return ScanResult::Eof;
        }
        // Try parse first double at 'start'. If cannot, do not consume input.
        if let Some((v1, next1)) = self.parse_double_at(start) {
            // We consumed the first double; advance pos to next1.
            self.pos = next1;
            // Allow optional whitespace before comma (be permissive).
            let after_ws = self.skip_ws(self.pos);
            self.pos = after_ws;
            if self.pos >= len {
                // Input ended after first number: in scanf this would result in 1 matched.
                return ScanResult::Matched1(v1);
            }
            let bytes = self.input.as_bytes();
            if bytes[self.pos] != b',' {
                // Comma not found: scanf would return 1 and leave the rest untouched.
                // We've already consumed the first number; emulate that.
                return ScanResult::Matched1(v1);
            }
            // Consume comma.
            self.pos += 1;
            // Skip whitespace before second number.
            self.pos = self.skip_ws(self.pos);
            // Attempt to parse second double.
            if let Some((v2, next2)) = self.parse_double_at(self.pos) {
                // Success: consume and return both.
                self.pos = next2;
                return ScanResult::Matched2(v1, v2);
            } else {
                // Second double failed. scanf would have consumed the comma and then fail,
                // returning the number of items matched so far (1). We emulate that by
                // returning Matched1 and leaving pos where it is (after comma).
                return ScanResult::Matched1(v1);
            }
        } else {
            // No double could be parsed at start: return Matched0 and do not consume input.
            return ScanResult::Matched0;
        }
    }
}

fn triangle_area(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64)) -> f64 {
    // Compute side lengths using hypot for numerical behavior similar to C hypot.
    let a = (p0.0 - p1.0).hypot(p0.1 - p1.1);
    let b = (p1.0 - p2.0).hypot(p1.1 - p2.1);
    let c = (p2.0 - p0.0).hypot(p2.1 - p0.1);
    let s = (a + b + c) / 2.0;
    // Heron's formula; sqrt may receive a small negative due to rounding -> NaN propagation.
    let inner = s * (s - a) * (s - b) * (s - c);
    inner.sqrt()
}

fn main() {
    // Read entire stdin into a string to allow scanner to "not consume" characters on parse failures
    // (emulating scanf behavior where an unmatched input leaves characters in the stream).
    let mut input = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut input) {
        // As per Spec, assume stdin is available; on read error we still exit with 0 after printing 0.000000
        println!("{:.6}", 0.0f64);
        return;
    }

    let mut scanner = Scanner::new(input);

    // Fixed-size array for up to 36 points. Initialize with NaNs to emulate uninitialized memory
    // producing NaNs when read.
    const CAP: usize = 36;
    let nan = f64::NAN;
    let mut pts: [(f64, f64); CAP] = [(nan, nan); CAP];

    // idx is the internal read counter. We increment it for each successful parse.
    // Additionally, when EOF is encountered we increment it once extra (to emulate the
    // off-by-one described in the Specification).
    let mut idx: usize = 0;

    loop {
        match scanner.scan_two_doubles() {
            ScanResult::Eof => {
                // Emulate the off-by-one: increment idx once extra on EOF then stop reading.
                idx = idx.wrapping_add(1);
                break;
            }
            ScanResult::Matched2(x, y) => {
                // Store into pts[idx] if within capacity; else ignore write (cannot emulate UB).
                if idx < CAP {
                    pts[idx] = (x, y);
                } else {
                    // Emulate write past end by doing nothing: later reads for these indices will
                    // produce NaN via our accessor behavior.
                }
                idx = idx.wrapping_add(1);
                continue;
            }
            ScanResult::Matched1(_) => {
                // scanf returned non-EOF, non-2; per Spec the program treats this as "continue".
                // We do not increment idx and we do not advance input beyond what the scanner did,
                // which already consumed the first double or the comma as per our scaner's semantics.
                continue;
            }
            ScanResult::Matched0 => {
                // No characters consumed and non-EOF -> the original program would loop forever.
                // We emulate that by continuing the loop without advancing scanner.pos, which will
                // cause repeated Matched0 results and thus an infinite loop in presence of malformed input.
                continue;
            }
        }
    }

    // Compute S = sum_{i=2..idx-1} area(p[0], p[i-1], p[i]) using the internal counter idx.
    // As discussed in the Specification, because idx was incremented once extra on EOF,
    // this can include a triangle that references an "uninitialized" or out-of-bounds point.
    //
    // We emulate out-of-bounds/uninitialized reads by returning (NaN, NaN) for indices >= CAP.
    let get_point = |i: usize| -> (f64, f64) {
        if i < CAP {
            pts[i]
        } else {
            (f64::NAN, f64::NAN)
        }
    };

    let mut ssum: f64 = 0.0;
    // We only compute triangles if idx >= 3 conceptually; the for-range will naturally skip otherwise.
    // In C code the loop would be: for (i = 2; i < idx; ++i) sum += area(p[0], p[i-1], p[i]);
    for i in 2..idx {
        let p0 = get_point(0);
        let p1 = get_point(i - 1);
        let p2 = get_point(i);
        let area = triangle_area(p0, p1, p2);
        ssum += area;
    }

    // Print using printf("%f\n", S) semantics -> default 6 digits after decimal in C.
    println!("{:.6}", ssum);
}
