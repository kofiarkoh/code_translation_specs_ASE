fn main() {
    use std::io::{self, BufRead};

    // Read the first line of input which indicates the number of integers
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // If the number of integers is less than 3, output 0
    if n < 3 {
        println!("0");
        return;
    }

    // Read the subsequent integers into a vector
    let mut numbers: Vec<i32> = Vec::with_capacity(n);
    for line in lines.take(n) {
        if let Ok(num_str) = line {
            // Attempt to parse each number; if it fails, we assume undefined behavior as per spec
            if let Ok(num) = num_str.trim().parse() {
                numbers.push(num);
            }
        }
    }

    // Count the number of strictly increasing or strictly decreasing sequences of three consecutive integers
    let mut count = 0;
    for i in 0..(n - 2) {
        if (numbers[i] < numbers[i + 1] && numbers[i + 1] < numbers[i + 2]) || 
           (numbers[i] > numbers[i + 1] && numbers[i + 1] > numbers[i + 2]) {
            count += 1;
        }
    }

    // Output the count
    println!("{}", count);
}
