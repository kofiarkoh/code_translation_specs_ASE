use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let h = nums.next().unwrap();
    let w = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Iterate through all combinations of i and j
    for i in 0..=h {
        for j in 0..=w {
            let now = (w * i) + (h * j) - 2 * i * j;

            if now == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
