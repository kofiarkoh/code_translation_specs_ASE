use std::collections::VecDeque;

fn count_connected_components(h: usize, w: usize, grid: Vec<Vec<char>>) -> usize {
    // Early exit if dimensions are zero
    if h == 0 && w == 0 {
        return 0;
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; w]; h];
    let mut component_count = 0;

    // Depth-First Search (DFS) function to explore connected components
    fn dfs(x: usize, y: usize, char: char, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, directions: &[(isize, isize)]) {
        let h = grid.len();
        let w = grid[0].len();
        let mut stack = VecDeque::new();
        stack.push_back((x, y));

        while let Some((cx, cy)) = stack.pop_back() {
            for &(dx, dy) in directions {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;

                // Check bounds and if the cell is the same character and not visited
                if nx >= 0 && nx < h as isize && ny >= 0 && ny < w as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !visited[nx][ny] && grid[nx][ny] == char {
                        visited[nx][ny] = true;
                        stack.push_back((nx, ny));
                    }
                }
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..h {
        for j in 0..w {
            if !visited[i][j] {
                // Found a new component
                visited[i][j] = true;
                component_count += 1;
                // Start DFS to mark all connected cells
                dfs(i, j, grid[i][j], &grid, &mut visited, &directions);
            }
        }
    }

    component_count
}

fn main() {
    // Read input until "0 0" is encountered
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (h, w) = (dimensions[0], dimensions[1]);
        
        if h == 0 && w == 0 {
            break; // Terminate on "0 0"
        }

        let mut grid = Vec::new();
        for _ in 0..h {
            let mut row = String::new();
            std::io::stdin().read_line(&mut row).unwrap();
            grid.push(row.chars().collect());
        }

        let result = count_connected_components(h, w, grid);
        println!("{}", result); // Output the number of connected components
    }
}
