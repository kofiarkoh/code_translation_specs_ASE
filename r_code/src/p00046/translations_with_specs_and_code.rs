use std::ffi::CString;
use std::io::Write;
use std::os::raw::{c_char, c_int};

extern "C" {
    // C varargs scanf: we declare it to call the C library's scanf to preserve
    // the exact parsing semantics of scanf("%f", ...). This is unsafe FFI use
    // but necessary here to match the C program's behavior (e.g. how it treats
    // non-numeric tokens and EOF).
    fn scanf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    // We'll use 32-bit floats to match C's `float`/`%f`.
    let mut m: f32 = 0.0;
    let mut hm: f32 = 0.0;
    let mut lm: f32 = 0.0;
    let mut first: i32 = 0;

    // Prepare the C format string "%f"
    let fmt = CString::new("%f").expect("CString::new failed");

    // EOF in C is a negative integer (typically -1)
    const EOF_C: c_int = -1;

    unsafe {
        // Loop with C scanf semantics: scanf returns EOF on end-of-file or error.
        // It returns 1 on successful parse, and returns 0 on a non-numeric token
        // (without consuming it) which reproduces the C program's potential
        // infinite-loop behavior on malformed input as specified.
        while scanf(fmt.as_ptr() as *const c_char, &mut m as *mut f32) != EOF_C {
            if first == 0 {
                first = 1;
                hm = m;
                lm = m;
            }
            if hm < m {
                hm = m;
            } else if lm > m {
                lm = m;
            }
        }
    }

    // The SPEC assumes at least one float is provided on stdin; under that
    // precondition we should have first == 1. If no floats were read, the C
    // program had undefined behavior. Here we choose to simply exit silently
    // (without printing) in that case rather than invoking undefined behavior
    // in Rust.
    if first == 0 {
        // Exit with status 0 (normal termination), matching the permissive
        // stance of the SPEC's preconditions (they require at least one value).
        std::process::exit(0);
    }

    // Print the difference formatted with one decimal place and a newline.
    // This uses Rust's formatting; it preserves the output shape "%.1f\n".
    // Flush explicitly to ensure identical observable stdout behavior.
    let delta = hm - lm;
    print!("{:.1}\n", delta);
    let _ = std::io::stdout().flush();
}
