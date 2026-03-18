use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (width, height) = (dimensions[0], dimensions[1]);
        if width == 0 && height == 0 {
            break; // Terminate on zero dimensions
        }

        let mut grid = Vec::new();
        for _ in 0..height {
            let grid_line = reader.lines().next().unwrap().unwrap();
            grid.push(grid_line.chars().collect::<Vec<_>>());
        }

        // Assuming the starting character is 'S' as per the specification
        let start_char = 'S';
        let mut min_transitions = usize::MAX;

        // Find the starting point
        let start_positions: Vec<(usize, usize)> = grid.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate()
                    .filter_map(|(x, &c)| if c == start_char { Some((x, y)) } else { None })
            })
            .collect();

        // If no starting point is found, behavior is undefined
        if start_positions.is_empty() {
            continue; // Skip this grid as per the specification
        }

        // BFS to find minimum transitions to any edge
        for &(start_x, start_y) in &start_positions {
            let mut queue = vec![(start_x, start_y, 0)]; // (x, y, transitions)
            let mut visited = vec![vec![false; width]; height];
            visited[start_y][start_x] = true;

            while !queue.is_empty() {
                let (x, y, transitions) = queue.remove(0); // Correctly remove the first element

                // Check if we are at the edge
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    min_transitions = min_transitions.min(transitions);
                    continue; // Found an edge, no need to explore further from here
                }

                // Explore adjacent cells
                let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for &(dx, dy) in &directions {
                    let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy)); // Wrap around for edges
                    if nx < width && ny < height && !visited[ny][nx] {
                        visited[ny][nx] = true;
                        queue.push((nx, ny, transitions + 1)); // Increment transition count
                    }
                }
            }
        }

        // Output the minimum transitions found for this grid
        println!("{}", min_transitions);
    }
}
