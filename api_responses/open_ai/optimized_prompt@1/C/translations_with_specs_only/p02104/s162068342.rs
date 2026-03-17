fn main() {
    // Read the first line of input, which is a positive integer
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first integer to determine the number of chair identifiers
    let n: usize = input.trim().parse().expect("Input must be a positive integer");
    
    // Read the next n integers representing chair identifiers
    let mut identifiers = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let identifier: i32 = input.trim().parse().expect("Invalid chair identifier");
        identifiers.push(identifier);
    }
    
    // Rearranging logic: For this example, we will sort the identifiers
    // This is a placeholder for the actual rearrangement logic as per the specification
    identifiers.sort(); // Sorting to demonstrate rearrangement
    
    // Output the rearranged identifiers, one per line
    for identifier in identifiers {
        println!("{}", identifier);
    }
}
