use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Parse N. If input is malformed this will panic (similar to a failing scanf
    // leaving the program in an unexpected state). Typical contest inputs are well-formed.
    let n: i32 = match it.next() {
        Some(s) => s.parse().expect("expected integer for N"),
        None => return, // no input -> nothing to do
    };

    // We mimic C's signed 32-bit int arithmetic with wrapping on overflow.
    // In C, signed overflow is technically undefined, but many implementations behave
    // as two's-complement wraparound; we preserve that behavior using wrapping ops.
    let mut i: i32 = 0;
    while i < n {
        // Read a, b, c as i32
        let a: i32 = it.next().expect("missing a").parse().expect("invalid a");
        let b: i32 = it.next().expect("missing b").parse().expect("invalid b");
        let c: i32 = it.next().expect("missing c").parse().expect("invalid c");

        // Follow the same chain of comparisons/orderings as the C code, in the same order.
        if a <= b && b <= c {
            // max = c; (unused in C)
            let lhs = c.wrapping_mul(c);
            let rhs = a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if a <= c && c <= b {
            // max = b;
            let lhs = b.wrapping_mul(b);
            let rhs = a.wrapping_mul(a).wrapping_add(c.wrapping_mul(c));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= c && c <= a {
            // max = a;
            let lhs = a.wrapping_mul(a);
            let rhs = b.wrapping_mul(b).wrapping_add(c.wrapping_mul(c));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= a && a <= c {
            // max = c;
            let lhs = c.wrapping_mul(c);
            let rhs = a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= a && a <= b {
            // max = b;
            let lhs = b.wrapping_mul(b);
            let rhs = a.wrapping_mul(a).wrapping_add(c.wrapping_mul(c));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= b && b <= a {
            // max = a;
            let lhs = a.wrapping_mul(a);
            let rhs = b.wrapping_mul(b).wrapping_add(c.wrapping_mul(c));
            if lhs == rhs {
                println!("YES");
            } else {
                println!("NO");
            }
        }

        i += 1;
    }
}
