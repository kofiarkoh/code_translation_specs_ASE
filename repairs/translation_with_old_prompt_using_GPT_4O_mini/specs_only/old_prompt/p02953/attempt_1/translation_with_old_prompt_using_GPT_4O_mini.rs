fn can_transform_to_non_decreasing(n: usize, a: &mut Vec<i32>) -> &'static str {
    // Start from the second last element and move towards the beginning
    for i in (1..n).rev() {
        // If the current element is greater than the next one, we can decrement it
        if a[i] > a[i - 1] {
            // Decrement the current element to match the next one
            a[i - 1] = a[i];
        }
    }
    // After processing, check if the array is non-decreasing
    for i in 1..n {
        if a[i] < a[i - 1] {
            return "No"; // If any element is less than the previous one, return "No"
        }
    }
    "Yes" // If we pass the check, return "Yes"
}

fn main() {
    // Example usage
    let n = 5;
    let mut a = vec![5, 3, 4, 2, 1]; // Example input
    let result = can_transform_to_non_decreasing(n, &mut a);
    println!("{}", result); // Outputs "No" for this example
}
