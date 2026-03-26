fn check_isolation(h: usize, w: usize, grid: &[Vec<char>]) -> &'static str {
    // Directions for checking adjacent cells: up, down, left, right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                // Check all four cardinal directions for adjacent '#'
                for &(di, dj) in &directions {
                    let ni = i as isize + di; // new row index
                    let nj = j as isize + dj; // new column index

                    // Ensure we stay within bounds
                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                        if grid[ni as usize][nj as usize] == '#' {
                            return "No"; // Found an adjacent '#'
                        }
                    }
                }
            }
        }
    }
    "Yes" // All '#' characters are isolated
}

fn main() {
    let h = 3; // Example height
    let w = 3; // Example width
    let grid = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '.'],
        vec!['#', '.', '#'],
    ];

    let result = check_isolation(h, w, &grid);
    println!("{}", result); // Outputs "Yes" or "No"
}
