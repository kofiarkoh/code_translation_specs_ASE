use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = input.split_whitespace();
    
    // Parse n and k as i64 (equivalent to long long int in C)
    let mut n: i64 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();
    
    // Initialize ans and modify n
    let mut ans = 0;
    n *= 7;

    // First loop: count occurrences based on n % 693
    for i in 1..=n % 693 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans += 1;
        }
    }

    // Second loop: count occurrences for fixed range 1 to 693
    let mut count = 0;
    for i in 1..=693 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            count += 1;
        }
    }
    ans += count * (n / 693);

    // Third loop: decrement ans for range 1 to 7
    for i in 1..=7 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans -= 1;
        }
    }

    // Fourth loop: decrement ans for the last 6 numbers in the range n - 6 to n
    for i in n - 6..=n {
        if i % 7 > 1 && (i * 9) % 11 == k {
            ans -= 1;
        }
    }

    // Print the result
    println!("{}", ans);
}
