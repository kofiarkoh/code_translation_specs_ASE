use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated floating-point numbers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    // Process in groups of 6 values until input is exhausted.
    // Spec: requires 6 valid floats per iteration; if not, stop (EOF behavior).
    let mut i = 0;
    while i + 5 < nums.len() {
        let a = nums[i];
        let b = nums[i + 1];
        let c = nums[i + 2];
        let d = nums[i + 3];
        let e = nums[i + 4];
        let f = nums[i + 5];
        i += 6;

        // Spec: if a == 0, division by zero is undefined; we intentionally do not handle it.
        let bai = d / a;
        let aa = a * bai; // computed in C but unused; keep for semantic fidelity
        let bb = b * bai;
        let cc = c * bai;

        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Match C's printf("%.3f %.3f\n", x, y);
        println!("{:.3} {:.3}", x, y);

        let _ = aa; // silence unused variable warning while preserving computation above
    }
}
