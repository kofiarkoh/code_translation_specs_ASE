fn can_transform_to_non_decreasing(n: usize, a: &mut Vec<i32>) -> &'static str {
    // Start checking from the second last element to the first
    for i in (1..n).rev() {
        // If the current element is greater than the next one, we can decrement it
        if a[i - 1] > a[i] {
            // Decrement the current element to match the next one
            a[i - 1] = a[i];
        }
    }
    // After processing, check if the array is non-decreasing
    for i in 1..n {
        if a[i - 1] > a[i] {
            return "No"; // If any element is greater than the next, return "No"
        }
    }
    "Yes" // If we pass the check, return "Yes"
}

fn main() {
    let n = 5; // Example size of the array
    let mut a = vec![5, 3, 4, 2, 1]; // Example array
    let result = can_transform_to_non_decreasing(n, &mut a);
    println!("{}", result); // Output the result
}
