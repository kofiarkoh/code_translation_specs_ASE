// Translated from C: Aizu Vol-1 0113: Period
// Preserves the original C semantics as closely as practical in safe Rust.
// Notes:
// - Parsing emulates the original getInt which skips at most one leading space
//   and reads digits until a char < '0'.
// - Arrays sizes (rr: MAX+2, sps/hat: 100) preserved. To avoid UB/panic when
//   indices exceed those sizes we clamp prints to the buffer lengths.
// - Integer multiplication by 10 uses wrapping to emulate typical C signed
//   wraparound behavior on overflow.

use std::cmp::min;
use std::io::{self, BufRead, Write};

const MAX: usize = 1_000_000;
fn get_int(bytes: &[u8], p: &mut usize) -> i32 {
    // Emulate C getInt: if current char is a single space, skip it.
    if bytes.get(*p) == Some(&b' ') {
        *p += 1;
    }
    let mut n: i32 = 0;
    // bytes has a trailing 0 sentinel when called, so indexing is safe.
    while let Some(&ch) = bytes.get(*p) {
        if ch < b'0' {
            break;
        }
        // (ch & 0xf) gives numeric value like in original C
        n = n.wrapping_mul(10).wrapping_add((ch & 0x0f) as i32);
        *p += 1;
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut rr: Vec<i32> = vec![-1; MAX + 2]; // match C global array size
    let mut sps = vec![b' '; 100];
    let mut hat = vec![b'^'; 100];

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(s) => s,
            Err(_) => break,
        };
        // Convert to bytes and append a 0 sentinel to emulate C's '\0' terminator
        let mut bytes = line.into_bytes();
        bytes.push(0);

        let mut p: usize = 0;
        let mut a = get_int(&bytes, &mut p) as i32;
        let b = get_int(&bytes, &mut p) as i32;

        // Initialize rr[0..b] to -1 as in C (memset first b ints to -1)
        let b_usize = if b >= 0 { b as usize } else { 0 };
        if b_usize <= rr.len() {
            rr[..b_usize].fill(-1);
        } else {
            // If b is larger than rr length, fill entire rr
            rr.fill(-1);
        }

        // Emulate original algorithm
        // In C they do rr[a] = 0; assume a is non-negative and within rr bounds.
        if (a as isize) >= 0 && (a as usize) < rr.len() {
            rr[a as usize] = 0;
        } else {
            // If out of bounds, we cannot set; but continue. Original C would UB here.
        }
        let mut k: i32 = 1;

        loop {
            // a *= 10 with wrapping to emulate typical C behavior
            a = a.wrapping_mul(10);

            // digit = a / b  (C truncates toward zero; Rust does same for i32)
            let digit = if b != 0 { a / b } else { 0 };
            // Write single digit char
            let dig_byte = (b'0' as i32 + digit) as u8;
            let _ = stdout.write_all(&[dig_byte]);

            // r = a % b
            let r = if b != 0 { a % b } else { 0 };

            if r == 0 {
                let _ = stdout.write_all(b"\n");
                break;
            }
            let r_usize = if r >= 0 { r as usize } else { 0usize };
            let prev = if r_usize < rr.len() { rr[r_usize] } else { -1 };
            if prev >= 0 {
                // Found cycle
                let _ = stdout.write_all(b"\n");
                let i = prev as usize;
                let w = prev as usize;

                // Print i spaces (sps[0..i]) - clamp to sps.len()
                let sps_len = min(i, sps.len());
                if sps_len > 0 {
                    let _ = stdout.write_all(&sps[..sps_len]);
                }

                // Print hat markers for length (k - w), clamp to hat.len()
                let hat_len_calc = (k - (w as i32)) as usize;
                let hat_len = min(hat_len_calc, hat.len());
                if hat_len > 0 {
                    let _ = stdout.write_all(&hat[..hat_len]);
                }
                let _ = stdout.write_all(b"\n");
                break;
            }
            // rr[r] = k; a = r;
            if r_usize < rr.len() {
                rr[r_usize] = k;
            } else {
                // out of bounds in original C would be UB; here we skip storing
            }
            a = r;
            k = k.wrapping_add(1);
        }

        let _ = stdout.flush();
    }
}
