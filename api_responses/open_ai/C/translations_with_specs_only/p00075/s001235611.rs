use std::io::{self, Read};
use std::str::FromStr;

/// This program implements the semantics described in the Specification:
/// repeatedly attempt to scan input according to the scanf format "%d,%lf,%lf".
/// The loop continues while the scanner does not report EOF. On each iteration
/// the body always executes (even if no fields were assigned). Values for
/// id/weight/height persist across iterations; they are updated only when the
/// corresponding conversion succeeds. This approximates the described C
/// behavior (including the possibility of infinite loops on malformed input).
///
/// Implementation notes / fidelity choices:
/// - The entire stdin is read into a byte buffer and a cursor `pos` is used to
///   emulate scanf's consumption and non-consumption of input on matching
///   failures (i.e., when a conversion or literal match fails, the offending
///   character is left at the current cursor, as scanf would).
/// - For numeric conversions (%d, %lf) leading whitespace is consumed even if
///   the conversion ultimately fails; this matches scanf behavior for
///   whitespace-consuming conversions.
/// - If the scanner would encounter EOF before any assignment in an attempt,
///   we treat that as EOF and terminate the loop. Otherwise we return the
///   number of assigned fields (0..3) and the caller executes the body.
/// - Variables are initialized to zero (id = 0, weight = 0.0, height = 0.0).
///   The Specification allowed "uninitialized" behavior in the original C,
///   but Rust cannot have UB; using zero is a conservative, deterministic
///   interpretation that preserves the described observable behavior pattern:
///   values persist across iterations and are used even when later scans fail.
/// - Floating parsing implements a subset of strtod's syntax sufficient for
///   typical decimal floating literals: optional sign, digits and/or a decimal
///   point, optional exponent with digits. It does not implement locale-
///   dependent forms, "inf"/"nan", or hexadecimal floats; this matches the
///   Specification's focus on standard numeric inputs like "70.0", "1.75".
fn main() {
    // Read whole stdin into buffer
    let mut buffer = Vec::new();
    // Read to end; treat read errors as empty input (no extra output)
    let _ = io::stdin().read_to_end(&mut buffer);

    let len = buffer.len();
    let mut pos: usize = 0;

    // Persistent variables across iterations. Initialized to zero as a
    // deterministic stand-in for potentially "uninitialized" C variables.
    let mut id: i32 = 0;
    let mut weight: f64 = 0.0;
    let mut height: f64 = 0.0;

    loop {
        // Attempt a scanf-like scan of "%d,%lf,%lf" starting at pos.
        // Returns:
        // - None => EOF (scanner would return EOF)
        // - Some(k) => number of assigned fields (0..3), and advances pos in buffer
        match attempt_scan(&buffer, len, &mut pos, &mut id, &mut weight, &mut height) {
            ScanResult::EOF => break,
            ScanResult::Count(_assigned) => {
                // Per Specification, the body executes regardless of how many
                // fields were assigned (as long as scanf did not return EOF).
                // Compute BMI and, if >= 25, print id with newline.
                // BMI computation: weight / (height * height)
                // Floating-point division by zero will yield inf or NaN per IEEE rules.
                let bmi = weight / (height * height);
                if bmi.is_finite() {
                    if bmi >= 25.0 {
                        println!("{}", id);
                    }
                } else {
                    // For non-finite BMI (Inf, -Inf, NaN): compare with 25.0
                    // Per IEEE, comparisons with NaN are false; Inf >= 25 -> true.
                    if bmi >= 25.0 {
                        println!("{}", id);
                    }
                    // NaN comparison yields false -> nothing printed. This matches
                    // the intent that only bmi >= 25 leads to printing.
                }
                // Continue looping until EOF as defined above.
            }
        }
    }

    // Exit normally with status 0 (implicit).
}

enum ScanResult {
    EOF,
    Count(usize),
}

