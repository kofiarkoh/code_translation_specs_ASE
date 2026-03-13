use std::io::Read;
use std::mem::MaybeUninit;

fn main() {
    // Read entire stdin into a String. This mirrors scanf("%f") reading text tokens from stdin.
    let mut input = String::new();
    // Ignore errors from read_to_string; a read error is treated like EOF per the specification.
    let _ = std::io::stdin().read_to_string(&mut input);

    let s = input;
    let len = s.len();
    let mut pos: usize = 0;

    // Track whether we've seen at least one successfully parsed float.
    let mut seen = false;
    let mut hm: f32 = 0.0;
    let mut lm: f32 = 0.0;

    // Main loop: mimic scanf("%f") behavior.
    // - Leading whitespace is skipped before each conversion.
    // - If no prefix starting at the first non-whitespace char can be parsed as a float,
    //   scanf would return 0 and not consume any input; that causes the program to repeatedly
    //   attempt the same conversion and (effectively) spin forever. We emulate that by
    //   not advancing `pos` in that case (which yields an infinite loop on such input).
    while pos < len {
        // Skip Unicode whitespace (roughly equivalent to C isspace on ASCII inputs).
        while pos < len {
            // Safe because pos < len, slicing at char boundary might fail; use chars().next()
            let rem = &s[pos..];
            if let Some(ch) = rem.chars().next() {
                if ch.is_whitespace() {
                    pos += ch.len_utf8();
                    continue;
                }
            }
            break;
        }

        if pos >= len {
            break; // EOF
        }

        // Try to find the longest prefix starting at `pos` that parses as an f32.
        // We attempt every byte-end index (skipping invalid char-boundary slices).
        let mut last_ok: Option<(usize, f32)> = None;
        let mut j = pos + 1;
        while j <= len {
            if let Some(sub) = s.get(pos..j) {
                if let Ok(v) = sub.parse::<f32>() {
                    // Record the furthest successful parse (maximal munch).
                    last_ok = Some((j, v));
                }
            }
            j += 1;
        }

        match last_ok {
            Some((new_pos, value)) => {
                // Consume the parsed characters and update min/max.
                pos = new_pos;
                if !seen {
                    hm = value;
                    lm = value;
                    seen = true;
                } else {
                    // Follow IEEE/C float semantics via f32 comparisons.
                    // Note: comparisons with NaN behave as in IEEE: they are false,
                    // so hm/lm won't update when compared with NaN, matching the spec.
                    if value > hm {
                        hm = value;
                    }
                    if value < lm {
                        lm = value;
                    }
                }
            }
            None => {
                // No numeric prefix: emulate scanf("%f") returning 0 and not consuming input.
                // According to the Specification, this typically results in an infinite loop.
                // We therefore do not advance `pos` and loop again (spin).
                // This intentionally produces the same observable behavior as the Spec.
                continue;
            }
        }
    }

    // After EOF:
    if seen {
        // Print (max - min) as a C printf("%.1f\n", ...): one fractional digit and newline.
        // Use f32 arithmetic to match C float semantics; format to 1 decimal place.
        let diff: f32 = hm - lm;
        // Use println! which appends a newline.
        println!("{:.1}", diff);
    } else {
        // No numeric tokens were successfully parsed.
        // The Specification states the program exhibits undefined behavior in this case
        // (e.g., printing an indeterminate value or crashing). To remain faithful to that
        // behavior we read two uninitialized f32 values and print their difference.
        //
        // WARNING: The following is intentionally unsafe and may produce undefined behavior,
        // which mirrors the C program described by the Specification when run with empty input.
        unsafe {
            let uninit_hm: f32 = MaybeUninit::uninit().assume_init();
            let uninit_lm: f32 = MaybeUninit::uninit().assume_init();
            let diff: f32 = uninit_hm - uninit_lm;
            println!("{:.1}", diff);
        }
    }

    // Exit with status 0 on normal termination (Rust exits with 0 unless panic or explicit exit).
}
