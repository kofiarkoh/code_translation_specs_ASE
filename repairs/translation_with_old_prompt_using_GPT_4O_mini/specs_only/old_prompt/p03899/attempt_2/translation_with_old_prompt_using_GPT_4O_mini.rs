use std::io;

fn main() {
    // Read input values for n, m, k, and the array a
    let (n, m, k, a) = read_input();

    // Ensure preconditions are met
    if n == 0 || m >= n {
        // If n is 0 or m is not less than n, we cannot proceed
        println!("0");
        return;
    }

    // Initialize the dynamic programming table
    let mut dp = vec![0; n as usize];
    let mut d = vec![0; n as usize];

    // Fill the dp array based on the dynamic programming logic
    for i in 0..n {
        dp[i as usize] = a[i as usize]; // Start with the value of a[i]
        if i > 0 {
            dp[i as usize] = dp[i as usize].max(dp[(i - 1) as usize]); // Carry forward the maximum
        }
        if i >= m {
            dp[i as usize] = dp[i as usize].max(d[(i - m) as usize] + a[i as usize]); // Consider the weighted sum with separation
        }
        d[i as usize] = dp[i as usize]; // Store the current maximum in d
    }

    // The maximum value will be the last element in dp
    let max_value = *dp.iter().max().unwrap();
    
    // Print the result as specified
    println!("{}", max_value);
}

// Function to read input values
fn read_input() -> (i32, i32, i32, Vec<i32>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    let mut a = Vec::new();
    for _ in 0..n {
        let value: i32 = iter.next().unwrap().parse().unwrap();
        a.push(value);
    }
    
    (n, m, k, a)
}
