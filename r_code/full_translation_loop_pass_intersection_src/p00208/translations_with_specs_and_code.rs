use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior on invalid input is unspecified per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let mut a: i32 = tok.parse().unwrap();
        if a == 0 {
            break;
        }

        // Spec precondition: nonzero inputs are non-negative.
        // Convert to base-8 digits (least significant first).
        let mut digits: Vec<i32> = Vec::new();
        while a != 0 {
            digits.push(a % 8);
            a /= 8;
        }

        // Transform each base-8 digit to a decimal digit skipping 4 and 6:
        // 0..3 -> same, 4..5 -> +1, 6..7 -> +2
        for d in &mut digits {
            if *d > 3 {
                *d += 1;
            }
            if *d > 5 {
                *d += 1;
            }
        }

        // Output most significant digit first.
        for &d in digits.iter().rev() {
            out.push_str(&d.to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
