use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior for invalid/missing input is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // If test-case count is negative, no output is produced.
    if t < 0 {
        return;
    }

    let mut out = String::new();

    for case_idx in 1..=(t as usize) {
        let mut x: i64 = it.next().unwrap().parse().unwrap();

        // Case header containing the 1-based case index.
        out.push_str(&format!("Case {}\n", case_idx));

        // Generate and print exactly 10 derived integers.
        for _ in 0..10 {
            // Transformation: repeatedly transform the current value.
            // The spec does not define the exact transformation; the narrowest
            // reasonable interpretation consistent with the stated invariant
            // (values in 0..=9999) is to map to the last 4 decimal digits.
            //
            // Note: For negative x, this yields a non-negative value in 0..=9999.
            x = x.rem_euclid(10_000);
            out.push_str(&format!("{}\n", x));
        }
    }

    print!("{out}");
}
