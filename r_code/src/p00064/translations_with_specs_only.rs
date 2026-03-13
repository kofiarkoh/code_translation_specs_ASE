use std::io::{self, Read};

fn main() {
    // Read entire stdin into a byte buffer.
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // On read error, per specification there's no error reporting; exit normally.
        // But in normal circumstances stdin is available.
        return;
    }

    // Use a 128-bit unsigned accumulator to minimize chances of overflow while
    // preserving non-negative semantics. We use wrapping arithmetic so that if
    // overflow does occur, the program behavior is well-defined (wrap-around),
    // avoiding panics in debug builds. The original C specification left signed
    // overflow undefined; this is a conservative, practical emulation.
    let mut sum: u128 = 0;

    // Tokenization: split on ASCII whitespace. This matches the spec's
    // "whitespace-delimited token" behavior (using ASCII whitespace).
    //
    // The specification notes that the original implementation used an 81-byte
    // buffer for tokens (i.e., tokens should be ≤ 80 chars). To stay close to
    // that behavior without causing UB, we truncate tokens to 80 bytes before
    // scanning. The spec says behavior is not guaranteed when tokens exceed
    // 80 bytes; truncation is a narrow, well-defined choice.
    const TOKEN_MAX: usize = 80;

    for token in input.split(|&b| b.is_ascii_whitespace()) {
        if token.is_empty() {
            continue;
        }

        // Truncate to TOKEN_MAX bytes to emulate the original fixed buffer.
        let tok = if token.len() > TOKEN_MAX {
            &token[..TOKEN_MAX]
        } else {
            token
        };

        // Scan token byte-by-byte, extracting contiguous runs of ASCII digits.
        let mut in_run = false;
        let mut run_value: u128 = 0;

        for &ch in tok {
            if (b'0'..=b'9').contains(&ch) {
                // accumulate digit: run_value = run_value * 10 + (ch - b'0')
                // Use wrapping arithmetic to avoid panics on overflow.
                run_value = run_value.wrapping_mul(10);
                run_value = run_value.wrapping_add((ch - b'0') as u128);
                in_run = true;
            } else {
                if in_run {
                    sum = sum.wrapping_add(run_value);
                    run_value = 0;
                    in_run = false;
                }
                // Non-digit characters are separators; continue scanning.
            }
        }

        // If token ended while in a digit run, add the final run.
        if in_run {
            sum = sum.wrapping_add(run_value);
        }
    }

    // Print the sum followed by a newline. If no digit runs were present, sum == 0.
    // Use standard output and ignore potential write errors per spec (no other output).
    let mut out = String::new();
    out.push_str(&format!("{}\n", sum));
    let _ = io::stdout().write_all(out.as_bytes());
}
