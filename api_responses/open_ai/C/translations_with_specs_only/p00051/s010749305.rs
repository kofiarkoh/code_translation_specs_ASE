fn main() {
    use std::io::{self, BufRead};

    // Read all input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // The first line contains the number of test cases
    let n: usize = lines[0].parse().expect("Expected a valid integer for number of test cases");

    // Process each test case
    for i in 1..=n {
        let number_str = &lines[i];
        
        // Ensure the number is an 8-digit non-negative integer
        if number_str.len() > 8 || !number_str.chars().all(char::is_numeric) {
            // Undefined behavior as per specification; we will just skip invalid input
            continue;
        }

        // Convert the string to a vector of digits
        let mut digits: Vec<char> = number_str.chars().collect();

        // Sort digits to find the minimum and maximum values
        digits.sort(); // Sorts in ascending order for minimum
        let min_value: i32 = digits.iter().collect::<String>().parse().unwrap();
        
        digits.reverse(); // Reverse to get descending order for maximum
        let max_value: i32 = digits.iter().collect::<String>().parse().unwrap();

        // Calculate the difference
        let difference = max_value - min_value;

        // Output the result
        println!("{}", difference);
    }
}
