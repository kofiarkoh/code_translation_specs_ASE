use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut grid = vec![vec![' '; 101]; 101];
    let mut transitions = vec![vec![0; 101]; 101];

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let width = dimensions.next().unwrap();
        let height = dimensions.next().unwrap();

        if width == 0 && height == 0 {
            break;
        }

        let mut start_x = 0;
        let mut start_y = 0;
        let mut start_found = false;

        for i in 0..height {
            let line = reader.lines().next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;
                if c == '&' {
                    grid[i][j] = '.'; // Mark the starting point
                    start_x = j;
                    start_y = i;
                    start_found = true;
                }
            }
        }

        if !start_found {
            continue; // No starting point found, skip this grid
        }

        let mut queue = vec![(start_x, start_y)];
        transitions[start_y][start_x] = 1;

        let mut r = 1;

        while r > 0 {
            r -= 1;
            let (x, y) = queue.remove(0);

            for &(dx, dy) in &directions {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if nx < width && ny < height {
                    let current_char = grid[y][x];
                    let next_char = grid[ny][nx];

                    if current_char == next_char {
                        if transitions[ny][nx] == 0 || transitions[ny][nx] > transitions[y][x] {
                            transitions[ny][nx] = transitions[y][x];
                            queue.push((nx, ny));
                        }
                    } else {
                        if transitions[ny][nx] == 0 || transitions[ny][nx] > transitions[y][x] + 1 {
                            transitions[ny][nx] = transitions[y][x] + 1;
                            queue.push((nx, ny));
                        }
                    }
                }
            }
        }

        let mut min_transitions = width;

        for i in 0..width {
            if transitions[0][i] > 0 {
                min_transitions = min_transitions.min(transitions[0][i] / 2);
            }
            if transitions[height - 1][i] > 0 {
                min_transitions = min_transitions.min(transitions[height - 1][i] / 2);
            }
        }

        for i in 0..height {
            if transitions[i][0] > 0 {
                min_transitions = min_transitions.min(transitions[i][0] / 2);
            }
            if transitions[i][width - 1] > 0 {
                min_transitions = min_transitions.min(transitions[i][width - 1] / 2);
            }
        }

        println!("{}", min_transitions);
    }
}
