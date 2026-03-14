use std::io::{self, BufRead};

const MAX_HEIGHT: usize = 100; // Define a maximum height for the grid
const MAX_WIDTH: usize = 100;  // Define a maximum width for the grid

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if dimensions.len() != 2 {
            continue; // Skip invalid input
        }

        let (height, width) = (dimensions[0], dimensions[1]);

        // Termination condition
        if height == 0 && width == 0 {
            break;
        }

        // Read the grid
        let mut grid = vec![vec![' '; width]; height];
        for i in 0..height {
            let line = reader.lines().next().unwrap().unwrap();
            grid[i] = line.chars().collect();
        }

        // Count distinct contiguous regions
        let region_count = count_regions(&mut grid, height, width);
        println!("{}", region_count);
    }
}

fn count_regions(grid: &mut Vec<Vec<char>>, height: usize, width: usize) -> usize {
    let mut count = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] != ' ' { // If the cell is not marked
                let char_to_mark = grid[i][j];
                count += 1; // Found a new region
                mark_region(grid, i, j, char_to_mark, &directions);
            }
        }
    }

    count
}

fn mark_region(grid: &mut Vec<Vec<char>>, x: usize, y: usize, char_to_mark: char, directions: &[(isize, isize)]) {
    let height = grid.len();
    let width = grid[0].len();

    let mut stack = vec![(x, y)];
    while let Some((cur_x, cur_y)) = stack.pop() {
        if cur_x < height && cur_y < width && grid[cur_x][cur_y] == char_to_mark {
            grid[cur_x][cur_y] = ' '; // Mark the cell as processed

            // Explore neighbors
            for &(dx, dy) in directions {
                let new_x = (cur_x as isize + dx) as usize;
                let new_y = (cur_y as isize + dy) as usize;
                if new_x < height && new_y < width {
                    stack.push((new_x, new_y));
                }
            }
        }
    }
}
