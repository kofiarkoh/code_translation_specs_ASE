// days_diff.rs
//
// Translated from the provided C program and specification.
// This implementation calls the platform C library's mktime and difftime
// to preserve the original program's semantics (local timezone interpretation,
// normalization of out-of-range month/day values by mktime, and DST effects).
//
// Unsafe usage is limited to the FFI calls to mktime/difftime and
// working with the C `struct tm`. This is necessary to match the C behavior
// described in the SPEC exactly.

use std::io::{self, Read};
use std::mem;
use std::str::FromStr;

// Bindings to the C functions and types we need.
// We rely on the libc crate's type definitions via the `libc` module available
// in the Rust environment. These symbols are declared as extern "C" below.
extern "C" {
    // struct tm and time_t are platform-dependent; we refer to them via libc types.
    // The libc crate exposes `tm` and `time_t`, but we re-declare minimal needed parts here
    // by using the libc crate's definitions through the `libc` module.
    //
    // We declare the functions we need from the C library:
    // time_t mktime(struct tm *timeptr);
    // double difftime(time_t end, time_t beginning);
    //
    // Safety: calls are unsafe and we ensure struct tm is zero-initialized before use.
    fn mktime(t: *mut libc::tm) -> libc::time_t;
    fn difftime(t1: libc::time_t, t0: libc::time_t) -> f64;
}

fn parse_tokens_to_i32s(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|tok| i32::from_str(tok).ok())
        .collect()
}

fn main() {
    // Read entire stdin and split into integers similar to repeated scanf("%d ...").
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // As in the SPEC/C program, behavior on read error/EOF is not explicitly handled.
        // Exiting silently (no output) is acceptable per SPEC's "undefined" behavior clause.
        return;
    }

    let nums = parse_tokens_to_i32s(&input);

    // Process in chunks of six integers. If leftover tokens are present (<6), we stop.
    let mut i = 0usize;
    while i + 5 < nums.len() {
        let year1 = nums[i];
        let month1 = nums[i + 1];
        let day1 = nums[i + 2];
        let year2 = nums[i + 3];
        let month2 = nums[i + 4];
        let day2 = nums[i + 5];
        i += 6;

        // Termination: if any of the six equals -1, stop processing (do not print).
        if year1 == -1
            || month1 == -1
            || day1 == -1
            || year2 == -1
            || month2 == -1
            || day2 == -1
        {
            break;
        }

        // Prepare C struct tm, zero-initialized to match memset(&d,0,sizeof(d)) in C code.
        let mut tm_struct: libc::tm = unsafe { mem::zeroed() };

        // Set fields for the first date: tm_year = year - 1900; tm_mon = month - 1; tm_mday = day;
        tm_struct.tm_year = (year1 - 1900) as libc::c_int;
        tm_struct.tm_mon = (month1 - 1) as libc::c_int;
        tm_struct.tm_mday = day1 as libc::c_int;

        // Call mktime to normalize and obtain time_t for first date.
        // Safety: calling C library function; tm_struct is a valid pointer to initialized memory.
        let tim1 = unsafe { mktime(&mut tm_struct as *mut libc::tm) };

        // Now set fields for the second date on the same tm_struct (as in original C code).
        tm_struct.tm_year = (year2 - 1900) as libc::c_int;
        tm_struct.tm_mon = (month2 - 1) as libc::c_int;
        tm_struct.tm_mday = day2 as libc::c_int;

        let tim2 = unsafe { mktime(&mut tm_struct as *mut libc::tm) };

        // Compute difference in seconds using difftime(tim2, tim1).
        let diff_seconds = unsafe { difftime(tim2, tim1) };

        // Convert to days, add 0.5, then truncate as in the C code: (int)(diff/60/60/24+0.5)
        let days_f = diff_seconds / 60.0 / 60.0 / 24.0 + 0.5;
        let days_int = days_f as i32;

        // Print result followed by a newline, exactly one integer per line.
        println!("{}", days_int);
    }

    // Normal termination: Rust main returns with exit code 0 implicitly.
}
