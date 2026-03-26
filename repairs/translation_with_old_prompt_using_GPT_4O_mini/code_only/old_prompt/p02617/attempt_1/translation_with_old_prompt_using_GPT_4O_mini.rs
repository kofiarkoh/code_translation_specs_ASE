use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut ans: i64 = 0;

    // First loop to calculate the initial value of ans
    for i in 1..=n {
        ans += (n - i + 2) as i64 * (n - i + 1) as i64 / 2;
    }

    // Second loop to adjust ans based on input pairs
    for _ in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let mut u: i32 = parts.next().unwrap().parse().unwrap();
        let mut w: i32 = parts.next().unwrap().parse().unwrap();

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
