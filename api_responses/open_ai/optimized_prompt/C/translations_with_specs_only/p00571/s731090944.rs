fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of pairs
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for N");

    // Preconditions: N must be greater than zero
    assert!(n > 0, "N must be greater than zero");

    let mut max_value: i64 = i64::MIN; // Initialize max_value to the smallest possible long long
    let mut sum_of_seconds: i64 = 0; // To maintain the sum of the second elements
    let mut min_value: i64 = i64::MAX; // To track the minimum value

    // Process each pair
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Each pair consists of a long long and an integer
        let first: i64 = parts[0].parse().expect("Invalid long long value");
        let second: i32 = parts[1].parse().expect("Invalid integer value");

        // Update the sum of the second elements
        sum_of_seconds += second as i64;

        // Compute a specific value based on the input pair
        let computed_value = first + sum_of_seconds; // Example computation

        // Update max_value and min_value
        if computed_value > max_value {
            max_value = computed_value;
        }
        if computed_value < min_value {
            min_value = computed_value;
        }
    }

    // Postcondition: Output the maximum computed value
    println!("{}", max_value);
}
