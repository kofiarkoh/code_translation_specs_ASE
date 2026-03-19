use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse ints like scanf("%d%d") in a loop.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // C uses `int`; assume 32-bit and emulate its wraparound behavior where relevant.
    while let (Some(xs), Some(ys)) = (it.next(), it.next()) {
        let mut x: i32 = xs.parse().unwrap();
        let mut y: i32 = ys.parse().unwrap();

        // Emulate C's abs(int) behavior, including the common two's-complement wrap for INT_MIN.
        // In C, abs(INT_MIN) is undefined; this wrapping behavior matches typical implementations.
        x = x.wrapping_abs();
        y = y.wrapping_abs();

        let t: i32;
        if x.wrapping_add(y) == 1 {
            t = 1;
        } else if (x % 2 != 0) && (y % 2 != 0) {
            t = x.wrapping_add(y).wrapping_sub(1);
        } else if (x % 2 == 0) && (y % 2 == 0) {
            t = x.wrapping_add(y);
        } else if x != 0 && y != 0 {
            t = x.wrapping_add(y);
        } else {
            t = x.wrapping_add(y).wrapping_add(1);
        }

        print!("{t}\n");
    }
}
