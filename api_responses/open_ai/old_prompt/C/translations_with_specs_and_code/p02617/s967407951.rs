use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Initialize ans to 0
    let mut ans: i64 = 0;

    // Calculate the initial sum based on N
    for i in 1..=n {
        ans += (n - i + 2) * (n - i + 1) / 2;
    }

    // Read N-1 pairs of integers (u, w)
    for _ in 1..n {
        let pair = lines.next().unwrap().unwrap();
        let mut nums = pair.split_whitespace().map(|s| s.parse::<usize>().expect("Invalid input for u or w"));
        let u = nums.next().unwrap();
        let w = nums.next().unwrap();

        // Ensure u is less than or equal to w
        let (u, w) = if u > w { (w, u) } else { (u, w) };

        // Adjust ans based on the pair
        ans -= (u as i64) * (n - w + 1) as i64;
    }

    // Print the result
    println!("{}", ans);
}
