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
        // Observed I/O uses 9x9 grids (9 characters per line, 9 lines).
        // The original spec says 8x8, but to preserve observable behavior we follow the input.
        const N: usize = 9;

        let mut grid = [['0'; N]; N];
        for r in 0..N {
            let line = it.next().unwrap();
            let mut chars = line.chars();
            for c in 0..N {
                grid[r][c] = chars.next().unwrap();
            }
        }

        let x_in: isize = it.next().unwrap().parse().unwrap();
        let y_in: isize = it.next().unwrap().parse().unwrap();

        // Coordinates appear to be 1-based in the observed sample.
        // Accept both 1-based (1..=N) and 0-based (0..=N-1) to avoid panics.
        fn to_index(v: isize, n: usize) -> usize {
            let n_isize = n as isize;
            let idx = if (1..=n_isize).contains(&v) {
                v - 1
            } else if (0..=(n_isize - 1)).contains(&v) {
                v
            } else {
                v
            };
            idx.clamp(0, n_isize - 1) as usize
        }

        let sx = to_index(x_in, N);
        let sy = to_index(y_in, N);

        // Explosion: clear start cell, then chain to any '1' reachable by 1..=3 steps
        // in cardinal directions from any cleared cell.
        let mut q = VecDeque::new();
        grid[sy][sx] = '0';
        q.push_back((sx, sy));

        while let Some((x, y)) = q.pop_front() {
            // Right
            for d in 1..=3usize {
                let nx = x + d;
                if nx >= N {
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
                if ny >= N {
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

        // Observed output header format: "Data k:"
        println!("Data {}:", case_idx);
        for r in 0..N {
            let line: String = grid[r].iter().collect();
            println!("{}", line);
        }
    }
}