/// Attempt to scan "%d,%lf,%lf" from buffer at position `pos`.
/// On success or partial success returns ScanResult::Count(k) where k is
/// number of successful assignments (0..3). `pos` is advanced to reflect
/// consumed input per scanf semantics. If EOF would be reported by scanf
/// (i.e., input exhausted and no conversions performed), returns ScanResult::EOF.
fn attempt_scan(
    buf: &[u8],
    len: usize,
    pos: &mut usize,
    id_out: &mut i32,
    weight_out: &mut f64,
    height_out: &mut f64,
) -> ScanResult {
    if *pos >= len {
        // No input available at attempt start -> EOF
        return ScanResult::EOF;
    }

    let mut assigned = 0usize;

    // Parse %d: skip leading whitespace (always consumed for numeric conversions)
    let mut p = skip_whitespace(buf, len, *pos);

    // If EOF encountered after skipping whitespace and no assignment yet, scanf would return EOF.
    if p >= len && assigned == 0 {
        *pos = p;
        return ScanResult::EOF;
    }

    // Attempt integer parse
    let (opt_int, p_after_int) = parse_int(buf, len, p);
    p = p_after_int;
    if let Some(v) = opt_int {
        *id_out = v;
        assigned += 1;
    } else {
        // No digits => conversion failed. Per scanf semantics, whitespace consumed
        // before the conversion remains consumed; the non-matching char is left.
        // We should return assigned (which is 0 here).
        *pos = p;
        return ScanResult::Count(assigned);
    }

    // Expect literal comma ','
    if p >= len {
        // EOF after some assignments -> return assigned (not EOF)
        *pos = p;
        return ScanResult::Count(assigned);
    }
    if buf[p] == b',' {
        p += 1;
    } else {
        // Literal didn't match; the offending character remains unread.
        *pos = p;
        return ScanResult::Count(assigned);
    }

    // Parse %lf weight: skip leading whitespace
    p = skip_whitespace(buf, len, p);
    if p >= len && assigned == 0 {
        *pos = p;
        return ScanResult::EOF;
    }

    let (opt_w, p_after_w) = parse_float(buf, len, p);
    p = p_after_w;
    if let Some(w) = opt_w {
        *weight_out = w;
        assigned += 1;
    } else {
        // Conversion failed; whitespace before conversion (if any) was consumed.
        *pos = p;
        return ScanResult::Count(assigned);
    }

    // Expect literal comma ','
    if p >= len {
        *pos = p;
        return ScanResult::Count(assigned);
    }
    if buf[p] == b',' {
        p += 1;
    } else {
        *pos = p;
        return ScanResult::Count(assigned);
    }

    // Parse %lf height
    p = skip_whitespace(buf, len, p);
    if p >= len && assigned == 0 {
        *pos = p;
        return ScanResult::EOF;
    }

    let (opt_h, p_after_h) = parse_float(buf, len, p);
    p = p_after_h;
    if let Some(h) = opt_h {
        *height_out = h;
        assigned += 1;
    } else {
        *pos = p;
        return ScanResult::Count(assigned);
    }

    // All three conversions done
    *pos = p;
    ScanResult::Count(assigned)
}

