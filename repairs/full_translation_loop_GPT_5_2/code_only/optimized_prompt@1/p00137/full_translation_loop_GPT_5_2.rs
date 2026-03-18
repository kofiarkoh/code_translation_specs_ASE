use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C's scanf("%d") would (whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let counter: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut count: i32 = 0;

    while counter > count {
        let mut x: i32 = it.next().unwrap().parse().unwrap();

        let cter: i32 = 10;
        let mut c: i32 = 0;

        // Match exact formatting: "Case %d:\n"
        print!("Case {}:\n", count + 1);

        while c < cter {
            // In C, signed overflow is UB; typical behavior is 32-bit wraparound.
            // Use wrapping ops to emulate common two's-complement behavior.
            x = x.wrapping_mul(x);
            // C integer division truncates toward zero; Rust does the same for i32.
            x /= 100;
            // C % keeps the sign of the dividend; Rust matches that for i32.
            x %= 10000;

            c += 1;
            print!("{}\n", x);
        }

        count += 1;
    }
}
