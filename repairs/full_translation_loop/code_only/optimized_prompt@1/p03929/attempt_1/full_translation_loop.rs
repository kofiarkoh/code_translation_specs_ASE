use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut ans = 0;
    let mut n = n * 7;

    // First loop: count for i from 1 to n % 693
    for i in 1..=n % 693 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                ans += 1;
            }
        }
    }

    // Count occurrences for i from 1 to 693
    let mut count = 0;
    for i in 1..=693 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                count += 1;
            }
        }
    }
    ans += count * (n / 693);

    // Decrement ans for i from 1 to 7
    for i in 1..=7 {
        if i % 7 > 1 {
            if (9 * i) % 11 == k {
                ans -= 1;
            }
        }
    }

    // Decrement ans for i from n - 6 to n
    for i in n - 6..=n {
        if i % 7 > 1 {
            if (i * 9) % 11 == k {
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
