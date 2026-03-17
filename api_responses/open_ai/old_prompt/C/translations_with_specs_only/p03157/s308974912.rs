use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

fn main() -> i32 {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read height and width
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if dimensions.len() != 2 {
        return 1; // Invalid input
    }

    let h = dimensions[0];
    let w = dimensions[1];

    // Validate dimensions
    if h < 1 || h > MAX_SIZE || w < 1 || w > MAX_SIZE {
        return 1; // Invalid input
    }

    let mut grid: [[char; MAX_SIZE]; MAX_SIZE] = [['\0'; MAX_SIZE]; MAX_SIZE];
    let mut group: [[i32; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];
    let mut count: Vec<(u64, u64)> = vec![(0, 0); MAX_SIZE * MAX_SIZE];
    let mut group_id = 0;

    // Read the grid
    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        for j in 0..w {
            grid[i][j] = line.chars().nth(j).unwrap();
        }
    }

    // Directions for DFS (up, down, left, right)
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // DFS function to explore the grid
    fn dfs(
        grid: &[[char; MAX_SIZE]; MAX_SIZE],
        group: &mut [[i32; MAX_SIZE]; MAX_SIZE],
        count: &mut Vec<(u64, u64)>,
        x: usize,
        y: usize,
        group_id: i32,
    ) {
        let mut dot_count = 0;
        let mut other_count = 0;
        let char_type = grid[x][y];

        let mut stack = vec![(x, y)];
        while let Some((cx, cy)) = stack.pop() {
            if group[cx][cy] != 0 {
                continue; // Already visited
            }
            group[cx][cy] = group_id;

            if char_type == '.' {
                dot_count += 1;
            } else {
                other_count += 1;
            }

            for &(dx, dy) in &directions {
                let nx = (cx as isize + dx) as usize;
                let ny = (cy as isize + dy) as usize;
                if nx < MAX_SIZE && ny < MAX_SIZE && group[nx][ny] == 0 && grid[nx][ny] != char_type {
                    stack.push((nx, ny));
                }
            }
        }

        count[group_id as usize] = (dot_count, other_count);
    }

    // Process the grid to find groups
    for i in 0..h {
        for j in 0..w {
            if group[i][j] == 0 {
                group_id += 1;
                dfs(&grid, &mut group, &mut count, i, j, group_id);
            }
        }
    }

    // Calculate the metric based on the counts of '.' and other characters
    let mut metric: u64 = 0;
    for &(dot_count, other_count) in &count {
        if dot_count > 0 && other_count > 0 {
            metric += dot_count * other_count;
        }
    }

    // Output the result
    println!("{}", metric);
    0 // Successful execution
}
