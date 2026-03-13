// reverse_token.rs
//
// Translation of the provided C program to Rust following the given specification.
// Notes:
// - This program assumes stdin contains at least one whitespace-delimited token
//   whose byte length is <= 20 (per the SPEC preconditions).
// - The program copies the token's bytes in reverse order into a fixed-size 21-byte
//   buffer but does NOT write a terminating NUL byte to that buffer.
// - To match the original program's I/O behavior (printing the buffer as a C string
//   via printf("%s", ...)), this code calls the C printf directly with the buffer
//   pointer. That call is unsafe and can read beyond the buffer if there is no NUL
//   in memory after the copied bytes, matching the undefined behavior described in
//   the SPEC. The unsafe use is deliberate to preserve semantics of the original.
//
// Safety justification for unsafe: calling C's printf with a pointer to a non-NUL-terminated
// buffer mirrors the original C program's behavior. All other operations are safe Rust.

use std::io::Read;
use std::os::raw::{c_char, c_int};

extern "C" {
    // Declaration of C printf. We pass a C-format string and varargs.
    fn printf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token.
    let mut stdin = String::new();
    // We unwrap here because SPEC assumes there is input; if not, behavior is undefined.
    std::io::stdin().read_to_string(&mut stdin).unwrap();

    // Take the first token (split_whitespace matches scanf("%s") behavior).
    // Per SPEC preconditions, there is at least one token.
    let token = stdin.split_whitespace().next().unwrap();

    // Work with raw bytes to match C's behavior on bytes (char).
    let bytes = token.as_bytes();
    let x = bytes.len(); // intended: x in (0..=20)

    // Fixed-size buffers of 21 bytes each, as in the C program.
    // Initialize with zeros for deterministic storage; the SPEC allows indeterminate
    // contents beyond the first x bytes, but zero-init here is harmless as the
    // SPEC assumes inputs fit (<=20). We intentionally do NOT set buf_out[x] = 0 later.
    let mut _buf_in: [u8; 21] = [0u8; 21];
    let mut buf_out: [u8; 21] = [0u8; 21];

    // Copy input bytes into buf_in similar to scanf("%s", str) followed by strlen.
    // We copy all bytes (no bounds check here under the SPEC assumption that x <= 20).
    // If x > 21 in practice, the following loop would panic on indexing; SPEC marks
    // such cases as undefined, and we follow the SPEC by not adding explicit checks.
    for i in 0..x {
        _buf_in[i] = bytes[i];
    }

    // Copy reversed bytes into buf_out[0..x], do NOT write a terminating NUL.
    for i in 0..x {
        buf_out[i] = _buf_in[x - 1 - i];
    }

    // Prepare format string "%s" as a C string.
    let fmt = b"%s\0";
    let fmt_ptr = fmt.as_ptr() as *const c_char;
    let buf_ptr = buf_out.as_ptr() as *const c_char;

    // Unsafe call to C's printf to reproduce the original program's behavior.
    // This may read beyond buf_out if buf_out has no NUL after the first x bytes,
    // which is exactly the undefined behavior noted in the SPEC.
    unsafe {
        printf(fmt_ptr, buf_ptr);
    }

    // Exit with status 0 (Rust main returns normally -> exit code 0).
}
