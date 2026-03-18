use std::io::{self, Read};

fn main() {
    // Read all input, parse as integers (undefined behavior for non-integers per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    let mut n = [0i32; 200]; // fixed-size digit buffer, initialized to zero

    for tok in input.split_whitespace() {
        let mut a: i32 = tok.parse().unwrap();
        if a == 0 {
            break; // sentinel
        }

        // Convert to octal digits (least significant first) into n[0..b)
        let mut b: usize = 0;
        while a != 0 {
            n[b] = a % 8;
            a /= 8;
            b += 1;
        }

        // Ensure terminating zero after last valid digit (invariant)
        if b < n.len() {
            n[b] = 0;
        }

        // Apply transformations. Note: C loops i <= b (includes the terminator).
        // This doesn't affect output because only digits 0..b-1 are printed,
        // but we preserve the behavior.
        for i in 0..=b {
            if i >= n.len() {
                break; // avoid out-of-bounds if b == 200 (not expected under typical inputs)
            }
            if n[i] > 3 {
                n[i] += 1;
            }
            if n[i] > 5 {
                n[i] += 1;
            }
        }

        // Output digits in reverse order (most significant first)
        for i in (0..b).rev() {
            out.push_str(&n[i].to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
