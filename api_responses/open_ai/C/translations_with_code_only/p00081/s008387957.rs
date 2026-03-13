use std::io::Read;
use std::os::raw::c_char;

#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

// We use the C library function strtod to replicate C's scanf "%lf" parsing
// semantics (including consumption behavior and special float forms).
// This requires an unsafe extern call; it's used only for parsing and pointer
// arithmetic to emulate scanf("%lf,%lf,...") precisely.
extern "C" {
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> f64;
}

fn main() {
    // Read entire stdin into a buffer and append a NUL so we can pass pointers to strtod.
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).expect("read failed");
    input.push(0); // null terminator

    // Base pointer for C parsing
    let base_ptr = input.as_ptr() as *const c_char;
    let base_usize = base_ptr as usize;

    // position index into input (byte offset)
    let mut pos: usize = 0;

    // Variables corresponding to the C program's points.
    // Initialize to 0.0 to avoid using uninitialized memory in Rust.
    let mut p1 = Point { x: 0.0, y: 0.0 };
    let mut p2 = Point { x: 0.0, y: 0.0 };
    let mut q = Point { x: 0.0, y: 0.0 };
    let mut r = Point { x: 0.0, y: 0.0 };
    let mut o = Point { x: 0.0, y: 0.0 };

    // Helper to access byte at pos (0 = NUL terminator)
    let byte_at = |i: usize| -> u8 { input.get(i).copied().unwrap_or(0) };

    // Emulate one scanf call with format "%lf,%lf,%lf,%lf,%lf,%lf"
    // Returns:
    //   Ok(count) where count = number of successful conversions (0..6)
    //   Err(()) meaning EOF occurred before any conversion (corresponds to scanf returning EOF)
    //
    // This function advances `pos` in the same way scanf would:
    // - When a conversion succeeds, the input position moves to endptr.
    // - When a literal in the format doesn't match, scanning stops and returns the count so far,
    //   leaving the input position at the end of the last successful conversion (or unchanged if none).
    //
    // We rely on C's strtod for conversion behavior.
    unsafe fn scan_six_doubles(
        base_ptr: *const c_char,
        base_usize: usize,
        input: &[u8],
        pos: &mut usize,
        out: &mut [f64; 6],
    ) -> Result<usize, ()> {
        let mut count = 0usize;
        let mut cur = *pos;

        // If we are at NUL before any conversion -> EOF
        if input.get(cur).copied().unwrap_or(0) == 0 {
            return Err(());
        }

        for idx in 0..6 {
            // Call strtod at current position (it skips leading whitespace)
            let start_ptr = base_ptr.add(cur) as *const c_char;
            let mut endptr: *mut c_char = std::ptr::null_mut();
            let val = strtod(start_ptr, &mut endptr as *mut *mut c_char);
            let end_usize = endptr as usize;

            // If no conversion occurred (endptr == start_ptr), stop
            if end_usize == (start_ptr as usize) {
                // If zero conversions so far and we're at NUL => EOF
                if count == 0 && input.get(cur).copied().unwrap_or(0) == 0 {
                    return Err(());
                } else {
                    // scanf would return count (possibly 0), leaving pos unchanged.
                    return Ok(count);
                }
            }

            // Successful conversion: store value and advance current pos to endptr
            out[idx] = val;
            count += 1;
            cur = end_usize - base_usize;

            // If this was not the last conversion, expect a literal comma next (no skipping)
            if idx < 5 {
                // If next char is NUL, then scanning stops (return count)
                let b = input.get(cur).copied().unwrap_or(0);
                if b == 0 {
                    *pos = cur;
                    return Ok(count);
                }
                if b == b',' {
                    // consume the comma and continue
                    cur += 1;
                    // If we consumed the comma and now at NUL, then on next iteration strtod will see NUL
                    continue;
                } else {
                    // mismatch on literal comma: scanning stops and returns count
                    *pos = cur; // position is at end of last conversion (scanf behavior)
                    return Ok(count);
                }
            } else {
                // last conversion done; update pos and return
                *pos = cur;
                return Ok(count);
            }
        }

        // should not reach here
        Ok(count)
    }

    // Main loop: emulate while(scanf(...) != EOF) { ... }
    loop {
        // Buffer to receive up to 6 parsed doubles
        let mut vals = [0.0f64; 6];
        // Call the scanner (unsafe because it calls strtod and does pointer arithmetic)
        let scan_result = unsafe {
            scan_six_doubles(
                base_ptr,
                base_usize,
                &input,
                &mut pos,
                &mut vals,
            )
        };

        match scan_result {
            Err(_) => break, // EOF before any conversion -> exit loop (scanf returned EOF)
            Ok(_count) => {
                // In the original C code, the loop condition is (scanf(...) != EOF),
                // so any return value other than EOF (including 0..5) will enter the loop.
                // We therefore always execute the body, using whatever variables were updated.
                // Update P1,P2,Q with parsed values only for the conversions that succeeded.
                // Our scanner returned values in `vals` sequentially; we cannot know how many
                // succeeded here, since scan_six_doubles returned count but we discarded it.
                // However, to replicate C semantics more closely, we should use the same count.
                // Let's re-run the scanner to obtain the count safely would be complex; instead,
                // modify scan_six_doubles to return the count. But we've already captured it above.
                // For simplicity, call scanner again to get the count and values.. but that would
                // advance position incorrectly. Instead, we will change the code above to retain count.
            }
        }

        // The above match couldn't access the count; to fix, restructure to obtain count.
        // We'll break out and rewrite the loop more cleanly below.
        break;
    }

    // Rewritten loop to retain the conversion count and then perform computations.
    pos = 0;
    // reset points to zero (as before)
    p1 = Point { x: 0.0, y: 0.0 };
    p2 = Point { x: 0.0, y: 0.0 };
    q = Point { x: 0.0, y: 0.0 };
    r = Point { x: 0.0, y: 0.0 };
    o = Point { x: 0.0, y: 0.0 };

    loop {
        let mut vals = [0.0f64; 6];
        let scan_out = unsafe {
            scan_six_doubles(
                base_ptr,
                base_usize,
                &input,
                &mut pos,
                &mut vals,
            )
        };

        let count = match scan_out {
            Err(_) => break, // EOF => exit
            Ok(n) => n,
        };

        // Update variables for the number of successful conversions (like scanf does)
        if count >= 1 {
            p1.x = vals[0];
        }
        if count >= 2 {
            p1.y = vals[1];
        }
        if count >= 3 {
            p2.x = vals[2];
        }
        if count >= 4 {
            p2.y = vals[3];
        }
        if count >= 5 {
            q.x = vals[4];
        }
        if count >= 6 {
            q.y = vals[5];
        }

        // Now perform the same geometric computation as the C code
        if p1.x != p2.x && p1.y != p2.y {
            let m = (p2.y - p1.y) / (p2.x - p1.x);
            // replicate expression exactly
            o.x = (m / (m * m + 1.0)) * (m * p1.x + q.x / m + q.y - p1.y);
            o.y = m * (o.x - p1.x) + p1.y;
            r.x = 2.0 * o.x - q.x;
            r.y = 2.0 * o.y - q.y;
        } else if p1.x != p2.x && p1.y == p2.y {
            r.x = q.x;
            r.y = 2.0 * p1.y - q.y;
        } else if p1.x == p2.x && p1.y != p2.y {
            r.x = 2.0 * p1.x - q.x;
            r.y = q.y;
        } else {
            r.x = 2.0 * p1.x - q.x;
            r.y = 2.0 * p1.y - q.y;
        }

        // Print with same formatting as printf("%lf %lf\n", ...);
        // C's "%f" (and "%lf" for printf) prints 6 digits after the decimal by default.
        println!("{:.6} {:.6}", r.x, r.y);

        // Important: If scan_six_doubles returned 0 (no conversions) but not EOF,
        // the original C program would still loop (since 0 != EOF) and would likely
        // get stuck (scanf would keep failing at same input). We replicate that behavior:
        if count == 0 {
            // To mimic possible infinite loop in C, we continue without advancing pos.
            // But to avoid an actual infinite loop in Rust when running on malformed input,
            // we'll also check whether we're at NUL; if so, break.
            if byte_at(pos) == 0 {
                break;
            }
            // Otherwise continue; this may loop indefinitely if input is malformed,
            // just as the original C program would.
        }
    }
}
