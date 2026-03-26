use std::collections::VecDeque;

const INF: i32 = 1_000_000_000; // A large value to represent infinity

fn max_distance_from_walls(h: usize, w: usize, a: Vec<Vec<char>>) -> i32 {
    let mut distances = vec![vec![INF; w]; h]; // Initialize distance array with INF
    let mut queue = VecDeque::new(); // Queue for BFS

    // Initialize distances and queue with wall positions
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                distances[i][j] = 0; // Distance to wall is 0
                queue.push_back((i, j)); // Add wall position to queue
            }
        }
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Perform BFS
    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize; // New x coordinate
            let ny = (y as isize + dy) as usize; // New y coordinate

            // Check if the new coordinates are within bounds
            if nx < h && ny < w && distances[nx][ny] == INF && a[nx][ny] == '.' {
                distances[nx][ny] = distances[x][y] + 1; // Update distance
                queue.push_back((nx, ny)); // Add new position to queue
            }
        }
    }

    // Find the maximum distance from the distances array
    let mut max_distance = 0;
    for i in 0..h {
        for j in 0..w {
            if distances[i][j] != INF {
                max_distance = max_distance.max(distances[i][j]); // Update max distance
            }
        }
    }

    max_distance // Return the maximum distance found
}

fn main() {
    let h: usize = 4; // Example height
    let w: usize = 5; // Example width
    let a: Vec<Vec<char>> = vec![
        vec!['.', '.', '#', '.', '.'],
        vec!['.', '#', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '#', '.', '.'],
    ]; // Example grid

    let result = max_distance_from_walls(h, w, a);
    println!("{}", result); // Print the result
}
