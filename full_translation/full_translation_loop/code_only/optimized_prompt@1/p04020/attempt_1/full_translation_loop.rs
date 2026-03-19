use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![0; n + 1]; // Using 1-based indexing, so size is n + 1
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().unwrap();
    }

    let mut ans = 0;
    for i in 1..n {
        ans += a[i] / 2;
        if a[i + 1] > 1 && a[i] % 2 == 1 {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n] / 2;

    // Print the result
    println!("{}", ans);
    io::stdout().flush().unwrap(); // Ensure output is flushed
}
