use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut ans: i64 = 0;

    // Calculate the initial value of ans
    for i in 1..=n {
        ans += (n - i + 2) as i64 * (n - i + 1) as i64 / 2;
    }

    // Process the pairs of u and w
    for _ in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (mut u, mut w) = (nums[0], nums[1]);

        // Swap u and w if u > w
        if u > w {
            std::mem::swap(&mut u, &mut w);
        }

        ans -= (u as i64) * (n - w + 1) as i64;
    }

    // Print the result
    println!("{}", ans);
    io::stdout().flush().unwrap();
}
