use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin into a buffer. This mirrors the C program's streaming
    // behavior sufficiently for the specification: we process tokens from the
    // input until EOF.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let mut pos: usize = 0;
    let len = input.len();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Helper: skip ASCII whitespace
    let mut skip_whitespace = |pos: &mut usize| {
        while *pos < len && input[*pos].is_ascii_whitespace() {
            *pos += 1;
        }
    };

    loop {
        skip_whitespace(&mut pos);

        // If we've reached EOF, terminate normally (exit status 0).
        if pos >= len {
            break;
        }

        // Peek next byte without consuming.
        let b = input[pos];

        // If next byte can start an integer scan in C (%d), it must be a plus,
        // minus, or digit. Otherwise scanf("%d",...) would return 0 and leave
        // the non-matching character in the stream. The original C program
        // loops on (scanf("%d",&n) != EOF) and therefore, on a non-integer
        // token, it will repeatedly attempt to read the same token (leading to
        // a livelock). We replicate that behavior by not consuming the byte
        // and continuing the loop, which will retry forever.
        if b == b'+' || b == b'-' || (b'0'..=b'9').contains(&b) {
            // Attempt to parse an optional sign followed by digits.
            let mut p = pos;

            // Handle optional sign: if sign is present, there must be at least
            // one digit following for a successful match; otherwise, matching
            // fails and no characters are consumed (mimic scanf).
            if input[p] == b'+' || input[p] == b'-' {
                if p + 1 >= len || !(input[p + 1].is_ascii_digit()) {
                    // Matching fails; do not consume. This mirrors scanf("%d")
                    // returning 0 and leaving the token in the stream.
                    continue;
                }
                p += 1; // consume sign for now (we'll commit on success)
            }

            // Consume digits
            let digits_start = p;
            while p < len && input[p].is_ascii_digit() {
                p += 1;
            }

            // At this point, input[pos..p] is the candidate integer token.
            // Parse it as an i32. The SPEC assumes tokens are valid C ints,
            // so we don't specially handle overflows here.
            let slice = &input[pos..p];
            match std::str::from_utf8(slice).ok().and_then(|s| s.parse::<i32>().ok()) {
                Some(mut n) => {
                    // Commit consumption of the token.
                    pos = p;

                    // Compute r = n % 39 using C's remainder semantics.
                    // Rust's `%` on signed integers matches C's sign behavior
                    // (remainder has the sign of the dividend).
                    let mut r = n % 39;
                    if r == 0 {
                        r = 39;
                    }

                    // Print with the exact formatting: "3C" followed by the
                    // integer formatted as "%02d" and a trailing newline.
                    // Using "{:02}" produces zero-padding consistent with "%02d";
                    // negative values include the '-' sign as in C's printf.
                    write!(out, "3C{:02}\n", r).unwrap();
                }
                None => {
                    // Parsing failed (e.g., encoding error or overflow). To be
                    // conservative and to follow the SPEC which does not handle
                    // such cases explicitly, mimic scanf returning 0 by NOT
                    // consuming any input and continuing (possible livelock).
                    continue;
                }
            }
        } else {
            // Non-integer starting character: mimic scanf("%d") returning 0
            // and leaving the byte in the stream. This causes repeated attempts
            // and hence a livelock, matching the behavior described in the SPEC.
            continue;
        }
    }
}
