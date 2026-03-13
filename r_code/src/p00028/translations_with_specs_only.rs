// modes.rs
//
// Translation of the given Specification into Rust.
// Behavior notes (kept faithful to the Specification):
// - The program reads the entire stdin into memory and then simulates repeated
//   calls to scanf("%d", &number) using an internal cursor. This allows us to
//   emulate the exact scanning semantics required by the spec (notably: skip
//   leading whitespace; if the first non-whitespace character is not a sign or
//   digit, the scan *fails* without consuming input, which in the original C
//   program would cause scanf to return 0 repeatedly and the program to spin).
// - A successful integer parse advances the cursor past the sign and digits.
// - If EOF is encountered before any successful integer read, the program
//   prints nothing and exits 0.
// - Counts are kept in a fixed-size array for indices 0..=100. We increment
//   counters using wrapping addition on i32 (to avoid Rust debug-overflow
//   panics and to better mimic low-level integer increment semantics).
// - Out-of-range integer values are not checked; indexing outside 0..=100 will
//   cause a panic (to emulate the undefined/crash behavior described).
//
// The program prints each integer v in 1..=mx (inclusive) such that count[v]
// equals the global maximum count, where mx is chosen as the largest index
// with that maximum count (tie-break rule), one value per line in ascending order.

use std::io::{self, Read};

fn is_ascii_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B)
}

fn is_ascii_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

// Simulate scanf("%d") semantics on a byte buffer with a cursor index.
// Returns:
// - Ok(Some(value)) on successful parse (and advances cursor past the integer).
// - Ok(None) to represent scanf returning 0 (non-integer token encountered).
//   In this case the cursor is NOT advanced (so repeated calls will see the same token).
// - Err(()) to represent EOF (scanf returning EOF).
fn scanf_like(buf: &[u8], cursor: &mut usize) -> Result<Option<i32>, ()> {
    let len = buf.len();

    // Skip leading whitespace (consuming it)
    while *cursor < len && is_ascii_whitespace(buf[*cursor]) {
        *cursor += 1;
    }

    if *cursor >= len {
        // EOF
        return Err(());
    }

    // Now buf[*cursor] is the first non-whitespace byte.
    let b = buf[*cursor];

    // Accept optional '+' or '-' or a digit. If it's not one of these, scanf("%d")
    // would fail returning 0 and would not consume the non-digit token.
    let mut idx = *cursor;
    if b == b'+' || b == b'-' {
        idx += 1;
        // Must have at least one digit after sign; if not, treat as non-integer (scanf returns 0).
        if idx >= len || !is_ascii_digit(buf[idx]) {
            // Do NOT advance cursor.
            return Ok(None);
        }
    } else if !is_ascii_digit(b) {
        // Non-sign, non-digit character: scanf would return 0 and not consume it.
        return Ok(None);
    }

    // From idx (start or after sign), consume contiguous digits.
    // If there was no sign, start from original cursor.
    let start = *cursor;
    let mut digits_start = if buf[start] == b'+' || buf[start] == b'-' { start + 1 } else { start };
    idx = digits_start;
    while idx < len && is_ascii_digit(buf[idx]) {
        idx += 1;
    }

    // At least one digit is guaranteed here by above checks.
    // Parse the substring start..idx as an i32. The spec assumes scanf("%d") succeeds
    // for tokens that are valid integer representations; we mirror that assumption.
    let slice = &buf[start..idx];
    // Convert to &str. Since input is specified as textual integer tokens, UTF-8 is expected.
    let s = match std::str::from_utf8(slice) {
        Ok(s) => s,
        // If invalid UTF-8 appears inside digits/sign (very unlikely under spec),
        // treat as scanf failure without consuming (to be conservative).
        Err(_) => return Ok(None),
    };

    // Parse into i32. If parsing fails (e.g., overflow), we'll propagate an Err by panicking,
    // which mirrors the undefined/unsafe behavior described in the Specification for bad inputs.
    // However, under the Specification it's assumed tokens are valid integers, so this path
    // should not be exercised for well-formed inputs.
    let value = match s.parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            // Parsing failed. To stay conservative and consistent with scanf behavior on
            // malformed numeric representations, do not consume input and return a 0-like result.
            return Ok(None);
        }
    };

    // Advance cursor past the parsed integer.
    *cursor = idx;
    Ok(Some(value))
}

fn main() {
    // Read entire stdin into memory to allow simulating scanf semantics with an index.
    let mut input = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut input) {
        // If reading stdin fails, exit with 0 per spec that doesn't define error codes;
        // but print nothing and exit.
        let _ = e;
        return;
    }

    // Fixed-size counts array for indices 0..=100 inclusive.
    // Use i32 counters and wrapping increment to avoid Rust debug-overflow panics
    // and to better emulate low-level increment behavior.
    let mut counts: [i32; 101] = [0; 101];

    let mut cursor: usize = 0;
    let mut any_success = false;

    // Loop simulating: while (scanf("%d", &number) != EOF) { ... }
    loop {
        match scanf_like(&input, &mut cursor) {
            Err(()) => {
                // EOF reached
                break;
            }
            Ok(Some(value)) => {
                // Successful read: increment count at index `value`.
                // The Specification does NOT validate range 0..100; out-of-range
                // indexing is undefined behavior. Here, an out-of-range index will
                // panic (crash), which is consistent with "undefined/unsafe behavior".
                let idx = value as usize;
                // Use wrapping add to avoid potential panic on integer overflow in counter.
                counts[idx] = counts[idx].wrapping_add(1);
                any_success = true;
            }
            Ok(None) => {
                // scanf returned 0: non-integer token encountered and NOT consumed.
                // The original program would spin indefinitely because the loop condition
                // checks only for EOF. To mirror that behavior, we must not consume
                // input and must continue; this will cause the same function to return
                // Ok(None) again, effectively spinning. Here we implement that spin by
                // an infinite loop that repeatedly yields the thread to avoid pegging CPU.
                //
                // The spec says the program will not progress and will not terminate;
                // therefore any behavior that prevents progress (including spinning) is valid.
                use std::thread;
                use std::time::Duration;
                // Loop forever, mirroring the C program's behavior of repeatedly failing
                // scanf on the same token. We yield/sleep briefly to avoid busy CPU spin,
                // but observable behavior (no progress, no termination) matches the spec.
                loop {
                    // Sleep a bit to avoid a hot spin while preserving "no progress".
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }

    // If EOF happened immediately (no successful integer reads), produce no output.
    if !any_success {
        return;
    }

    // Determine the maximum count and choose mx as the largest index with that count.
    // As per spec: tie-break chooses the largest numeric index among those with the max count.
    let mut max_count = counts[0];
    let mut mx: usize = 0;
    for i in 1..=100 {
        let c = counts[i];
        if c >= max_count {
            max_count = c;
            mx = i;
        }
    }

    // Print integers v in 1..=mx whose count equals max_count, in ascending order.
    // If mx == 0 or no v in 1..=mx has count == max_count, nothing is printed.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    use std::io::Write;
    for v in 1..=mx {
        if counts[v] == max_count {
            // Each integer line followed by newline.
            let _ = writeln!(out, "{}", v);
        }
    }
}
