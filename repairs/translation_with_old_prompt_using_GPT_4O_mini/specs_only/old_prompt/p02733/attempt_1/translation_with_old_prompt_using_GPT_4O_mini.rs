use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read dimensions H and W
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    // Read maximum allowed number of '1's in any selected segment
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the binary grid
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(h);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    // Calculate the minimum number of segments
    let mut min_segments = 0;
    let mut current_segment_count = 0;
    let mut current_ones_count = 0;

    for start_row in 0..h {
        current_ones_count = 0;
        for end_row in start_row..h {
            // Count '1's in the current segment
            current_ones_count += grid[end_row].iter().filter(|&&c| c == '1').count();

            // If the count exceeds K, we need to finalize the current segment
            if current_ones_count > k {
                min_segments += 1;
                current_segment_count = 0; // Reset for the next segment
                break; // Exit the inner loop
            }
            current_segment_count += 1; // Increment the count of rows in the current segment
        }

        // If we finished the inner loop without exceeding K, we can finalize the last segment
        if current_ones_count <= k {
            min_segments += 1; // Count the last segment
        }
    }

    // Output the result
    println!("{}", min_segments);
}
