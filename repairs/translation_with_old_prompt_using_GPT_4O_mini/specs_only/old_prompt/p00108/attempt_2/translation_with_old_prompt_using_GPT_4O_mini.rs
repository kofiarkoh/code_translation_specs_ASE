use std::collections::HashMap;
use std::io;

fn main() {
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first integer as the number of elements
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // If n is 0, we terminate without processing
    if n == 0 {
        return;
    }

    // Read the sequence of integers
    let mut numline: Vec<usize> = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    numline.extend(input.trim().split_whitespace().map(|s| s.parse::<usize>().expect("Invalid input")));

    // Ensure the length of numline matches n
    assert_eq!(numline.len(), n);

    let mut iterations = 0;
    let mut stable = false;

    while !stable {
        iterations += 1;
        let mut count_map: HashMap<usize, usize> = HashMap::new();

        // Count occurrences of each integer
        for &num in &numline {
            *count_map.entry(num).or_insert(0) += 1;
        }

        // Create a new transformed sequence based on counts
        let mut tmpline: Vec<usize> = Vec::with_capacity(n);
        for &num in &numline {
            tmpline.push(*count_map.get(&num).unwrap());
        }

        // Check if the new sequence is the same as the old one
        stable = tmpline == numline;

        // Update numline to the new transformed sequence
        numline = tmpline;
    }

    // Output the number of iterations and the final transformed sequence
    println!("{}", iterations);
    println!("{}", numline.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
}
