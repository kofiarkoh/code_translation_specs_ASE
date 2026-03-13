use std::io::{self, Read};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    // Read all stdin and split on whitespace tokens.
    // The SPEC assumes well-formed input; on parse failure behavior is unspecified.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut toks = input.split_whitespace();

    // Parse datasets count (int in the SPEC / C code).
    // Using unwrap to mirror the SPEC's assumption that parsing succeeds.
    let datasets: i32 = toks
        .next()
        .expect("missing datasets")
        .parse()
        .expect("datasets not an integer");

    // Process each dataset in order. SPEC requires datasets >= 0; we iterate 0..datasets.
    // If datasets is negative, behavior is unspecified by the SPEC; here we treat it as zero
    // to avoid infinite/unsafe behavior in Rust.
    if datasets <= 0 {
        // If datasets == 0, nothing to do; exit normally.
        return;
    }

    for _ in 0..datasets {
        // Read circle A: x, y, r
        let ax: f64 = toks.next().expect("missing ax").parse().expect("invalid ax");
        let ay: f64 = toks.next().expect("missing ay").parse().expect("invalid ay");
        let ar: f64 = toks.next().expect("missing ar").parse().expect("invalid ar");
        let a = Circle { x: ax, y: ay, r: ar };

        // Read circle B: x, y, r
        let bx: f64 = toks.next().expect("missing bx").parse().expect("invalid bx");
        let by: f64 = toks.next().expect("missing by").parse().expect("invalid by");
        let br: f64 = toks.next().expect("missing br").parse().expect("invalid br");
        let b = Circle { x: bx, y: by, r: br };

        // Determine bigger and smaller radii. If equal, A is treated as the larger-or-equal circle.
        let (bigr, smallr) = if a.r >= b.r { (a.r, b.r) } else { (b.r, a.r) };

        // Euclidean distance between centers.
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Classification following the SPEC and the C code's comparison logic.
        // Note: comparisons are exact as in the C code (no epsilon).
        let judge = if dist > bigr + smallr {
            0
        } else if dist > bigr - smallr {
            1
        } else {
            // d <= bigr - smallr: containment or internal tangency.
            // If bigr belongs to A (including tie), print 2; otherwise -2.
            if (bigr == a.r) {
                2
            } else {
                -2
            }
        };

        println!("{}", judge);
    }
}
