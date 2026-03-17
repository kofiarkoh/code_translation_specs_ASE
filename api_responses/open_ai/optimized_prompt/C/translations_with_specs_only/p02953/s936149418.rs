fn can_transform_to_non_increasing_sequence(input: Vec<i32>) -> String {
    // The first element indicates the number of elements in the sequence.
    let n = input[0] as usize;

    // We need to ensure we only consider the specified number of elements.
    let sequence = &input[1..=n];

    // We will check if we can transform the sequence into a non-increasing sequence.
    for i in 1..sequence.len() {
        // If the current element is greater than the previous one,
        // we cannot make it non-increasing by decrementing.
        if sequence[i] > sequence[i - 1] {
            return "No".to_string();
        }
    }

    // If we never found a violation, we can transform it.
    "Yes".to_string()
}

fn main() {
    // Example usage:
    let input = vec![5, 5, 4, 3, 2, 1]; // First element is the count, followed by the sequence.
    let result = can_transform_to_non_increasing_sequence(input);
    println!("{}", result); // Outputs: Yes
}
