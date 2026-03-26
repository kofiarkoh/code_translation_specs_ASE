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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dimensions: Vec<usize> = input.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
    let h = dimensions[0];
    let w = dimensions[1];

    let mut grid = Vec::new();
    for _ in 0..h {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        grid.push(line.trim().chars().collect());
    }

    let result = check_isolation(h, w, &grid);
    println!("{}", result); // Outputs "Yes" or "No"
}
