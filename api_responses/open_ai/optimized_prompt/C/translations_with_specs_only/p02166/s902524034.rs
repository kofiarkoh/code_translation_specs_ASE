use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let k: i32 = nums.next().unwrap().parse().unwrap();

    // Preconditions check
    if n == 0 {
        // Behavior is not defined for n <= 0, so we exit
        return;
    }

    // Read the array of integers
    let mut array: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Check if the number of elements equals the specified value
    if array.len() == n {
        // Check the modular arithmetic condition
        let all_satisfy_condition = array.iter().all(|&x| x % k == 0);
        // Mark elements as processed
        for elem in &mut array {
            *elem = -1; // Modify to -1 to indicate processed
        }
        // Output based on the condition
        if all_satisfy_condition {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        // Evaluate the parity of the count of processed elements
        let processed_count = array.iter().filter(|&&x| x == -1).count();
        // Output "Yes" if processed count is even, "No" if odd
        if processed_count % 2 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
