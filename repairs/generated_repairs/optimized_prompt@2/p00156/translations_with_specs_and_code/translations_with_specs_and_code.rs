use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (w, h) = (dimensions[0], dimensions[1]);
        if w == 0 && h == 0 {
            break; // Terminate on zero dimensions
        }

        let mut grid = vec![vec![' '; w]; h];
        let mut transition_count = vec![vec![0; w]; h];
        let mut start_x = 0;
        let mut start_y = 0;
        let mut found_start = false;

        for i in 0..h {
            let line = reader.lines().next().unwrap().unwrap(); // Read the next line for the grid
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;
                if c == '&' {
                    grid[i][j] = '.'; // Replace starting character
                    start_x = j;
                    start_y = i;
                    found_start = true;
                    transition_count[i][j] = 1; // Initial transition count
                }
            }
        }

        if !found_start {
            // Behavior is not defined if starting character is not found
            continue;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut queue = vec![(start_x, start_y)];
        let mut r = 1;

        while r > 0 {
            r = 0; // Reset the count of new transitions
            let mut new_queue = Vec::new();

            for &(x, y) in &queue {
                for &(dx, dy) in &directions {
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;

                    if nx < w && ny < h {
                        let current_char = grid[y][x];
                        let next_char = grid[ny][nx];

                        if current_char == next_char {
                            if transition_count[ny][nx] == 0 || transition_count[ny][nx] > transition_count[y][x] {
                                transition_count[ny][nx] = transition_count[y][x];
                                new_queue.push((nx, ny));
                                r += 1;
                            }
                        } else {
                            if transition_count[ny][nx] == 0 || transition_count[ny][nx] > transition_count[y][x] + 1 {
                                transition_count[ny][nx] = transition_count[y][x] + 1;
                                new_queue.push((nx, ny));
                                r += 1;
                            }
                        }
                    }
                }
            }

            queue = new_queue; // Update the queue for the next iteration
        }

        let mut min_transitions = w; // Start with maximum possible transitions

        // Check the edges for minimum transitions
        for i in 0..w {
            min_transitions = min_transitions.min(transition_count[0][i] / 2);
            min_transitions = min_transitions.min(transition_count[h - 1][i] / 2);
        }
        for i in 0..h {
            min_transitions = min_transitions.min(transition_count[i][0] / 2);
            min_transitions = min_transitions.min(transition_count[i][w - 1] / 2);
        }

        println!("{}", min_transitions);
    }
}
