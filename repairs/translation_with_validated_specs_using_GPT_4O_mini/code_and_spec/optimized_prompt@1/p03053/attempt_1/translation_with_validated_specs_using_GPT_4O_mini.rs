use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = 1001001001;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read height and width
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    // Initialize the grid and distance array
    let mut grid = vec![vec!['.'; w]; h];
    let mut distances = vec![vec![INF; w]; h];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Read the grid and initialize distances
    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        for j in 0..w {
            grid[i][j] = line.chars().nth(j).unwrap();
            if grid[i][j] == '#' {
                distances[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // Directions for BFS (right, down, left, up)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // BFS to calculate distances
    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < h as isize && ny >= 0 && ny < w as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == '.' && distances[nx][ny] > distances[x][y] + 1 {
                    distances[nx][ny] = distances[x][y] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // Find the maximum distance
    let mut max_distance = 0;
    for i in 0..h {
        for j in 0..w {
            max_distance = max_distance.max(distances[i][j]);
        }
    }

    // Output the result
    println!("{}", max_distance);
}
