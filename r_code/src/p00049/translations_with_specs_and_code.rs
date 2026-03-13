// blood_groups.rs
//
// Translated from the provided C program, following the SPEC as the source of truth.
// Behavior notes:
// - Reads lines from stdin. Each line: integer ID (up to two digits), a separator char, and a blood-group token.
// - First assignment for an ID records its blood group and increments that group's counter.
// - Repeated identical assignments are ignored.
// - If a later line assigns a different group to an already-seen ID, the program enters an infinite loop (non-terminating failure).
// - On EOF (if no conflicting assignment was seen), prints four lines with counts for A, B, AB, O (in that order).
//
// Implementation choices:
// - Uses safe Rust; no `unsafe` used.
// - Parsing mirrors the SPEC/C behavior: skips leading whitespace, reads up to two ASCII digits for the ID,
//   skips whitespace, consumes one separator character if present, skips whitespace, then inspects the token.
// - Token mapping: "AB" -> AB (3), "B" -> B (2), "O" -> O (4), anything else -> A (1). This matches the C logic.
// - IDs greater than MAX (50) cause an out-of-bounds access if present, which in Rust will panic. The SPEC
//   states behavior for IDs > MAX is undefined; a panic in Rust is an acceptable manifestation of undefined behavior here.
// - On conflicting assignment the program loops forever using `loop { }` to model the non-terminating failure state.

use std::io::{self, BufRead};

const MAX: usize = 50;

fn main() {
    let stdin = io::stdin();
    // blood[k] == 0 means unseen; otherwise stores t in {1,2,3,4}
    let mut blood = [0u8; MAX + 5];
    // cnt[1]..cnt[4] used: 1=A, 2=B, 3=AB, 4=O
    let mut cnt = [0i32; 5];

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(s) => s,
            Err(_) => break,
        };
        let bytes = line.as_bytes();
        let n = bytes.len();
        let mut i: usize = 0;

        // skip leading whitespace
        while i < n && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // parse up to two digits for the ID
        let mut k: usize = 0;
        if i < n && bytes[i].is_ascii_digit() {
            k = (bytes[i] - b'0') as usize;
            i += 1;
            if i < n && bytes[i].is_ascii_digit() {
                k = k * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
        } else {
            // SPEC assumes a digit is present; if not, treat as 0 (mirrors permissive/undefined behavior)
            k = 0;
        }

        // skip whitespace, then consume one separator character if present (the C code does p++ unconditionally after skipping spaces)
        while i < n && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < n {
            // consume separator (could be comma or any char) as in original
            i += 1;
        }

        // skip whitespace before token
        while i < n && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        // Determine token -> t
        // "AB" -> 3, "B" -> 2, "O" -> 4, else -> 1 (A)
        let t: usize = if i < n && bytes[i] == b'A' && (i + 1) < n && bytes[i + 1] == b'B' {
            3
        } else if i < n && bytes[i] == b'B' {
            2
        } else if i < n && bytes[i] == b'O' {
            4
        } else {
            1
        };

        // Record or detect conflict.
        // Note: if k > MAX this will panic in Rust (index OOB). The SPEC declares behavior undefined for IDs > MAX.
        if blood[k] == 0 {
            blood[k] = t as u8;
            cnt[t] += 1;
        } else if blood[k] != t as u8 {
            // Enter non-terminating failure state on conflicting assignment.
            loop {
                // infinite loop: mirrors C's `while (1);`
            }
        }
    }

    // Print counts for A, B, AB, O (cnt[1]..cnt[4]), each on its own line.
    for idx in 1..5 {
        println!("{}", cnt[idx]);
    }
}
