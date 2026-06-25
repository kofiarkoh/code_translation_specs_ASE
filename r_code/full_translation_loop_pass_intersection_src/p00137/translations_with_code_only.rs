use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C-like whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, scanf("%d",&counter) is assumed to succeed.
    // If input is missing, do nothing (Rust can't proceed).
    let counter: i32 = match it.next() {
        Some(s) => s.parse::<i32>().unwrap(),
        None => return,
    };

    let mut count: i32 = 0;

    while counter > count {
        // scanf("%d",&x) is assumed to succeed for each case.
        let mut x: i32 = it.next().unwrap().parse::<i32>().unwrap();

        let cter: i32 = 10;
        let mut c: i32 = 0;

        print!("Case {}:\n", count + 1);

        while c < cter {
            // C signed overflow is UB; to preserve typical two's-complement wraparound
            // behavior, use wrapping arithmetic in Rust.
            x = x.wrapping_mul(x);
            x /= 100; // truncates toward zero, matching C for i32
            x %= 10000; // remainder has sign of dividend, matching C for i32
            c += 1;
            print!("{}\n", x);
        }

        count += 1;
    }
}
