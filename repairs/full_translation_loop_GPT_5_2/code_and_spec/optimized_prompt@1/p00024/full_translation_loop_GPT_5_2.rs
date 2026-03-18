use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated floats until EOF.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // Per spec: invalid floats => undefined behavior; we mirror C by not handling gracefully.
        let v: f32 = tok.parse().unwrap();

        // Mirror C float arithmetic and truncation semantics:
        // n = (int)((4.9*v*v/9.8/9.8)+5)/5; n = n + 1;
        let mut n: i32 = (((4.9_f32 * v * v / 9.8_f32 / 9.8_f32) + 5.0_f32) as i32) / 5;
        n += 1;

        println!("{n}");
    }
}
