use std::io::{self, Read};

fn main() {
    // Read all stdin to a string and split on whitespace.
    // We use unwraps to mirror the original C program's lack of explicit input validation:
    // invalid/missing tokens will cause a panic here similar to undefined behavior in the C program.
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();
    let mut toks = input_str.split_whitespace();

    // Parse number of test cases as a signed int to match C behavior.
    let total: i32 = toks.next().unwrap().parse().unwrap();

    // If total <= 0, do zero iterations (matches C for negative or zero values).
    if total <= 0 {
        return;
    }

    for _ in 0..total {
        // Parse six doubles per test case.
        let x1: f64 = toks.next().unwrap().parse().unwrap();
        let y1: f64 = toks.next().unwrap().parse().unwrap();
        let x2: f64 = toks.next().unwrap().parse().unwrap();
        let y2: f64 = toks.next().unwrap().parse().unwrap();
        let x3: f64 = toks.next().unwrap().parse().unwrap();
        let y3: f64 = toks.next().unwrap().parse().unwrap();

        // Follow the C program's sequence of computations exactly (order matters for
        // floating-point corner cases). We intentionally allow IEEE floating behavior
        // (infinities/NaNs) when dividing by zero, matching the original.
        let midx1 = (x1 + x2) / 2.0;
        let midy1 = (y1 + y2) / 2.0;
        let midx2 = (x1 + x3) / 2.0;
        let midy2 = (y1 + y3) / 2.0;

        let slant1 = - (x2 - x1) / (y2 - y1);
        let slant2 = - (x3 - x1) / (y3 - y1);

        let slice1 = midy1 - slant1 * midx1;
        let slice2 = midy2 - slant2 * midx2;

        let mut circlex = (slice2 - slice1) / (slant1 - slant2);
        let mut circley = slant1 * circlex + slice1;

        // Replicate the C program's special-case adjustments in the same order.
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

        // Print with three digits after the decimal, space-separated, newline-terminated.
        // This matches the C printf format "%0.3f %0.3f %0.3f\n".
        println!("{:.3} {:.3} {:.3}", circlex, circley, radius);
    }
}
