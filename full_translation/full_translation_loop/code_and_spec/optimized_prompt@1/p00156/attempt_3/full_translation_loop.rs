use std::io::{self, BufRead};

const MAX_SIZE: usize = 101;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut min_transitions = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (w, h) = (dimensions[0], dimensions[1]);
        if w == 0 && h == 0 {
            break;
        }

        let mut m = vec![vec![' '; w]; h];
        let mut c = [[0; MAX_SIZE]; MAX_SIZE];
        let mut start_x = 0;
        let mut start_y = 0;
        let mut found_start = false;

        for i in 0..h {
            let line = reader.lines().next().unwrap().unwrap();
            for (j, ch) in line.chars().enumerate() {
                m[i][j] = ch;
                if ch == '&' {
                    m[i][j] = '.'; // Mark the starting point
                    start_y = i;
                    start_x = j;
                    found_start = true;
                    c[i][j] = 1; // Initial transition count
                }
            }
        }

        if !found_start {
            // Behavior is not defined if no starting point is found
            continue;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut queue = vec![(start_x, start_y)];
        let mut r = 1;

        while r > 0 {
            let (x, y) = queue.remove(0);
            r -= 1;

            for &(dx, dy) in &directions {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if nx < w && ny < h {
                    if m[y][x] == m[ny][nx] {
                        if c[y][x] < c[ny][nx] || c[ny][nx] == 0 {
                            c[ny][nx] = c[y][x];
                            queue.push((nx, ny));
                        }
                    } else {
                        if c[y][x] + 1 < c[ny][nx] || c[ny][nx] == 0 {
                            c[ny][nx] = c[y][x] + 1;
                            queue.push((nx, ny));
                        }
                    }
                }
            }
        }

        let mut min = w; // Start with maximum possible transitions

        for i in 0..w {
            if c[0][i] / 2 < min {
                min = c[0][i] / 2;
            }
            if c[h - 1][i] / 2 < min {
                min = c[h - 1][i] / 2;
            }
        }

        for i in 0..h {
            if c[i][0] / 2 < min {
                min = c[i][0] / 2;
            }
            if c[i][w - 1] / 2 < min {
                min = c[i][w - 1] / 2;
            }
        }

        min_transitions.push(min);
    }

    for &min in min_transitions.iter() {
        println!("{}", min);
    }
}
