use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse whitespace-separated tokens as i32. Invalid tokens are ignored.
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // Process complete groups of 6 integers, like scanf("%d %d %d %d %d %d", ...)
    // We use wrapping arithmetic to emulate typical 32-bit two's-complement wraparound
    // behavior instead of Rust's default checked behavior. The original C code would
    // have signed overflow as undefined, but many implementations wrap; to preserve
    // a reasonable and stable semantics we use wrapping ops.
    for chunk in nums.chunks(6) {
        if chunk.len() < 6 {
            break;
        }
        let a = chunk[0];
        let b = chunk[1];
        let c = chunk[2];
        let d = chunk[3];
        let e = chunk[4];
        let f = chunk[5];

        // Compute numerators and denominators with wrapping arithmetic (i32)
        let num_y = c.wrapping_mul(d).wrapping_sub(a.wrapping_mul(f));
        let den_y = b.wrapping_mul(d).wrapping_sub(a.wrapping_mul(e));

        let num_x = c.wrapping_mul(e).wrapping_sub(b.wrapping_mul(f));
        let den_x = a.wrapping_mul(e).wrapping_sub(b.wrapping_mul(d));

        // Convert to f32 and perform float division (as in C: float casts and float division)
        let mut y = (num_y as f32) / (den_y as f32);
        let mut x = (num_x as f32) / (den_x as f32);

        // Preserve the small-negative-to-zero adjustment:
        // if(-0.0005<x && x<=0 ) x = 0;
        if -0.0005_f32 < x && x <= 0.0_f32 {
            x = 0.0;
        }
        if -0.0005_f32 < y && y <= 0.0_f32 {
            y = 0.0;
        }

        // Print with three decimal places, preserving ordering and spacing: "x y\n"
        // This matches the C printf("%.3lf %.3f\n", x, y);
        println!("{:.3} {:.3}", x, y);
    }
}
