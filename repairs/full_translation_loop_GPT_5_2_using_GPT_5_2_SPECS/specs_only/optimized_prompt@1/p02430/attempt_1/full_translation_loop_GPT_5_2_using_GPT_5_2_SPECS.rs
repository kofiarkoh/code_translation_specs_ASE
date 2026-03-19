use std::io::{self, Read};

fn main() {
    // Read entire stdin; if it fails, proceed with empty input (unspecified behavior in spec).
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut it = input.split_whitespace();

    // Spec says inputs are present/parseable; if not, behavior is not robustly defined.
    // We choose a narrow, deterministic fallback (0) rather than inventing error handling.
    let bit_width: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let required: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Spec assumes non-negative and small enough; if negative/too large, behavior is undefined.
    // We avoid panics and keep behavior deterministic by producing no output in such cases.
    if bit_width < 0 || required < 0 {
        return;
    }

    // Use u128 to keep shifts well-defined for "small enough" widths.
    // If bit_width is too large for u128 shifts, we produce no output (deterministic).
    let bw = bit_width as u32;
    if bw >= 128 {
        return;
    }

    let limit: u128 = 1u128 << bw;

    let mut out = String::new();

    for x in 0u128..limit {
        // Count set bits within the bit-width.
        let mut count: i64 = 0;
        for i in 0..bw {
            if ((x >> i) & 1) != 0 {
                count += 1;
            }
        }

        if count == required {
            // Output: integer value followed by indices of set bits in increasing order.
            out.push_str(&x.to_string());
            for i in 0..bw {
                if ((x >> i) & 1) != 0 {
                    out.push(' ');
                    out.push_str(&i.to_string());
                }
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
