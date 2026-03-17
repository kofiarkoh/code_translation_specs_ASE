use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");
    
    let mut ans = 0;
    let mut n = n * 7; // Transform n as per the specification

    // First loop: count occurrences based on n % 693
    for i in 1..=n % 693 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                ans += 1;
            }
        }
    }

    // Second loop: count occurrences for the full range of 693
    let mut count = 0;
    for i in 1..=693 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                count += 1;
            }
        }
    }
    ans += count * (n / 693);

    // Third loop: decrement ans for specific values
    for i in 1..=7 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                ans -= 1;
            }
        }
    }

    // Fourth loop: decrement ans for the last 6 values of n
    for i in n - 6..=n {
        if i % 7 > 1 {
            if (i * 9) % 11 == k {
                ans -= 1;
            }
        }
    }

    // Output the result
    println!("{}", ans);
}
