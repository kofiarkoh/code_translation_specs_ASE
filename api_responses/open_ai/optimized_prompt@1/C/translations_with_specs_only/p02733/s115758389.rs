fn min_segments(h: usize, w: usize, k: usize, grid: Vec<Vec<char>>) -> usize {
    // Initialize the count of segments
    let mut segments = 0;
    // This will hold the count of '1's in each column for the current segment
    let mut column_counts = vec![0; w];
    
    // Iterate through each row in the grid
    let mut start_row = 0;
    while start_row < h {
        // Reset column counts for the new segment
        column_counts.fill(0);
        let mut valid_segment = true;

        // Try to extend the segment from start_row
        for end_row in start_row..h {
            // Update column counts based on the current row
            for col in 0..w {
                if grid[end_row][col] == '1' {
                    column_counts[col] += 1;
                }
            }

            // Check if the current segment exceeds the allowed '1's in any column
            if column_counts.iter().any(|&count| count > k) {
                valid_segment = false;
                break; // Exit if the segment is invalid
            }
        }

        // If we found a valid segment, increment the segment count
        if valid_segment {
            segments += 1;
            // Move to the next row after the last valid end_row
            start_row += 1;
        } else {
            // If the segment is invalid, we need to move the start_row
            start_row += 1; // Move to the next row to try a new segment
        }
    }

    segments
}

// Example usage
fn main() {
    let h = 3;
    let w = 3;
    let k = 2;
    let grid = vec![
        vec!['1', '0', '1'],
        vec!['1', '1', '0'],
        vec!['0', '1', '1'],
    ];
    
    let result = min_segments(h, w, k, grid);
    println!("{}", result); // Output the result
}
