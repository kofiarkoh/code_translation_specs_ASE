use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    // Behavior for non-integers is unspecified by the spec; we simply stop parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        let n: u64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified; choose narrow behavior: stop processing
        };

        if n == 0 {
            break; // sentinel
        }

        // Fixed-size array of octal digits, initialized to zero.
        // Invariant: there is a terminating zero after the last valid digit.
        let mut a = [0u8; 200];

        // Populate with octal digits (least significant first).
        // The spec assumes non-negative inputs; we use u64.
        let mut x = n;
        let mut i: usize = 0;

        // The spec expects at least one integer before sentinel; for n>0 we will store >=1 digit.
        while x != 0 && i < a.len() - 1 {
            a[i] = (x % 8) as u8;
            x /= 8;
            i += 1;
        }
        // a[i] is already 0 due to initialization, serving as terminator.

        // Apply transformations to each stored digit.
        for j in 0..i {
            if a[j] > 3 {
                a[j] = a[j].wrapping_add(1);
            }
            if a[j] > 5 {
                a[j] = a[j].wrapping_add(1);
            }
        }

        // Output in reverse order of the octal digits (most significant first).
        // Print digits as decimal characters (0..9) as implied by "octal digits".
        for j in (0..i).rev() {
            print!("{}", a[j]);
        }
        println!();
    }
}
