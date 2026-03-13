use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

/// Parse a floating-point token from bytes starting at *pos.
/// Returns Some((value, new_pos)) if a valid float token was consumed,
/// otherwise returns None and leaves pos unchanged.
///
/// The parser accepts typical numeric formats with optional sign,
/// fractional part, and exponent (e.g. "-1.23", "4", ".5", "6e-2").
fn parse_float(bytes: &[u8], pos: usize) -> Option<(f64, usize)> {
    let len = bytes.len();
    if pos >= len {
        return None;
    }

    let mut i = pos;

    // Optional sign
    if bytes[i] == b'+' || bytes[i] == b'-' {
        i += 1;
        if i >= len {
            return None;
        }
    }

    let start_digits = i;

    // Digits before decimal point
    while i < len && (bytes[i] >= b'0' && bytes[i] <= b'9') {
        i += 1;
    }
    let digits_before = i > start_digits;

    // Fractional part
    if i < len && bytes[i] == b'.' {
        i += 1;
        let start_frac = i;
        while i < len && (bytes[i] >= b'0' && bytes[i] <= b'9') {
            i += 1;
        }
        let digits_after = i > start_frac;
        if !digits_before && !digits_after {
            // Neither side has digits -> not a valid float like "."
            return None;
        }
    } else if !digits_before {
        // No digits anywhere
        return None;
    }

    // Exponent part (optional)
    if i < len && (bytes[i] == b'e' || bytes[i] == b'E') {
        let exp_start = i;
        i += 1;
        if i < len && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let exp_digits_start = i;
        while i < len && (bytes[i] >= b'0' && bytes[i] <= b'9') {
            i += 1;
        }
        if i == exp_digits_start {
            // No exponent digits -> not a valid exponent
            return None;
        }
        // exponent consumed
    }

    // Convert the consumed slice to &str and parse f64
    let s = match std::str::from_utf8(&bytes[pos..i]) {
        Ok(ss) => ss,
        Err(_) => return None,
    };
    match s.parse::<f64>() {
        Ok(v) => Some((v, i)),
        Err(_) => None,
    }
}

/// Skip ASCII whitespace from bytes starting at pos, return new pos.
fn skip_whitespace(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r' || bytes[pos] == 0x0C || bytes[pos] == 0x0B) {
        pos += 1;
    }
    pos
}

/// Attempt to parse one point from bytes starting at cursor pos, following scanf("%lf,%lf") semantics:
/// - skip whitespace before first %lf
/// - parse first float
/// - expect a literal ',' immediately after the float (do NOT skip whitespace before comma)
/// - after comma, skip whitespace before second %lf, then parse second float
///
/// Returns Some((Point, new_pos)) on success, or None if a complete point cannot be parsed (including EOF).
fn parse_point_scanned(bytes: &[u8], mut pos: usize) -> Option<(Point, usize)> {
    // Skip whitespace before first %lf
    pos = skip_whitespace(bytes, pos);
    // Parse first float
    let (x, pos_after_x) = parse_float(bytes, pos)?;
    // After first float, the next byte must be a literal comma (no skipping whitespace)
    if pos_after_x >= bytes.len() {
        // EOF encountered where comma expected -> treat as EOF (no point)
        return None;
    }
    if bytes[pos_after_x] != b',' {
        // Literal comma not found -> parse fails
        return None;
    }
    let mut pos2 = pos_after_x + 1;
    // Skip whitespace before second %lf
    pos2 = skip_whitespace(bytes, pos2);
    // Parse second float
    let (y, pos_after_y) = parse_float(bytes, pos2)?;
    Some((Point { x, y }, pos_after_y))
}

/// Compute Euclidean distance hypot between two points
fn len(a: &Point, b: &Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

/// Heron's formula area for triangle with vertices x,y,z (returns non-negative or NaN if numeric issues)
fn area(x: &Point, y: &Point, z: &Point) -> f64 {
    let a = len(x, y);
    let b = len(y, z);
    let c = len(z, x);
    let s = (a + b + c) / 2.0;
    let prod = s * (s - a) * (s - b) * (s - c);
    prod.sqrt()
}

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, behave like C program would: exit printing 0.000000
        println!("{:.6}", 0.0);
        return;
    }
    let bytes = input.as_bytes();

    // Storage intended to hold up to 36 points.
    // We use a Vec<Point> but will not accept more than 36 stored points.
    const CAP: usize = 36;
    let mut points: Vec<Point> = Vec::with_capacity(CAP);

    // Parse sequentially following scanf("%lf,%lf") semantics as described above.
    let mut pos = 0usize;
    loop {
        match parse_point_scanned(bytes, pos) {
            Some((p, new_pos)) => {
                // Store if within capacity; if more points are present than CAP, we stop storing further points.
                // The original C code would write out-of-bounds (undefined). Here we avoid UB by stopping storage.
                if points.len() < CAP {
                    points.push(p);
                } else {
                    // We mimic the original program's lack of bounds checks by simply continuing the parsing loop
                    // but not storing the extra points (to avoid undefined behavior in safe Rust).
                }
                pos = new_pos;
            }
            None => {
                // Cannot parse another full point. Treat this as EOF/termination of input attempt.
                break;
            }
        }
    }

    // IMPORTANT: The original C program increments the input counter once extra on the final (failing) attempt,
    // producing an off-by-one effect: n = parsed_count + 1.
    // We reproduce that observable behavior by setting n accordingly.
    let parsed_count = points.len();
    let n = parsed_count + 1;

    // When computing triangles for i in 2..n, the original C code may reference an uninitialized
    // or out-of-bounds point for the final triangle. To avoid undefined behavior in Rust, we
    // substitute any missing point (index >= parsed_count) with a zero point (0.0, 0.0).
    // This choice keeps the program safe while reflecting that the final triangle may use an invalid value.
    //
    // Note: This is a safety-driven deviation from exact undefined behavior in C; it is documented here.
    let zero_point = Point { x: 0.0, y: 0.0 };

    let mut s = 0.0;
    for i in 2..n {
        let p0 = if 0 < parsed_count { points[0] } else { zero_point };
        let p1 = if (i - 1) < parsed_count {
            points[i - 1]
        } else {
            zero_point
        };
        let p2 = if i < parsed_count { points[i] } else { zero_point };
        s += area(&p0, &p1, &p2);
    }

    // Print with same formatting as printf("%f\n") -> default 6 decimal places
    println!("{:.6}", s);
}
