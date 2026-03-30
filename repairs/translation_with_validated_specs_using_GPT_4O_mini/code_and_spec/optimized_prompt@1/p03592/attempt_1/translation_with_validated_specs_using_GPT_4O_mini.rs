use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid input"));

    let h = nums.next().unwrap();
    let w = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Preconditions: h, w, k must be non-negative
    if h < 0 || w < 0 || k < 0 {
        panic!("Input values must be non-negative integers.");
    }

    for i in 0..=h {
        for j in 0..=w {
            let now = (w * i) + (h * j) - (2 * i * j);
            if now == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
