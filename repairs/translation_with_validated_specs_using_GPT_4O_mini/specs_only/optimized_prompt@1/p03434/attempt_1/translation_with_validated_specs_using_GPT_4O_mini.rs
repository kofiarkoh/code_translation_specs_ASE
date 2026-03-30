fn compute_difference(n: usize, arr: &mut [i32]) -> i32 {
    // Sort the array in non-decreasing order
    arr.sort();

    // Initialize sums for even and odd indexed elements
    let mut even_sum = 0;
    let mut odd_sum = 0;

    // Calculate sums based on sorted array's indices
    for i in 0..n {
        if i % 2 == 0 {
            even_sum += arr[i]; // Sum of elements at even indices
        } else {
            odd_sum += arr[i]; // Sum of elements at odd indices
        }
    }

    // Return the difference between even and odd sums
    even_sum - odd_sum
}

// Example usage
fn main() {
    let mut arr = [3, 1, 4, 1, 5, 9];
    let n = arr.len();
    let result = compute_difference(n, &mut arr);
    println!("{}", result); // Output the result
}
