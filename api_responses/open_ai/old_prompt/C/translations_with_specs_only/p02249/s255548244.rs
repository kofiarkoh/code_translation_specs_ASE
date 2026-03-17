use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read dimensions of the text grid
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let (h, w) = (dimensions[0], dimensions[1]);

    // Read dimensions of the pattern grid
    let second_line = lines.next().unwrap().unwrap();
    let pattern_dimensions: Vec<usize> = second_line.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let (r, c) = (pattern_dimensions[0], pattern_dimensions[1]);

    // Read the text grid
    let mut text_grid = Vec::with_capacity(h);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        text_grid.push(line.chars().collect::<Vec<char>>());
    }

    // Read the pattern grid
    let mut pattern_grid = Vec::with_capacity(r);
    for _ in 0..r {
        let line = lines.next().unwrap().unwrap();
        pattern_grid.push(line.chars().collect::<Vec<char>>());
    }

    // Check if the pattern is larger than the text
    if r > h || c > w {
        return; // No output if the pattern is larger than the text
    }

    // Compute hash values for the pattern
    let pattern_hash = compute_hash(&pattern_grid);

    // Search for the pattern in the text grid
    for i in 0..=h - r {
        for j in 0..=w - c {
            let text_hash = compute_hash(&text_grid[i..i + r].iter().map(|row| &row[j..j + c]).collect::<Vec<_>>());
            if text_hash == pattern_hash {
                // Output the coordinates of the top-left corner of the match
                println!("{} {}", i, j);
            }
        }
    }
}

// Function to compute a simple hash for a grid
fn compute_hash(grid: &[&[char]]) -> usize {
    let mut hash = 0;
    for row in grid {
        for &ch in row {
            hash = hash.wrapping_mul(31).wrapping_add(ch as usize);
        }
    }
    hash
}
