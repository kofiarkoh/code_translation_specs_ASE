use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    let mut ans: i64 = 0;

    // Calculate the initial value of ans
    for i in 1..=n {
        ans += (n - i + 2) as i64 * (n - i + 1) as i64 / 2;
    }

    // Read pairs of integers and adjust ans accordingly
    for _ in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.split_whitespace();
        let u: i64 = parts.next().expect("Expected u").parse().expect("u must be an integer");
        let w: i64 = parts.next().expect("Expected w").parse().expect("w must be an integer");

        // Ensure u is less than or equal to w
        let (u, w) = if u > w { (w, u) } else { (u, w) };

        ans -= u * (n as i64 - w + 1);
    }

    // Output the result
    println!("{}", ans);
    io::stdout().flush().expect("Failed to flush stdout");
}
