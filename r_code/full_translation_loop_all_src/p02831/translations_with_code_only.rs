use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code uses `long int` and scanf("%ld"), so use i64 (typical on modern platforms).
    // If input is missing/malformed, C's behavior is undefined; here we just default to 0.
    let a: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // The C code:
    // for (i = 1; i <= b + 1; i++) { if ((a*i)%b==0) { print a*i; break; } }
    //
    // Note: In C, signed overflow and modulo by 0 are undefined behavior.
    // We emulate common two's-complement wraparound for multiplication/addition using wrapping_*.
    // For b == 0, C would have UB at (..)%b; we avoid panicking and simply produce no output.
    if b != 0 {
        let mut i: i64 = 1;
        let limit = b.wrapping_add(1);
        while i <= limit {
            let prod = a.wrapping_mul(i);
            if prod % b == 0 {
                print!("{}\n", prod);
                break;
            }
            i = i.wrapping_add(1);
        }
    }
}
