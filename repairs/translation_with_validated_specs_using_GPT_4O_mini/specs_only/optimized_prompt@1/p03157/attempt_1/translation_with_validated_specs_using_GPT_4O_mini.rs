use std::collections::HashSet;

const MAX_SIZE: usize = 512;

fn count_groups_and_calculate_sum(h: usize, w: usize, grid: Vec<Vec<char>>) -> u64 {
    // Ensure the grid dimensions are within the specified limits
    assert!(h > 0 && h <= MAX_SIZE);
    assert!(w > 0 && w <= MAX_SIZE);
    assert_eq!(grid.len(), h);
    for row in &grid {
        assert_eq!(row.len(), w);
    }

    let mut visited = vec![vec![false; w]; h];
    let mut total_sum: u64 = 0;

    // Directions for adjacent cells (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Depth-first search to explore groups
    fn dfs(
        x: usize,
        y: usize,
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &[(isize, isize)],
    ) -> (u64, u64) {
        let mut dot_count = 0;
        let mut other_count = 0;
        let char_type = grid[x][y];

        // Stack for DFS
        let mut stack = vec![(x, y)];
        visited[x][y] = true;

        while let Some((cx, cy)) = stack.pop() {
            if char_type == '.' {
                dot_count += 1;
            } else {
                other_count += 1;
            }

            for &(dx, dy) in directions {
                let nx = (cx as isize + dx) as usize;
                let ny = (cy as isize + dy) as usize;

                if nx < grid.len() && ny < grid[0].len() && !visited[nx][ny] && grid[nx][ny] != char_type {
                    visited[nx][ny] = true;
                    stack.push((nx, ny));
                }
            }
        }

        (dot_count, other_count)
    }

    for i in 0..h {
        for j in 0..w {
            if !visited[i][j] {
                let (dot_count, other_count) = dfs(i, j, &grid, &mut visited, &directions);
                total_sum += dot_count * other_count;
            }
        }
    }

    total_sum
}

fn main() {
    // Example usage
    let h = 3;
    let w = 3;
    let grid = vec![
        vec!['.', 'a', 'a'],
        vec!['.', 'b', 'b'],
        vec!['c', 'c', '.'],
    ];

    let result = count_groups_and_calculate_sum(h, w, grid);
    println!("{}", result);
}
