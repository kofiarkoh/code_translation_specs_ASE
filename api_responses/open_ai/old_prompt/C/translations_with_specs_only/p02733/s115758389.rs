fn min_segments(h: usize, w: usize, k: usize, grid: Vec<&str>) -> usize {
    // Initialize the number of segments needed
    let mut segments = 0;

    // Iterate over the rows to find segments
    let mut start_row = 0;

    while start_row < h {
        let mut count_ones = 0;
        let mut end_row = start_row;

        // Expand the segment until the count of '1's exceeds k
        while end_row < h && count_ones + grid[end_row].chars().filter(|&c| c == '1').count() <= k {
            count_ones += grid[end_row].chars().filter(|&c| c == '1').count();
            end_row += 1;
        }

        // We have found a valid segment from start_row to end_row - 1
        segments += 1;

        // Move to the next segment starting after the last valid row
        start_row = end_row;
    }

    segments
}

fn main() {
    // Example input
    let h = 3; // Number of rows
    let w = 5; // Number of columns
    let k = 2; // Maximum allowed '1's in a segment
    let grid = vec!["11000", "00101", "00011"]; // Binary grid

    // Calculate the minimum number of segments
    let result = min_segments(h, w, k, grid);

    // Print the result
    println!("{}", result);
}
