use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let (h, w, k) = (nums[0], nums[1], nums[2]);

    // Ensure inputs are non-negative integers as per the specification
    if h < 0 || w < 0 || k < 0 {
        panic!("Inputs must be non-negative integers");
    }

    for i in 0..=h {
        for j in 0..=w {
            let now = (w * i) + (h * j) - 2 * i * j;

            if now == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
