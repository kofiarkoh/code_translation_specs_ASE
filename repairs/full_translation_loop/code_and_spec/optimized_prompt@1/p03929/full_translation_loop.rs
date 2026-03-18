use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = input.split_whitespace();
    
    // Parse the two integers
    let n: i64 = nums.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = nums.next().unwrap().parse().expect("Invalid input for k");

    // Preconditions: n should be non-negative
    assert!(n >= 0, "n must be non-negative");

    let mut ans = 0;
    let n = n * 7;

    // First loop: count valid integers from 1 to n % 693
    for i in 1..=n % 693 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans += 1;
        }
    }

    // Count valid integers from 1 to 693
    let mut count = 0;
    for i in 1..=693 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            count += 1;
        }
    }
    ans += count * (n / 693);

    // Adjust ans by decrementing for specific conditions
    for i in 1..=7 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans -= 1;
        }
    }

    // Final adjustment for the range from n - 6 to n
    for i in n - 6..=n {
        if i % 7 > 1 && (i * 9) % 11 == k {
            ans -= 1;
        }
    }

    // Output the result
    println!("{}", ans);
}
