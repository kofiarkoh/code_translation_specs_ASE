fn max_sum_subarray(matrix: &Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut max_sum = i32::MIN;

    // Iterate over all possible starting rows
    for start_row in 0..n {
        // Create a temporary array to store cumulative sums for columns
        let mut temp = vec![0; n];

        // Iterate over all possible ending rows
        for end_row in start_row..n {
            // Update the cumulative sums for the current row
            for col in 0..n {
                temp[col] += matrix[end_row][col];
            }

            // Now find the maximum sum of the contiguous subarray in temp
            let mut current_sum = 0;
            for value in &temp {
                current_sum += value;
                if current_sum > max_sum {
                    max_sum = current_sum;
                }
                // Reset current_sum if it drops below zero
                if current_sum < 0 {
                    current_sum = 0;
                }
            }
        }
    }

    max_sum
}

fn main() {
    // Example usage
    let n = 3; // Size of the matrix
    let matrix = vec![
        vec![1, -2, 3],
        vec![4, -5, 6],
        vec![-7, 8, 9],
    ];

    let result = max_sum_subarray(&matrix);
    println!("{}", result); // Output the maximum sum
}
```

### Explanation of Choices:
1. **Matrix Representation**: The matrix is represented as a `Vec<Vec<i32>>`, which allows for dynamic sizing and is idiomatic in Rust.
2. **Cumulative Sums**: A temporary array `temp` is used to store cumulative sums for each column as we iterate through the rows, preserving the invariant that cumulative sums are correctly calculated.
3. **Maximum Subarray Calculation**: The inner loop implements a variation of Kadane's algorithm to find the maximum sum of contiguous subarrays in the `temp` array.
4. **Error Handling**: As per the specification, no explicit error handling is implemented for input validation, and behavior is undefined for invalid inputs.
5. **Performance**: The algorithm operates in O(n^3) time complexity, as required by the specification, due to the nested loops iterating over rows and columns.
