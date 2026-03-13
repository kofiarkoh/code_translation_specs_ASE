extern crate libc;

use std::io::{self, Read};
use std::mem;
use std::str::FromStr;

/// This program follows the provided specification:
/// - Repeatedly read six whitespace-separated integers: year1, month1, day1, year2, month2, day2
/// - If any of the six equals -1, stop (no output for that terminating tuple) and exit 0
/// - Otherwise, build C `struct tm` values (hour/min/sec = 0, tm_isdst = -1),
///   call mktime (local time conversion), compute seconds difference via difftime,
///   compute days = int((seconds / 86400.0) + 0.5) using truncation toward zero,
///   and print that integer on its own line for each processed tuple.
///
/// Notes:
/// - Behavior on malformed input or out-of-range dates mirrors the specification: we do not try to
///   recover or enforce additional checks beyond parsing tokens. We stop processing on parse/EOF.
/// - We call C library functions `mktime` and `difftime` via `libc`. This requires the C runtime
///   timezone and normalization behavior as described.
/// - Unsafe is used only for the FFI calls and zero-initialization of `libc::tm` to match C semantics.

fn main() {
    // Read all stdin into a string, split on whitespace into tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // As per spec, undefined behavior for read errors; exit silently with 0.
        return;
    }

    // Collect tokens
    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to read six integers. If any are missing or fail to parse, stop processing.
        let mut vals = [0i64; 6];
        let mut ok = true;
        for i in 0..6 {
            match tokens.next() {
                Some(tok) => {
                    // Try parse as integer. If parse fails, stop (spec says behavior undefined; choose to stop).
                    match i64::from_str(tok) {
                        Ok(v) => vals[i] = v,
                        Err(_) => {
                            ok = false;
                            break;
                        }
                    }
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }

        // Termination: if any of the six integers equals -1, stop without printing for that tuple.
        if vals.iter().any(|&v| v == -1) {
            break;
        }

        // Convert parsed values into C `struct tm` fields.
        // Use year as given (year - 1900 for tm_year), month-1 for tm_mon, day for tm_mday.
        // Set hour/min/sec = 0 and tm_isdst = -1 to let C library determine DST.
        let year1 = vals[0];
        let month1 = vals[1];
        let day1 = vals[2];
        let year2 = vals[3];
        let month2 = vals[4];
        let day2 = vals[5];

        // Prepare libc::tm structs. We zero them to mimic typical C initialization.
        let mut tm1: libc::tm = unsafe { mem::zeroed() };
        let mut tm2: libc::tm = unsafe { mem::zeroed() };

        // Fill fields. Casts assume values fit in c_int; this matches the narrow interpretation
        // from the specification (the program expects conventional integer inputs).
        tm1.tm_sec = 0;
        tm1.tm_min = 0;
        tm1.tm_hour = 0;
        // tm_mday, tm_mon, tm_year are typically of type c_int
        tm1.tm_mday = day1 as libc::c_int;
        tm1.tm_mon = (month1 - 1) as libc::c_int;
        tm1.tm_year = (year1 - 1900) as libc::c_int;
        tm1.tm_isdst = -1; // let mktime determine DST

        tm2.tm_sec = 0;
        tm2.tm_min = 0;
        tm2.tm_hour = 0;
        tm2.tm_mday = day2 as libc::c_int;
        tm2.tm_mon = (month2 - 1) as libc::c_int;
        tm2.tm_year = (year2 - 1900) as libc::c_int;
        tm2.tm_isdst = -1;

        // Call mktime and difftime from the C library.
        // Unsafe is required for FFI.
        let t1: libc::time_t;
        let t2: libc::time_t;
        unsafe {
            // mktime may modify the tm structure (normalization) and returns a time_t.
            // We follow the specification and do not check for error sentinel values.
            t1 = libc::mktime(&mut tm1 as *mut libc::tm);
            t2 = libc::mktime(&mut tm2 as *mut libc::tm);
        }

        // difftime returns a double representing (time1 - time0) in seconds.
        let diff_seconds: f64 = unsafe { libc::difftime(t2, t1) as f64 };

        // Compute days per specification: int((diff_seconds / 86400) + 0.5)
        let days_f = diff_seconds / 86400.0_f64 + 0.5_f64;
        // Casting f64 -> i64 in Rust truncates toward zero, matching C's cast behavior.
        let days_i = days_f as i64;

        // Print the resulting integer followed by a newline.
        println!("{}", days_i);
    }

    // Exit with status 0 implicitly.
}
