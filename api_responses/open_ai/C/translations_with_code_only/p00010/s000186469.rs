use std::io::{self, Read};

fn main() {
    // Read all stdin and split by whitespace, similar to repeated scanf calls.
    let mut input_text = String::new();
    io::stdin().read_to_string(&mut input_text).unwrap();
    let mut parts = input_text.split_whitespace();

    // Parse the first token as the number of test cases.
    // If missing, exit (like scanf would block; here we simply end).
    let input_count: i32 = match parts.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => return,
    };

    // If input_count <= 0, C's for (i=0; i<input; i++) would do 0 iterations.
    let n = if input_count > 0 {
        input_count as usize
    } else {
        0usize
    };

    for _ in 0..n {
        // Read six f64 values; if any are missing or fail to parse, default to 0.0.
        // This is a pragmatic choice for robustness; the original C would block waiting for input.
        let x1: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y1: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let x2: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y2: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let x3: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y3: f64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);

        // Follow the original C computation exactly, using f64 arithmetic.
        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        // Preserve the original order of operations, including divisions that may
        // produce +/-inf or NaN when denominators are zero. Rust f64 semantics
        // match IEEE-754 like C on typical platforms.
        let slant1 = - (x2 - x1) / (y2 - y1);
        let slant2 = - (x3 - x1) / (y3 - y1);

        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let mut circlex = (slice2 - slice1) / (slant1 - slant2);
        let mut circley = slant1 * circlex + slice1;

        // Replicate the exact conditional adjustments from the C code.
        if x2 == x1 {
            circley = midy1;
            circlex = (circley - slice2) / slant2;
        } else if y2 == y1 {
            circlex = midx1;
            circley = slant2 * circlex + slice2;
        }

        if x3 == x1 {
            circley = midy2;
            circlex = (circley - slice1) / slant1;
        } else if y3 == y1 {
            circlex = midx2;
            circley = slant1 * circlex + slice1;
        }

        if x2 == x1 && y3 == y1 {
            circley = midy1;
            circlex = midx2;
        } else if x3 == x1 && y2 == y1 {
            circley = midy2;
            circlex = midx1;
        }

        let dx = circlex - x1;
        let dy = circley - y1;
        let radius = (dx * dx + dy * dy).sqrt();

        // Match printf("%0.3f %0.3f %0.3f\n", ...)
        // Rust's formatting "{:.3}" produces the same fixed 3-decimal output.
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
