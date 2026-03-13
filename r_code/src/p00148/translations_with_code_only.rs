// main.rs
//
// Translation of the given C program to Rust.
// This uses the C stdio functions via FFI to preserve scanf/printf semantics
// exactly (including corner cases such as scanf returning 0 and not
// consuming input). The FFI calls require `unsafe` blocks.
//
// The behavior matches the original C program:
//  - Repeatedly reads integers with scanf("%d", &n) until scanf returns EOF.
//  - For each read value, computes n = n % 39; if n == 0 then n = 39.
//  - Prints "3C%02d\n" with the resulting n.
//
// Note: Using C's scanf/printf preserves formatting and edge-case behaviors
// that would be difficult to emulate exactly with purely safe Rust IO.

use std::os::raw::{c_char, c_int};

extern "C" {
    // Variadic C functions from libc/stdio
    fn scanf(fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    // C string literals (null-terminated)
    let scan_fmt = b"%d\0".as_ptr() as *const c_char;
    let print_fmt = b"3C%02d\n\0".as_ptr() as *const c_char;
    // We must use unsafe to call C variadic functions.
    unsafe {
        let mut n: c_int = 0;
        loop {
            // scanf returns EOF (-1) on end-of-file. The original C loop used
            // `while (scanf("%d",&n) != EOF)` which continues for any return
            // value other than EOF (including 0). We mirror that behavior.
            let ret = scanf(scan_fmt, &mut n as *mut c_int);
            if ret == -1 {
                break;
            }
            // Preserve C-like remainder semantics (Rust/i32 has the same sign
            // behavior for % as C for two's complement platforms).
            n = n % 39;
            if n == 0 {
                n = 39;
            }
            // Print with the same format as C's printf.
            printf(print_fmt, n);
        }
    }
}
