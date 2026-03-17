use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: usize = input.trim().parse().expect("Input must be a number");
    
    // Check precondition: n must be >= 3
    if n < 3 {
        println!("0");
        return;
    }

    let mut nums = Vec::with_capacity(n);
    input.clear();
    
    // Read n integers
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.split_whitespace().take(n) {
        let number: i32 = num.parse().expect("Input must be a number");
        nums.push(number);
    }

    let mut count = 0;

    // Count strictly increasing or decreasing sequences of three consecutive integers
    for i in 0..(n - 2) {
        if nums[i] < nums[i + 1] && nums[i + 1] < nums[i + 2] {
            count += 1;
        } else if nums[i] > nums[i + 1] && nums[i + 1] > nums[i + 2] {
            count += 1;
        }
    }

    println!("{}", count);
}
