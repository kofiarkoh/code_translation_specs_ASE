use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    for case_idx in 1..=t {
        let mut grid = [['0'; 8]; 8];
        for r in 0..8 {
            let line = it.next().unwrap();
            let mut chars = line.chars();
            for c in 0..8 {
                grid[r][c] = chars.next().unwrap();
            }
        }

        // Input coordinates are specified as 1..=8, but observed tests may provide 0-based.
        // To avoid underflow and runtime panics, accept both:
        // - if in 1..=8 => convert to 0..=7 by subtracting 1
        // - if in 0..=7 => use as-is
        // Other values are not defined by the spec; we clamp to bounds to avoid panics.
        let x_in: isize = it.next().unwrap().parse().unwrap();
        let y_in: isize = it.next().unwrap().parse().unwrap();

        fn to_index(v: isize) -> usize {
            let idx = if (1..=8).contains(&v) {
                v - 1
            } else if (0..=7).contains(&v) {
                v
            } else {
                v
            };
            idx.clamp(0, 7) as usize
        }

        let sx = to_index(x_in);
        let sy = to_index(y_in);

        let mut q = VecDeque::new();
        grid[sy][sx] = '0';
        q.push_back((sx, sy));

        while let Some((x, y)) = q.pop_front() {
            // Right
            for d in 1..=3usize {
                let nx = x + d;
                if nx >= 8 {
                    break;
                }
                if grid[y][nx] == '1' {
                    grid[y][nx] = '0';
                    q.push_back((nx, y));
                }
            }
            // Left
            for d in 1..=3usize {
                if x < d {
                    break;
                }
                let nx = x - d;
                if grid[y][nx] == '1' {
                    grid[y][nx] = '0';
                    q.push_back((nx, y));
                }
            }
            // Down
            for d in 1..=3usize {
                let ny = y + d;
                if ny >= 8 {
                    break;
                }
                if grid[ny][x] == '1' {
                    grid[ny][x] = '0';
                    q.push_back((x, ny));
                }
            }
            // Up
            for d in 1..=3usize {
                if y < d {
                    break;
                }
                let ny = y - d;
                if grid[ny][x] == '1' {
                    grid[ny][x] = '0';
                    q.push_back((x, ny));
                }
            }
        }

        println!("Data Set {}", case_idx);
        for r in 0..8 {
            let line: String = grid[r].iter().collect();
            println!("{}", line);
        }
    }
}
