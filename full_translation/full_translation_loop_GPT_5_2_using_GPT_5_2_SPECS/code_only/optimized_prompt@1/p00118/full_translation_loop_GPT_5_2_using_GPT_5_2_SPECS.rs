use std::io::{self, Read};

fn dfs(i: usize, j: usize, h: usize, w: usize, grid: &Vec<Vec<u8>>, area_id: &mut Vec<Vec<i32>>) {
    // Down
    if i + 1 < h {
        let ni = i + 1;
        if grid[i][j] == grid[ni][j] && area_id[ni][j] == (ni * w + j) as i32 {
            area_id[ni][j] = area_id[i][j];
            dfs(ni, j, h, w, grid, area_id);
        }
    }
    // Up
    if i >= 1 {
        let ni = i - 1;
        if grid[i][j] == grid[ni][j] && area_id[ni][j] == (ni * w + j) as i32 {
            area_id[ni][j] = area_id[i][j];
            dfs(ni, j, h, w, grid, area_id);
        }
    }
    // Right
    if j + 1 < w {
        let nj = j + 1;
        if grid[i][j] == grid[i][nj] && area_id[i][nj] == (i * w + nj) as i32 {
            area_id[i][nj] = area_id[i][j];
            dfs(i, nj, h, w, grid, area_id);
        }
    }
    // Left
    if j >= 1 {
        let nj = j - 1;
        if grid[i][j] == grid[i][nj] && area_id[i][nj] == (i * w + nj) as i32 {
            area_id[i][nj] = area_id[i][j];
            dfs(i, nj, h, w, grid, area_id);
        }
    }
}

fn main() {
    // Tokenize stdin like repeated scanf calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let h_opt = it.next();
        if h_opt.is_none() {
            break; // no more input
        }
        let h: usize = h_opt.unwrap().parse().unwrap();
        let w: usize = it.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        // area_id initialized to i*W+j for i<H, j<W
        let mut area_id = vec![vec![0i32; w]; h];
        for i in 0..h {
            for j in 0..w {
                area_id[i][j] = (i * w + j) as i32;
            }
        }

        // area_num[128*128] = 0
        let mut area_num = vec![0u8; 128 * 128];
        let mut cnt: i32 = 0;

        // Read H strings (like scanf("%s", str[i])).
        // C code assumes each row has at least W chars; we preserve behavior by indexing bytes.
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
        for _ in 0..h {
            let s = it.next().unwrap_or("");
            grid.push(s.as_bytes().to_vec());
        }

        // Run DFS from every cell
        for i in 0..h {
            for j in 0..w {
                dfs(i, j, h, w, &grid, &mut area_id);
            }
        }

        // Mark seen component ids
        for i in 0..h {
            for j in 0..w {
                let idx = area_id[i][j] as usize;
                if idx < area_num.len() {
                    area_num[idx] = 1;
                }
            }
        }

        // Count unique ids among i*W+j positions
        for i in 0..h {
            for j in 0..w {
                if area_num[i * w + j] == 1 {
                    cnt += 1;
                }
            }
        }

        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
