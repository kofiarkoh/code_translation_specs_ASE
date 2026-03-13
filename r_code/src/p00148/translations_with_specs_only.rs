// translate_spec.rs
//
// Implements the behavior described in the Specification:
// - Read whitespace-separated integer tokens from stdin until EOF.
// - For each integer n read (treated as a C int, here i32), compute r = n % 39
//   using C-like remainder semantics (sign follows dividend).
// - If r == 0, substitute r := 39.
// - Print a line "3C" followed by r formatted like printf("%02d"), then '\n'.
// 
// Important fidelity notes / choices (documented to justify choices):
// - We use i32 to represent a C int (common, narrow reasonable choice).
// - Input is parsed from the raw stdin bytes. Leading ASCII whitespace is skipped
//   like scanf("%d") would do. If the next non-whitespace character does not
//   begin a valid %d conversion (optional '+'/'-' followed by at least one digit),
//   the program does NOT consume that token and enters a livelock (infinite loop),
//   matching the Specification's described behavior for scanf("%d") returning 0.
// - If the digit sequence cannot be parsed into i32 (e.g., overflow or other
//   parse error), we treat that like a non-integer token (i.e., do not consume
//   it and livelock). The Specification left out exact overflow handling; this
//   is the narrowest reasonable interpretation that preserves the described
//   exceptional behavior (no explicit handling).
// - The "%02d" formatting behavior for negatives (sign plus zero-padding of
//   magnitude) is matched using Rust's "{:02}" formatting which places the sign
//   before any zero padding, consistent with C's printf behavior.
//
// The program returns exit code 0 on normal termination (EOF reached).
use std::io::Read;

fn is_ascii_whitespace(b: u8) -> bool {
    b.is_ascii_whitespace()
}

fn is_ascii_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

fn main() {
    // Read entire stdin into a buffer. This keeps processing semantics simple:
    // we can examine and choose not to consume a token (to emulate scanf("%d")
    // returning 0 on non-integer tokens).
    let mut input = Vec::new();
    // Ignore read errors per spec (I/O error that causes EOF is treated as termination).
    let _ = std::io::stdin().read_to_end(&mut input);

    let mut pos: usize = 0;
    let len = input.len();

    // Process until EOF
    while pos < len {
        // Skip leading ASCII whitespace (like scanf("%d") would do).
        while pos < len && is_ascii_whitespace(input[pos]) {
            pos += 1;
        }
        if pos >= len {
            break; // EOF
        }

        // At this point, input[pos] is the next non-whitespace byte.
        let b = input[pos];

        // Helper to parse an integer token starting at `pos` when we have already
        // determined that it starts with sign+digit or digit. On success, advances
        // pos to the first byte after the integer and returns Some(n). On parse
        // failure (e.g., overflow), returns None and does NOT advance pos.
        let try_parse_number = |start: usize, end: usize| -> Option<i32> {
            // Convert slice to &str (digits and optional sign are ASCII).
            if let Ok(s) = std::str::from_utf8(&input[start..end]) {
                if let Ok(n) = s.parse::<i32>() {
                    return Some(n);
                }
            }
            // Parsing failed (invalid UTF-8 unlikely, or parse overflow/invalid).
            // According to our choice above, treat this as non-integer (do not consume).
            None
        };

        // Determine if we have an integer token at pos.
        if b == b'+' || b == b'-' {
            // Must be followed by at least one digit to constitute a valid "%d".
            if pos + 1 < len && is_ascii_digit(input[pos + 1]) {
                // Consume sign and subsequent digits.
                let mut idx = pos + 2;
                while idx < len && is_ascii_digit(input[idx]) {
                    idx += 1;
                }
                // Try to parse; on failure do not advance pos and enter livelock.
                if let Some(n) = try_parse_number(pos, idx) {
                    pos = idx; // consume token
                    // Compute r = n % 39 with C-like remainder semantics.
                    let mut r = n % 39;
                    if r == 0 {
                        r = 39;
                    }
                    // Print "3C" + formatted-number + newline. Use "{:02}" to emulate "%02d".
                    println!("3C{:02}", r);
                    continue;
                } else {
                    // Parse failed (e.g., overflow). Specification: not handled explicitly.
                    // We emulate the behavior of scanf returning 0 / not consuming the token,
                    // which leads to a livelock (infinite loop).
                    loop {
                        // busy-wait to emulate repeated unsuccessful scanf attempts.
                        // Intentionally empty to match specified behavior.
                    }
                }
            } else {
                // Sign not followed by digit => scanf would return 0 and not consume token.
                // Emulate livelock.
                loop {
                    // busy-wait infinite loop
                }
            }
        } else if is_ascii_digit(b) {
            // Starts with digit: consume consecutive digits
            let mut idx = pos + 1;
            while idx < len && is_ascii_digit(input[idx]) {
                idx += 1;
            }
            if let Some(n) = try_parse_number(pos, idx) {
                pos = idx;
                let mut r = n % 39;
                if r == 0 {
                    r = 39;
                }
                println!("3C{:02}", r);
                continue;
            } else {
                // Parse failed (overflow or other): emulate lifelock (do not consume)
                loop {
                    // busy-wait infinite loop
                }
            }
        } else {
            // Next non-whitespace char is not sign or digit: scanf("%d") would fail,
            // return 0 and not consume token => livelock.
            loop {
                // busy-wait infinite loop
            }
        }
    }

    // Normal termination on EOF: exit status 0
}
