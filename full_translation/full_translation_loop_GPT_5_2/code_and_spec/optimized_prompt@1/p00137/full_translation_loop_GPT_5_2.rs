use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let counter: i32 = it.next().unwrap().parse().unwrap();
    let mut count: i32 = 0;

    while counter > count {
        let mut x: i32 = it.next().unwrap().parse().unwrap();

        let cter: i32 = 10;
        let mut c: i32 = 0;

        print!("Case {}:\n", count + 1);

        while c < cter {
            // Match C's int arithmetic semantics as closely as possible.
            // Note: In C, signed overflow is undefined; the spec does not define behavior here.
            // We use wrapping operations to avoid Rust panics and keep deterministic behavior.
            x = x.wrapping_mul(x);
            x /= 100;
            x %= 10000;
            c += 1;
            print!("{}\n", x);
        }

        count += 1;
    }
}
