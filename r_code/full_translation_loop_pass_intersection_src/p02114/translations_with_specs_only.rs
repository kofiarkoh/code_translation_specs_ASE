use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; input is assumed well-formed per specification, but the
    // grid may be provided as n lines of length n without whitespace between cells.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse n from the beginning, then parse the grid as raw characters.
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Skip leading whitespace
    while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }
    // Parse integer n
    let mut n: usize = 0;
    while idx < bytes.len() && !bytes[idx].is_ascii_whitespace() {
        let b = bytes[idx];
        if b.is_ascii_digit() {
            n = n * 10 + (b - b'0') as usize;
        }
        idx += 1;
    }

    // Collect next n*n non-whitespace bytes as cells.
    let need = n.saturating_mul(n);
    let mut cells: Vec<u8> = Vec::with_capacity(need);
    while idx < bytes.len() && cells.len() < need {
        let b = bytes[idx];
        if !b.is_ascii_whitespace() {
            cells.push(b);
        }
        idx += 1;
    }

    // If insufficient cells are present, behavior is undefined per spec; avoid panic.
    if cells.len() < need {
        return;
    }

    let mut grid: Vec<Vec<u8>> = vec![vec![b'.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = cells[i * n + j];
        }
    }

    let mut visited = vec![vec![false; n]; n];
    let mut components: i64 = 0;

    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] && grid[i][j] == b'o' {
                components += 1;
                visited[i][j] = true;

                let mut q = VecDeque::new();
                q.push_back((i, j));

                while let Some((x, y)) = q.pop_front() {
                    for (dx, dy) in dirs {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx < 0 || ny < 0 || nx >= n as isize || ny >= n as isize {
                            continue;
                        }
                        let nxu = nx as usize;
                        let nyu = ny as usize;
                        if !visited[nxu][nyu] && grid[nxu][nyu] == b'o' {
                            visited[nxu][nyu] = true;
                            q.push_back((nxu, nyu));
                        }
                    }
                }
            }
        }
    }

    // Output floor((C-1)/3) as integer division on signed integers.
    let ans = (components - 1) / 3;
    println!("{}", ans);
}
