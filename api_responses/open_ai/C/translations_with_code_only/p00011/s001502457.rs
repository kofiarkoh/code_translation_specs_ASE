use std::io::{self, Read};

/// This Rust translation attempts to preserve the C program's semantics closely.
///
/// Notes on choices to preserve behavior:
/// - The original C uses a fixed array `l[31]` and indexes it with 1..=w, and later
///   swaps elements at indices read from input. C performs no bounds checks; out-of-bounds
///   accesses are therefore possible (undefined behavior in C). To emulate that lack of
///   bounds checks we use raw pointer arithmetic and `unsafe` dereferences. This is necessary
///   to match the original program behavior as closely as possible.
/// - Loop variables are `int` in C. To match wraparound behavior for `int` increments
///   (which in C is undefined on overflow but commonly wraps), the loops use `wrapping_add`.
/// - Input is parsed from the entire stdin buffer. The C program uses scanf formats:
///     "%d\n", "%d\n", and "%d,%d\n"
///   We emulate these formats:
///     - Each %d skips leading whitespace before the number.
///     - The comma in "%d,%d" is matched literally (no automatic skipping).
///     - The trailing `\n` in formats is treated as whitespace skipping (matches any amount
///       of white space, as scanf does for whitespace in format strings).
///
/// The code assumes well-formed input similar to what the C program expects.
fn skip_whitespace(buf: &[u8], pos: &mut usize) {
    while *pos < buf.len() && buf[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

/// Parse a signed decimal integer like scanf("%d").
/// This skips leading whitespace, accepts optional '+' or '-', then digits.
/// On failure to find digits, returns 0 (practical choice; original C would
/// leave variable unchanged on scanf failure, but typical inputs are well-formed).
fn parse_int(buf: &[u8], pos: &mut usize) -> i32 {
    // skip leading whitespace as %d does
    skip_whitespace(buf, pos);

    if *pos >= buf.len() {
        return 0;
    }

    // sign
    let mut sign: i64 = 1;
    if buf[*pos] == b'+' {
        *pos += 1;
    } else if buf[*pos] == b'-' {
        sign = -1;
        *pos += 1;
    }

    // digits
    let mut any = false;
    let mut acc: i64 = 0;
    while *pos < buf.len() {
        let b = buf[*pos];
        if b >= b'0' && b <= b'9' {
            any = true;
            acc = acc * 10 + (b - b'0') as i64;
            *pos += 1;
        } else {
            break;
        }
    }

    if !any {
        // No digits found; return 0 as a pragmatic fallback.
        return 0;
    }

    let val = sign * acc;
    // Cast to i32 (wrap if out-of-range) to emulate typical low-level behavior.
    val as i32
}

fn main() {
    // Read entire stdin to emulate scanf reading from the input stream.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let buf = input.as_bytes();
    let mut pos: usize = 0;

    // Read w with "%d\n" -> parse int then skip whitespace (the '\n' in format)
    let w = parse_int(buf, &mut pos);
    skip_whitespace(buf, &mut pos);

    // Read n with "%d\n"
    let n = parse_int(buf, &mut pos);
    skip_whitespace(buf, &mut pos);

    // Create array l[31] (indices 0..30). We'll manipulate it via raw pointers
    // to avoid bounds checks and emulate C behavior.
    let mut l: [i32; 31] = [0; 31];

    // for(i=1;i<=w;i++) { l[i]=i; }
    let mut i: i32 = 1;
    while i <= w {
        // emulate C's unchecked indexing
        let idx = i as usize;
        unsafe {
            // Using raw pointer arithmetic to mirror C's behavior (no bounds checks).
            let ptr = l.as_mut_ptr().add(idx);
            // write i (i32)
            *ptr = i;
        }
        i = i.wrapping_add(1);
    }

    // for(i=0;i<n;i++) { scanf("%d,%d\n",&a,&b); swap(&l[a],&l[b]); }
    let mut loop_i: i32 = 0;
    while loop_i < n {
        // parse a with %d (skips leading whitespace)
        let a = parse_int(buf, &mut pos);

        // Next format char is ',' literal: require exact match
        if pos < buf.len() && buf[pos] == b',' {
            pos += 1;
        } else {
            // If comma not present where expected, we proceed without advancing.
            // This mirrors that an input mismatch would cause scanf to fail; here we
            // take a pragmatic approach and continue (tests are expected to be well-formed).
        }

        // parse b with %d
        let b = parse_int(buf, &mut pos);

        // trailing '\n' in format: skip whitespace
        skip_whitespace(buf, &mut pos);

        // swap(&l[a], &l[b]) using unchecked pointer ops to emulate original.
        let a_idx = a as usize;
        let b_idx = b as usize;
        unsafe {
            let pa = l.as_mut_ptr().add(a_idx);
            let pb = l.as_mut_ptr().add(b_idx);
            let tmp = *pa;
            *pa = *pb;
            *pb = tmp;
        }

        loop_i = loop_i.wrapping_add(1);
    }

    // for(i=1;i<=w;i++){ printf("%d\n", l[i]); }
    let mut out_i: i32 = 1;
    while out_i <= w {
        let idx = out_i as usize;
        let val: i32;
        unsafe {
            let p = l.as_ptr().add(idx);
            val = *p;
        }
        // Print exactly one integer per line (matches "%d\n").
        println!("{}", val);
        out_i = out_i.wrapping_add(1);
    }
}
