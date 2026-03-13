// translated.c -> translated.rs
// Direct translation of the given C program.
// Unsafe is used to emulate C-style raw indexing (car[i], out[k]) including
// the possibility of negative or out-of-range indices via pointer offset,
// so the Rust program's behavior matches the original C program as closely
// as possible (including corner cases that rely on pointer arithmetic).
// In typical use (well-formed push/pop sequences) the unsafe code behaves
// like normal array access; for malformed inputs this mirrors the C program's UB.

use std::io::{self, Read};
use std::ptr;

fn main() {
    // Read all input from stdin, equivalent to repeated scanf("%d", &n) until EOF
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Arrays sized exactly like the C code
    let mut car: [i32; 100] = [0; 100];
    let mut out: [i32; 100] = [0; 100];

    // Counters as in the C code
    let mut i: i32 = 0;
    let mut k: i32 = 0;

    // Parse whitespace-separated integers (equivalent to scanf "%d")
    for token in input.split_whitespace() {
        // If parsing fails we skip the token (scanf would behave differently on invalid input,
        // but typical inputs are integers).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        unsafe {
            // Get raw pointers to arrays to perform C-like pointer arithmetic.
            let car_ptr = car.as_mut_ptr();
            let out_ptr = out.as_mut_ptr();

            if n == 0 {
                i -= 1;
                // C does: out[k] = car[i];
                // Emulate by pointer offset: read from car + i, write to out + k.
                // This will mimic C pointer behavior (including possible out-of-bounds access).
                let src = car_ptr.offset(i as isize);
                let val = ptr::read(src);
                let dst = out_ptr.offset(k as isize);
                ptr::write(dst, val);
                k += 1;
            } else {
                // C does: car[i] = n;
                let dst = car_ptr.offset(i as isize);
                ptr::write(dst, n);
                i += 1;
            }
        }
    }

    // Print the collected outputs exactly as the C program: one integer per line.
    // Loop from 0 to k-1 inclusive (k is non-negative in normal operation).
    for idx in 0..(k as isize) {
        // Accessing out[ idx ] here is safe for the indices that the C program would print
        // (they are the same indices that were written to via out_ptr.offset above).
        // Use unsafe pointer deref to mirror any non-standard writes that may have occurred.
        let v = unsafe { ptr::read(out.as_ptr().offset(idx)) };
        println!("{}", v);
    }
}
