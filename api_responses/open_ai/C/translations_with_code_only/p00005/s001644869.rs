use std::io::{self, Read};

fn main() {
    // Read all input like scanf would from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");

    // Parse whitespace-separated tokens as 32-bit signed integers.
    // We use i32 to mimic C's `int` (typical 32-bit).
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // Process pairs of integers. If there's an odd trailing token it is ignored.
    // (The original C with malformed input could exhibit undefined behavior;
    // here we choose to stop if a pair is not complete.)
    let mut i = 0usize;
    while i + 1 < nums.len() {
        let mut a = nums[i];
        let mut b = nums[i + 1];
        i += 2;

        // Mirror C logic: c starts at 1
        let mut c: i32 = 1;

        // If b > a swap so that a >= b
        if b > a {
            std::mem::swap(&mut a, &mut b);
        }

        // tmp_a and tmp_b are mutated versions of a and b
        let mut tmp_a = a;
        let mut tmp_b = b;

        // The original C loop uses `for (int i = 2; i <= tmp_b; i++)`
        // where `tmp_b` is mutated inside the loop, and the loop condition
        // is re-evaluated each iteration. Reproduce that by using a while loop.
        let mut divisor: i32 = 2;
        while divisor <= tmp_b {
            // while both divisible by divisor, multiply c by divisor and divide both
            while tmp_b % divisor == 0 && tmp_a % divisor == 0 {
                // Use wrapping_mul to emulate typical C (two's complement) wrap-around
                // behavior for signed 32-bit integers on overflow.
                c = c.wrapping_mul(divisor);
                tmp_b = tmp_b / divisor;
                tmp_a = tmp_a / divisor;
            }
            divisor = divisor + 1;
        }

        // Compute (tmp_a * tmp_b) * c with wrapping multiplications to mimic C int overflow.
        let product = tmp_a.wrapping_mul(tmp_b).wrapping_mul(c);

        // Preserve output formatting: "%d %d\n"
        println!("{} {}", c, product);
    }
}
