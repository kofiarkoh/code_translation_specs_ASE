// rle_decode.rs
//
// Translated from the provided C program.
// Behavior:
// - Reads all bytes from stdin.
// - Decodes sequences of the form '@' <digit_char> <char> by printing <char> repeated <digit> times.
// - Prints bytes (including newlines) otherwise, and always prints a final newline at the end.
// Notes on translation choices:
// - The original C code used a fixed-size uninitialized buffer and wrote NULs at i+1..i+3
//   (likely an off-by-one when attempting to null-terminate). To avoid undefined behavior
//   while preserving the intended semantics (read-all-bytes then process until a terminator),
//   this Rust version appends a single 0 (NUL) byte as a sentinel after the input bytes.
// - The digit is parsed as (byte - b'0') similarly to the original C code.
// - All I/O is done at the byte level to preserve exact output (no UTF-8 assumptions).
use std::io::{self, Read, Write};

fn main() {
    // Read all bytes from stdin
    let mut stdin = io::stdin();
    let mut input: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut input).expect("read failed");

    // Append a NUL sentinel so the processing loop can detect end-of-data.
    // The original C code set st[i+1..i+3] = 0 which appears to attempt to terminate
    // the buffer; here we append a single 0 to produce a well-defined termination.
    input.push(0);

    let mut out = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        match input[i] {
            b'@' => {
                // Need at least two more bytes (digit and the character to repeat).
                if i + 2 < input.len() {
                    // Emulate: kosuu = st[i+1] - 48;
                    // In C this is integer arithmetic on the char value.
                    let kosuu_i32 = (input[i + 1] as i32) - 48;
                    // If kosuu is negative, the original C code would loop strangely;
                    // here we treat negative as zero repetitions.
                    let reps = if kosuu_i32 > 0 { kosuu_i32 as usize } else { 0usize };
                    let ch = input[i + 2];
                    for _ in 0..reps {
                        out.push(ch);
                    }
                    i += 3;
                } else {
                    // Not enough bytes to form a complete '@' sequence.
                    // The C code would have undefined/out-of-bounds behavior here.
                    // To remain safe while being reasonable, emit the remaining bytes as-is.
                    // We'll push the '@' and move one forward.
                    out.push(b'@');
                    i += 1;
                }
            }
            b'\n' => {
                out.push(b'\n');
                i += 1;
            }
            0 => {
                // sentinel: stop processing
                break;
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }

    // The original program prints an extra final newline unconditionally.
    out.push(b'\n');

    // Write output to stdout
    let mut stdout = io::stdout();
    stdout.write_all(&out).expect("write failed");
}
