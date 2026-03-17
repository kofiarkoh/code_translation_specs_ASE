fn min_segments(h: usize, w: usize, k: usize, grid: Vec<Vec<char>>) -> usize {
    // Initialize the number of segments to 0
    let mut segments = 0;
    // Initialize the current count of '1's in the current segment
    let mut current_count = 0;

    // Iterate over each row in the grid
    for row in grid {
        // Count the number of '1's in the current row
        let row_count: usize = row.iter().filter(|&&c| c == '1').count();

        // If adding this row exceeds the limit, we need a new segment
        if current_count + row_count > k {
            segments += 1; // Increment the segment count
            current_count = row_count; // Start a new segment with the current row
        } else {
            // Otherwise, add the row's '1's to the current segment count
            current_count += row_count;
        }
    }

    // If there are any '1's counted in the last segment, we need to count it as well
    if current_count > 0 {
        segments += 1;
    }

    segments // Return the total number of segments
}

// Example usage (uncomment to test):
// fn main() {
//     let h = 3;
//     let w = 3;
//     let k = 2;
//     let grid = vec![
//         vec!['1', '0', '1'],
//         vec!['1', '1', '0'],
//         vec!['0', '0', '1'],
//     ];
//     let result = min_segments(h, w, k, grid);
//     println!("{}", result); // Expected output based on the grid and k
// }
