use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_SIZE: usize = 101;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (width, height) = (dimensions[0], dimensions[1]);

        // Terminate if both width and height are zero
        if width == 0 && height == 0 {
            break;
        }

        let mut grid = vec![vec![' '; MAX_SIZE]; MAX_SIZE];
        let mut start_x = 0;
        let mut start_y = 0;
        let mut found_start = false;

        // Read the grid
        for i in 0..height {
            let line = reader.lines().next().unwrap().unwrap();
            for j in 0..width {
                grid[i][j] = line.chars().nth(j).unwrap();
                if grid[i][j] == 'S' && !found_start {
                    start_x = i;
                    start_y = j;
                    found_start = true;
                }
            }
        }

        // Perform BFS to find the minimum transitions to the edge
        let min_transitions = bfs(&grid, start_x, start_y, width, height);
        println!("{}", min_transitions);
    }
}

fn bfs(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize, width: usize, height: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; MAX_SIZE]; MAX_SIZE];
    let mut transition_count = vec![vec![usize::MAX; MAX_SIZE]; MAX_SIZE];

    queue.push_back((start_x, start_y, 0, grid[start_x][start_y]));
    visited[start_x][start_y] = true;
    transition_count[start_x][start_y] = 0;

    while let Some((x, y, transitions, last_char)) = queue.pop_front() {
        // Check if we are at the edge of the grid
        if x == 0 || x == height - 1 || y == 0 || y == width - 1 {
            return transitions;
        }

        for &(dx, dy) in &directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < height && new_y < width && !visited[new_x][new_y] {
                let new_char = grid[new_x][new_y];
                let new_transitions = if new_char != last_char { transitions + 1 } else { transitions };

                visited[new_x][new_y] = true;
                transition_count[new_x][new_y] = new_transitions;
                queue.push_back((new_x, new_y, new_transitions, new_char));
            }
        }
    }

    // If no edge is reachable, return a large number (or handle as needed)
    usize::MAX
}
