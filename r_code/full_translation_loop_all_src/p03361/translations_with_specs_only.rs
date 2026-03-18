use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    // Fixed maximum grid size per specification (51x60).
    // If input exceeds these limits, behavior is not defined by the spec; we do not add handling.
    let mut grid = vec![vec!['\0'; w]; h];
    for i in 0..h {
        let s = it.next().unwrap();
        // The spec says the grid consists of characters; we interpret each row as a token.
        // If the row length differs from w, behavior is not defined; we do not add handling.
        for (j, ch) in s.chars().take(w).enumerate() {
            grid[i][j] = ch;
        }
    }

    let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != '#' {
                continue;
            }
            let mut ok = false;
            for (di, dj) in dirs {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                    continue;
                }
                if grid[ni as usize][nj as usize] == '#' {
                    ok = true;
                    break;
                }
            }
            if !ok {
                print!("No");
                return;
            }
        }
    }

    print!("Yes");
}
