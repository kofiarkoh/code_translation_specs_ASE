// Define the maximum capacity for the input sequence
const MAX_CAPACITY: usize = 100_000;

fn main() {
    // Read the number of elements in the sequence
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Ensure the number of elements does not exceed the maximum capacity
    if n == 0 || n > MAX_CAPACITY {
        panic!("Input must contain at least one integer and not exceed the maximum capacity.");
    }

    // Read the sequence of integers
    let mut sequence = Vec::with_capacity(n);
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.trim().split_whitespace() {
        let value: i32 = num.parse().expect("Invalid integer");
        sequence.push(value);
    }

    // Compute the length of the longest increasing subsequence
    let length = longest_increasing_subsequence(&sequence);
    
    // Output the length of the longest increasing subsequence
    println!("{}", length);
}

// Function to compute the length of the longest increasing subsequence
fn longest_increasing_subsequence(sequence: &[i32]) -> usize {
    let mut lis = Vec::new(); // This will store the longest increasing subsequence

    for &num in sequence {
        // Use binary search to find the insertion point
        let pos = lis.binary_search(&num).unwrap_or_else(|e| e);
        // If pos is equal to the length of lis, it means num is greater than all elements in lis
        if pos == lis.len() {
            lis.push(num);
        } else {
            // Replace the element at pos with num
            lis[pos] = num;
        }
    }

    // The length of lis is the length of the longest increasing subsequence
    lis.len()
}