/// Skip ASCII whitespace starting at pos, return new position.
fn skip_whitespace(buf: &[u8], len: usize, mut pos: usize) -> usize {
    while pos < len && buf[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Attempt to parse an integer starting at pos. Leading whitespace is NOT skipped
/// here (the caller handles whitespace consumption as required). Returns
/// (Option<i32>, new_pos). If no digits found, returns (None, pos) where pos
/// is the position after having consumed any leading whitespace (if caller did).
fn parse_int(buf: &[u8], len: usize, mut pos: usize) -> (Option<i32>, usize) {
    if pos >= len {
        return (None, pos);
    }

    let start = pos;

    // optional sign
    if buf[pos] == b'+' || buf[pos] == b'-' {
        pos += 1;
        if pos >= len {
            // sign only -> no digits -> fail, leave pos at position after sign
            return (None, pos - 1); // rollback to before sign? scanf would leave sign as unread or consumed?
            // Note: In real scanf, sign is part of conversion attempt: if no digits after sign,
            // conversion fails and sign is not consumed. To better match that, we should not
            // advance past the sign when there are no digits. So we adjust below instead.
        }
    }

    // We need to check digits; but if we consumed a sign and there are no digits, we must
    // treat as fail and leave the stream at the position before the sign (i.e., the sign is not consumed).
    // To implement that precisely, implement scanning differently:
    let mut idx = start;
    // check optional sign
    if idx < len && (buf[idx] == b'+' || buf[idx] == b'-') {
        idx += 1;
    }
    let digits_start = idx;
    while idx < len && (buf[idx] as char).is_ascii_digit() {
        idx += 1;
    }
    if digits_start == idx {
        // No digits found; conversion fails; per scanf semantics the sign is not consumed.
        return (None, start);
    }
    // digits exist in [digits_start..idx); parse substring start..idx (including optional sign)
    let slice = &buf[start..idx];
    match std::str::from_utf8(slice) {
        Ok(s) => match i32::from_str(s) {
            Ok(v) => (Some(v), idx),
            Err(_) => {
                // parse overflow or other error -> treat as conversion failure,
                // do not consume input (per conservative choice) -> leave pos at start
                (None, start)
            }
        },
        Err(_) => (None, start),
    }
}

/// Attempt to parse a floating point number (simple decimal form) starting at pos.
/// Leading whitespace is NOT skipped here (caller handles whitespace). Returns
/// (Option<f64>, new_pos). If no valid float lexeme at pos, returns (None, pos).
///
/// Recognized forms: [sign] ( digits [ . digits ] | . digits ) [ exponent ]
/// exponent = [eE] [sign] digits
fn parse_float(buf: &[u8], len: usize, mut pos: usize) -> (Option<f64>, usize) {
    if pos >= len {
        return (None, pos);
    }
    let start = pos;

    // optional sign
    if buf[pos] == b'+' || buf[pos] == b'-' {
        pos += 1;
        if pos >= len {
            // sign only, no digits/decimal part -> fail and do not consume sign
            return (None, start);
        }
    }

    let mut idx = start;
    // optional sign handled in idx
    if idx < len && (buf[idx] == b'+' || buf[idx] == b'-') {
        idx += 1;
    }

    let mut digits_before = 0usize;
    while idx < len && (buf[idx] as char).is_ascii_digit() {
        idx += 1;
        digits_before += 1;
    }

    let mut digits_after = 0usize;
    if idx < len && buf[idx] == b'.' {
        idx += 1;
        let mut j = idx;
        while j < len && (buf[j] as char).is_ascii_digit() {
            j += 1;
            digits_after += 1;
        }
        idx = j;
    }

    // At least digits_before > 0 or digits_after > 0 required
    if digits_before == 0 && digits_after == 0 {
        // No floating-literal digits -> fail and do not consume optional sign
        return (None, start);
    }

    // optional exponent
    if idx < len && (buf[idx] == b'e' || buf[idx] == b'E') {
        let exp_pos = idx; // position of 'e' or 'E'
        let mut j = idx + 1;
        if j < len && (buf[j] == b'+' || buf[j] == b'-') {
            j += 1;
        }
        let digits_exp_start = j;
        while j < len && (buf[j] as char).is_ascii_digit() {
            j += 1;
        }
        if digits_exp_start == j {
            // No digits in exponent -> do not consume 'e' and following sign; exponent not part of number.
            // So leave idx as before 'e'
        } else {
            // valid exponent; include it
            idx = j;
        }
    }

    let slice = &buf[start..idx];
    match std::str::from_utf8(slice) {
        Ok(s) => match f64::from_str(s) {
            Ok(v) => (Some(v), idx),
            Err(_) => (None, start),
        },
        Err(_) => (None, start),
    }
}
