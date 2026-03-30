use std::collections::HashSet;

fn compute_hash(grid: &[Vec<char>]) -> usize {
    // Simple hash function based on the contents of the grid
    let mut hash = 0;
    for row in grid {
        for &ch in row {
            hash = hash.wrapping_mul(31).wrapping_add(ch as usize);
        }
    }
    hash
}

fn find_pattern_in_grid(
    text: &[Vec<char>],
    pattern: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let h = text.len();
    let w = text[0].len();
    let r = pattern.len();
    let c = pattern[0].len();

    let mut matches = Vec::new();
    let pattern_hash = compute_hash(pattern);

    for i in 0..=h - r {
        for j in 0..=w - c {
            // Extract the sub-grid from the text
            let sub_grid: Vec<Vec<char>> = text[i..i + r].iter().map(|row| row[j..j + c].to_vec()).collect();
            let sub_grid_hash = compute_hash(&sub_grid);

            // Check if hashes match before comparing characters
            if sub_grid_hash == pattern_hash {
                // Check for exact character match
                if sub_grid == *pattern {
                    matches.push((i, j));
                }
            }
        }
    }

    matches
}

// Example usage (not part of the specification):
// fn main() {
//     let text = vec![
//         vec!['a', 'b', 'c', 'd'],
//         vec!['e', 'f', 'g', 'h'],
//         vec!['i', 'j', 'k', 'l'],
//     ];
//     let pattern = vec![
//         vec!['f', 'g'],
//         vec!['j', 'k'],
//     ];
//     let result = find_pattern_in_grid(&text, &pattern);
//     for (row, col) in result {
//         println!("Pattern found at: ({}, {})", row, col);
//     }
// }
