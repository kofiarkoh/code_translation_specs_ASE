use std::collections::VecDeque;

fn min_transitions_to_edge(w: usize, h: usize, grid: Vec<Vec<char>>, start_char: char) -> Option<usize> {
    // Check for zero dimensions
    if w == 0 || h == 0 {
        return None; // Terminate without output
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    let mut min_transitions = usize::MAX;

    // Directions for moving up, down, left, right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Enqueue all starting positions with the starting character
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == start_char {
                queue.push_back((i, j, 0)); // (row, col, transition count)
                visited[i][j] = true;
            }
        }
    }

    // BFS to find minimum transitions to the edge
    while let Some((x, y, transitions)) = queue.pop_front() {
        // Check if we are at the edge
        if x == 0 || x == h - 1 || y == 0 || y == w - 1 {
            min_transitions = min_transitions.min(transitions);
        }

        // Explore adjacent cells
        for &(dx, dy) in &directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < h && new_y < w && !visited[new_x][new_y] {
                visited[new_x][new_y] = true;
                // Increment transition count if the character differs
                let new_transitions = if grid[new_x][new_y] != grid[x][y] {
                    transitions + 1
                } else {
                    transitions
                };
                queue.push_back((new_x, new_y, new_transitions));
            }
        }
    }

    // Return the minimum transitions divided by two
    if min_transitions == usize::MAX {
        None // No valid path found
    } else {
        Some(min_transitions / 2)
    }
}

fn main() {
    // Example usage
    let w = 5;
    let h = 5;
    let grid = vec![
        vec!['a', 'a', 'b', 'a', 'a'],
        vec!['a', 'b', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a', 'a'],
    ];
    let start_char = 'b';
    
    if let Some(transitions) = min_transitions_to_edge(w, h, grid, start_char) {
        println!("Minimum transitions to edge: {}", transitions);
    } else {
        println!("No valid path found or grid dimensions are zero.");
    }
}
