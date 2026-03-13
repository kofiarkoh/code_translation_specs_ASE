// translated_from_c.rs
//
// Faithful translation of the provided C program.
// Notes:
// - We call the C scanf function directly (unsafe) to preserve the original
//   scanf("%f", &v) semantics, including its return values (EOF = -1, or 0
//   on no-match which would cause the same behavior as the C program).
// - Floating-point values are stored as f32 to match C's `float` and we use
//   f32 arithmetic and ceil() to emulate ceilf.
//
// Unsafe usage justification:
// - Calling the C variadic function `scanf` requires an unsafe extern call.
//   This is necessary to preserve the exact I/O/loop semantics of the original C code.

use std::os::raw::{c_char, c_int};

extern "C" {
    // variadic scanf from libc
    // signature: int scanf(const char *format, ...);
    fn scanf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    // match C's EOF macro value
    const EOF: c_int = -1;

    let mut v: f32 = 0.0;

    // format string as a nul-terminated C string literal (bytes)
    // "%f" matches the original C code.
    let fmt = b"%f\0";

    // Loop condition mirrors: while (scanf("%f", &v) != EOF) { ... }
    // Note: if scanf returns 0 (no conversion), this loop will not exit,
    // just like the original C program.
    unsafe {
        while scanf(fmt.as_ptr() as *const c_char, &mut v as *mut f32) != EOF {
            let t: f32 = v / 9.8_f32;
            let y: f32 = t * t * 4.9_f32;
            // ceilf(y / 5.0) -> (y / 5.0f).ceil()
            let res = (y / 5.0_f32).ceil() as i32 + 1;
            println!("{}", res);
        }
    }
}
