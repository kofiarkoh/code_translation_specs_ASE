fn pattern_search(
    text_height: usize,
    text_width: usize,
    text_grid: Vec<Vec<char>>,
    pattern_height: usize,
    pattern_width: usize,
    pattern_grid: Vec<Vec<char>>,
) {
    // Preconditions: Check if dimensions are valid
    if text_height == 0 || text_width == 0 || pattern_height == 0 || pattern_width == 0 {
        return; // Undefined behavior for non-positive dimensions
    }
    if pattern_height > text_height || pattern_width > text_width {
        return; // Undefined behavior if pattern exceeds text dimensions
    }

    // Iterate over each possible starting position in the text grid
    for i in 0..=text_height - pattern_height {
        for j in 0..=text_width - pattern_width {
            // Check for a match
            let mut match_found = true;
            for pi in 0..pattern_height {
                for pj in 0..pattern_width {
                    if text_grid[i + pi][j + pj] != pattern_grid[pi][pj] {
                        match_found = false;
                        break;
                    }
                }
                if !match_found {
                    break;
                }
            }
            // If a match is found, print the starting position
            if match_found {
                println!("{} {}", i, j);
            }
        }
    }
}

// Example usage (commented out to avoid execution in a library context)
// fn main() {
//     let text_grid = vec![
//         vec!['a', 'b', 'c', 'd'],
//         vec!['e', 'f', 'g', 'h'],
//         vec!['i', 'j', 'k', 'l'],
//     ];
//     let pattern_grid = vec![
//         vec!['f', 'g'],
//         vec!['j', 'k'],
//     ];
//     pattern_search(3, 4, text_grid, 2, 2, pattern_grid);
// }
