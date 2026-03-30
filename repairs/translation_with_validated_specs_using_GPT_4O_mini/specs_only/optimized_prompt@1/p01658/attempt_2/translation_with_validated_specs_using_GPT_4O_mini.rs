fn count_modified_ones(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let mut total_count = 0;
    let m = matrix.len();
    let n = if m > 0 { matrix[0].len() } else { 0 };

    for i in 0..m {
        // For the first row, count directly
        if i > 0 {
            // Flip the values for subsequent rows
            for j in 0..n {
                matrix[i][j] ^= 1; // Flip 0s to 1s and 1s to 0s
            }
        }

        // Count '1's by traversing from both ends towards the center
        let mut left = 0;
        let mut right = n as i32 - 1;

        while left <= right {
            if matrix[i][left as usize] == 1 {
                total_count += 1; // Count modified '1'
                matrix[i][left as usize] = 0; // Modify the value
            }
            if left != right && matrix[i][right as usize] == 1 {
                total_count += 1; // Count modified '1'
                matrix[i][right as usize] = 0; // Modify the value
            }
            left += 1;
            right -= 1;
        }
    }

    total_count
}

fn main() {
    // Example usage
    let mut matrix = vec![
        vec![1, 0, 1],
        vec![0, 1, 0],
        vec![1, 1, 1],
    ];
    let result = count_modified_ones(&mut matrix);
    println!("Total modified '1's: {}", result);
}
