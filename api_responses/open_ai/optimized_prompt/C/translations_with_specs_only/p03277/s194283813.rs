fn find_threshold(n: usize, list: Vec<i32>) -> i32 {
    // Preconditions: The input list must contain at least one element.
    assert!(n > 0, "The list must contain at least one element.");
    
    // Initialize the threshold to the maximum possible value in the list.
    let mut threshold = *list.iter().max().unwrap();
    
    // Function to count valid pairs based on the current threshold.
    fn count_valid_pairs(threshold: i32, list: &[i32]) -> usize {
        let mut count = 0;
        let len = list.len();
        
        // Count pairs (i, j) where i < j and list[i] + list[j] > threshold
        for i in 0..len {
            for j in (i + 1)..len {
                if list[i] + list[j] > threshold {
                    count += 1;
                }
            }
        }
        count
    }
    
    // Calculate the total number of possible pairs.
    let total_pairs = n * (n - 1) / 2;
    
    // The required number of valid pairs is at least a quarter of total pairs.
    let required_pairs = (total_pairs + 3) / 4; // Using (total_pairs + 3) / 4 for ceiling division
    
    // Iterate to find the largest threshold value that meets the condition.
    while threshold >= 0 {
        let valid_pairs = count_valid_pairs(threshold, &list);
        if valid_pairs >= required_pairs {
            break; // Found the largest threshold meeting the condition
        }
        threshold -= 1; // Decrease threshold to check the next value
    }
    
    threshold
}

fn main() {
    // Example usage
    let n = 5;
    let list = vec![1, 2, 3, 4, 5];
    let threshold = find_threshold(n, list);
    println!("Threshold: {}", threshold);
}
