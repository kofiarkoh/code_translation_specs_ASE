fn main() {
    // Read the number of integers from input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of integers
    let n: usize = input.trim().parse().expect("Input must be a valid integer");
    
    // Check precondition: n must be >= 3
    if n < 3 {
        println!("0");
        return;
    }
    
    // Read the subsequent integers
    let mut numbers = Vec::new();
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the integers into a vector
    for num in input.trim().split_whitespace() {
        let parsed_num: i32 = num.parse().expect("Input must be valid integers");
        numbers.push(parsed_num);
    }
    
    // Initialize the count of valid sequences
    let mut count = 0;

    // Iterate through the list to count valid sequences
    for i in 0..(n - 2) {
        // Check for strictly increasing sequence
        if numbers[i] < numbers[i + 1] && numbers[i + 1] < numbers[i + 2] {
            count += 1;
        }
        // Check for strictly decreasing sequence
        else if numbers[i] > numbers[i + 1] && numbers[i + 1] > numbers[i + 2] {
            count += 1;
        }
    }
    
    // Output the total count of valid sequences
    println!("{}", count);
}
