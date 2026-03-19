use std::io::{self, Read};

fn digit_sum_5(mut x: i64) -> i64 {
    // Sum of the five least significant decimal digits.
    // For N <= 99,999 this matches the full digit sum; otherwise it intentionally ignores higher digits.
    let mut sum = 0i64;
    for _ in 0..5 {
        sum += x % 10;
        x /= 10;
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    if n < 1 {
        print!("0");
        return;
    }

    let mut total: i64 = 0;
    for i in 1..=n {
        let ds = digit_sum_5(i);
        if a <= ds && ds <= b {
            // Overflow behavior is unspecified by the specification; use normal i64 addition.
            total += i;
        }
    }

    print!("{total}");
}
