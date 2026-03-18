fn min_segments(h: usize, w: usize, k: usize, grid: Vec<Vec<char>>) -> usize {
    // Initialize the count of segments
    let mut segments = 0;
    // This will hold the count of '1's in each column for the current segment
    let mut column_counts = vec![0; w];
    
    // Start from the first row
    let mut start_row = 0;

    while start_row < h {
        // Reset column counts for the new segment
        column_counts.fill(0);
        let mut valid_segment = true;

        // Try to extend the segment as far as possible
        for end_row in start_row..h {
            for col in 0..w {
                // Increment the count of '1's in the current column
                if grid[end_row][col] == '1' {
                    column_counts[col] += 1;
                }
                // Check if any column exceeds the limit K
                if column_counts[col] > k {
                    valid_segment = false;
                    break;
                }
            }
            // If the segment is still valid, continue
            if valid_segment {
                continue;
            } else {
                // If we exceed the limit, we finalize the current segment
                segments += 1;
                // Move the start_row to the next row after the last valid end_row
                start_row = end_row;
                break;
            }
        }

        // If we finished the loop without breaking, it means we can take the last segment
        if valid_segment {
            segments += 1;
            break;
        }
    }

    segments
}

fn main() {
    // Example usage
    let h = 3;
    let w = 3;
    let k = 1;
    let grid = vec![
        vec!['0', '1', '0'],
        vec!['1', '0', '1'],
        vec!['0', '0', '0'],
    ];
    
    let result = min_segments(h, w, k, grid);
    println!("{}", result);
}
