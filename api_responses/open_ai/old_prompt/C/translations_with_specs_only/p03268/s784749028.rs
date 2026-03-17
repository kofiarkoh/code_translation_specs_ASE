fn main() {
    // Read input values for N and K
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse N and K as i64 (equivalent to long long in C)
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check if K is zero to avoid division by zero
    if k == 0 {
        // Undefined behavior as per specification; we can choose to panic here
        panic!("Division by zero: K should not be zero");
    }

    // Calculate the number of complete groups
    let complete_groups = n / k;

    // Compute the result based on the parity of K
    let result = if k % 2 == 0 {
        // If K is even, adjust the result
        complete_groups * complete_groups * complete_groups + complete_groups
    } else {
        // If K is odd, just return the cubic value
        complete_groups * complete_groups * complete_groups
    };

    // Output the result
    println!("{}", result);
}
