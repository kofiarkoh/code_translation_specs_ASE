use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Translation of the Specification into Rust:
///
/// - Reads lines from stdin.
/// - For each line:
///   1) Replaces each input character c by ENCODE[(c - ' ') as usize] and concatenates those bit-strings.
///      ENCODE has 64 entries corresponding to ASCII ' ' (32) through '_' (95).
///      If an input byte is outside that range, the program will panic (emulating undefined/out-of-bounds behavior).
///   2) Scans the concatenated bit string left-to-right. Each bit '0' -> digit 1, '1' -> digit 2.
///      The running key K is a decimal number built as K = K*10 + digit, using wrapping arithmetic
///      (to emulate C unsigned wraparound semantics).
///      After each digit append, the code looks up the decode table for key K. If a non-zero byte is present,
///      that byte is written to stdout and K is reset to 0.
///      The decode table returns zero for keys not explicitly initialized.
///   3) After processing all bits, the program attempts to write a newline to stdout.
///
/// Notes:
/// - To remain faithful to the Specification and not invent mappings, ENCODE entries are initialized
///   to empty strings and the decode table is left empty (no keys initialized). This means typical input
///   will produce no decoded characters; this is consistent with the Spec which states only explicitly
///   initialized decode keys produce output.
/// - Wrapping arithmetic is used for the running key to emulate C unsigned overflow behavior.
/// - If input bytes fall outside ' '..'_', the code panics. The Spec says this is undefined behavior in C;
///   panicking here is the closest safe Rust analogue (program termination).
fn main() {
    // ENCODE table indexed by (byte - b' '), valid indices 0..64 corresponding to ' '..'_'
    // Per specification we do not invent the mapping: leave entries empty (empty strings are tolerated).
    // In the original program these would be short strings containing '0' and '1'.
    static ENCODE: [&str; 64] = [
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    ];

    // Decode table: maps decimal keys composed of digits 1 and 2 (as u64) to output byte (u8).
    // The Spec guarantees decode lookup returns zero for keys not set. We therefore represent only
    // explicitly-initialized keys in this map. To remain faithful, we initialize it empty.
    let decode: HashMap<u64, u8> = {
        let m = HashMap::new();
        // If the original program had explicit initializations, they'd be inserted here,
        // e.g. m.insert(12, b'A'); but the Spec forbids inventing such mappings.
        m
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut out_lock = stdout.lock();

    for line_result in stdin.lock().lines() {
        match line_result {
            Ok(line) => {
                // Build concatenated bit string as per Spec: replace each character by its ENCODE entry
                let mut bits = String::new();

                // The Spec's behavior is defined on bytes in ASCII range ' ' .. '_' inclusive.
                // We'll iterate the raw bytes and enforce that range; out-of-range leads to panic
                // (to emulate the original C program's undefined out-of-bounds access).
                for &b in line.as_bytes() {
                    if b < b' ' || b > b'_' {
                        panic!("input byte out of supported range causing undefined behavior (emulated)");
                    }
                    let idx = (b - b' ') as usize; // 0..63
                    // Indexing ENCODE with out-of-range would panic; here idx is guaranteed 0..63.
                    let s = ENCODE[idx];
                    // Append s to bits (s is expected to be composed of '0' and '1' chars).
                    bits.push_str(s);
                }

                // Now scan the concatenated bit string left-to-right, converting '0'->1, '1'->2,
                // building a decimal key K as K = K*10 + digit. Use wrapping arithmetic to emulate C unsigned wraparound.
                let mut key: u64 = 0;
                for ch in bits.bytes() {
                    let digit = match ch {
                        b'0' => 1u64,
                        b'1' => 2u64,
                        _ => {
                            // If ENCODE contained anything other than '0'/'1', original behavior is unspecified.
                            // We treat any other character as ignored (no digit append). This is a conservative choice.
                            continue;
                        }
                    };
                    // wrapping to emulate C unsigned behavior on overflow
                    key = key.wrapping_mul(10).wrapping_add(digit);

                    // Lookup decode table: missing keys are treated as zero (no output)
                    if let Some(&val) = decode.get(&key) {
                        if val != 0 {
                            // Write the corresponding byte to stdout immediately (as per Spec).
                            if let Err(e) = out_lock.write_all(&[val]) {
                                // In case of I/O error, propagate by terminating the program.
                                eprintln!("write error: {}", e);
                                std::process::exit(1);
                            }
                        }
                        // Reset running key after a match (even if val==0, Spec says only non-zero entries map to characters;
                        // the invariant states decode returns zero for uninitialized keys. We only reset when a non-zero is found.)
                        // Here, we reset only when val != 0.
                        if val != 0 {
                            key = 0;
                        }
                    } else {
                        // No entry -> treat as zero (no output), continue accumulating.
                        // (Do not reset key.)
                    }
                }

                // After processing all bits, attempt to emit a newline.
                // The original Spec notes the C program used printf("%\n"), which is undefined behavior.
                // We attempt to write a newline here, but the Spec does not guarantee it; we perform a normal write.
                if let Err(e) = out_lock.write_all(b"\n") {
                    eprintln!("failed to write newline: {}", e);
                    std::process::exit(1);
                }
                // Flush to ensure output is visible progressively, as decoded characters were written as discovered.
                if let Err(e) = out_lock.flush() {
                    eprintln!("flush error: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("error reading stdin: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Normal termination (return 0). In Rust main returns (), exit code 0 by default.
}
