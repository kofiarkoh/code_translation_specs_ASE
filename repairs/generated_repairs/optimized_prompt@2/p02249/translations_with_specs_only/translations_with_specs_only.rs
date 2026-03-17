// Define a struct to hold the grid data and its dimensions
struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

// Function to search for a pattern in a text grid
fn find_pattern(text: &Grid, pattern: &Grid) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();

    // Iterate over each possible starting position in the text grid
    for row in 0..=text.height.saturating_sub(pattern.height) {
        for col in 0..=text.width.saturating_sub(pattern.width) {
            // Check if the pattern matches the text at the current position
            if is_match(text, pattern, row, col) {
                matches.push((row + 1, col + 1)); // Adjusting to 1-based indexing
            }
        }
    }

    matches
}

// Helper function to check if the pattern matches the text at a specific position
fn is_match(text: &Grid, pattern: &Grid, start_row: usize, start_col: usize) -> bool {
    for r in 0..pattern.height {
        for c in 0..pattern.width {
            if text.data[start_row + r][start_col + c] != pattern.data[r][c] {
                return false; // Mismatch found
            }
        }
    }
    true // All characters matched
}

// Example usage
fn main() {
    // Define a text grid
    let text = Grid {
        data: vec![
            vec!['a', 'b', 'c', 'd', 'e', 'f'],
            vec!['g', 'h', 'i', 'j', 'k', 'l'],
            vec!['m', 'n', 'o', 'p', 'q', 'r'],
            vec!['s', 't', 'u', 'v', 'w', 'x'],
            vec!['y', 'z', 'a', 'b', 'c', 'd'],
        ],
        height: 5,
        width: 6,
    };

    // Define a pattern grid
    let pattern = Grid {
        data: vec![
            vec!['i', 'j'],
            vec!['o', 'p'],
        ],
        height: 2,
        width: 2,
    };

    // Find matches
    let matches = find_pattern(&text, &pattern);
    
    // Print the results
    if matches.is_empty() {
        println!("Pattern not found");
    } else {
        for (row, col) in matches {
            println!("Pattern found at: ({}, {})", row, col);
        }
    }
}
