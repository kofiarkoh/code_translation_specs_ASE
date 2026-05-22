use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (matches scanf("%d %d") and scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let h_opt = it.next();
        if h_opt.is_none() {
            break; // unspecified behavior in spec for malformed input; we just stop.
        }
        let h: usize = h_opt.unwrap().parse().unwrap();
        let w: usize = it.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        // Read grid rows; each row has at least w meaningful characters; extra ignored.
        let mut grid: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
        for i in 0..h {
            let row = it.next().unwrap_or("");
            let bytes = row.as_bytes();
            for j in 0..w {
                grid[i][j] = bytes[j];
            }
        }

        // Count 4-connected same-character regions.
        // The C code uses recursive DFS and an "area_id" propagation scheme.
        // Here we implement equivalent region counting via iterative flood fill to avoid recursion depth issues.
        // Note: Spec allows recursion and leaves stack overflow unspecified; using an explicit stack preserves
        // functional behavior while being safer.
        let mut visited = vec![vec![false; w]; h];
        let mut cnt: i32 = 0;

        for i in 0..h {
            for j in 0..w {
                if visited[i][j] {
                    continue;
                }
                cnt += 1;
                let ch = grid[i][j];
                let mut stack = Vec::new();
                stack.push((i, j));
                visited[i][j] = true;

                while let Some((x, y)) = stack.pop() {
                    // Down
                    if x + 1 < h && !visited[x + 1][y] && grid[x + 1][y] == ch {
                        visited[x + 1][y] = true;
                        stack.push((x + 1, y));
                    }
                    // Up
                    if x > 0 && !visited[x - 1][y] && grid[x - 1][y] == ch {
                        visited[x - 1][y] = true;
                        stack.push((x - 1, y));
                    }
                    // Right
                    if y + 1 < w && !visited[x][y + 1] && grid[x][y + 1] == ch {
                        visited[x][y + 1] = true;
                        stack.push((x, y + 1));
                    }
                    // Left
                    if y > 0 && !visited[x][y - 1] && grid[x][y - 1] == ch {
                        visited[x][y - 1] = true;
                        stack.push((x, y - 1));
                    }
                }
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
