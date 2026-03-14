use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        let n = nums.next().unwrap();

        // Adjust 'a' to be the remainder of 'a' divided by 'b'
        let mut a = a - (a / b) * b;
        let mut sum = 0;

        for _ in 1..=n {
            a *= 10;
            let mut j = 9;

            // Find the largest j such that b * j <= a
            while j >= 0 {
                if b * j <= a {
                    break;
                }
                j -= 1;
            }

            a -= b * j;
            sum += j;
        }

        // Print the result for the current input
        println!("{}", sum);
    }
}
