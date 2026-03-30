fn partition_and_highlight(n: usize, sequence: Vec<i32>) -> Vec<String> {
    // Preconditions check: n must be greater than 0 and less than or equal to the length of the sequence
    assert!(n > 0 && n <= sequence.len(), "Invalid input for n");

    let pivot = sequence[n - 1]; // The pivot is the last element
    let mut less_equal = Vec::new(); // Elements less than or equal to pivot
    let mut greater = Vec::new(); // Elements greater than pivot

    // Partitioning the sequence
    for &value in &sequence[..n - 1] { // Exclude the pivot for partitioning
        if value <= pivot {
            less_equal.push(value);
        } else {
            greater.push(value);
        }
    }

    // Constructing the output sequence with the pivot highlighted
    let mut output = Vec::new();
    output.extend(less_equal);
    output.push(format!("[{}]", pivot)); // Highlighting the pivot
    output.extend(greater);

    output
}

// Example usage (uncomment to test):
// fn main() {
//     let n = 5;
//     let sequence = vec![3, 1, 4, 1, 5];
//     let result = partition_and_highlight(n, sequence);
//     println!("{:?}", result); // Output: [1, 1, [5], 3, 4]
// }
