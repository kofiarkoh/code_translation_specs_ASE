use std::collections::VecDeque;

fn min_changes_to_center(w: usize, h: usize, m: Vec<Vec<char>>) -> i32 {
    // Find the position of the '&' character
    let mut start_x = 0;
    let mut start_y = 0;
    let mut found = false;

    for i in 0..h {
        for j in 0..w {
            if m[i][j] == '&' {
                start_x = j;
                start_y = i;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    // Initialize the change count array with a large number
    let mut c = vec![vec![i32::MAX; w]; h];
    c[start_y][start_x] = 0; // No changes needed at the starting point

    // BFS queue
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            // Check if the new position is within bounds
            if new_x < w && new_y < h {
                // Calculate the cost of changing the character
                let change_cost = if m[new_y][new_x] == '&' { 0 } else { 1 };
                let new_change_count = c[y][x] + change_cost;

                // If we found a cheaper way to reach (new_x, new_y), update and enqueue
                if new_change_count < c[new_y][new_x] {
                    c[new_y][new_x] = new_change_count;
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }

    // Find the minimum changes required to reach any edge of the grid
    let mut min_changes = i32::MAX;

    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                min_changes = min_changes.min(c[i][j]);
            }
        }
    }

    // If no edge was reachable, return -1 (though spec implies this won't happen)
    if min_changes == i32::MAX {
        return -1; // This case should not occur based on the spec
    }

    min_changes
}

fn main() {
    // Example usage
    let w = 5;
    let h = 5;
    let m = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '&', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];

    let result = min_changes_to_center(w, h, m);
    println!("{}", result); // Output the result
}
