use std::io::{self, BufRead};

const MAX_SIZE: usize = 128;

fn dfs(i: usize, j: usize, h: usize, w: usize, grid: &Vec<Vec<char>>, area_id: &mut Vec<Vec<usize>>) {
    // Check down
    if i + 1 < h {
        if grid[i][j] == grid[i + 1][j] && area_id[i + 1][j] == (i + 1) * w + j {
            area_id[i + 1][j] = area_id[i][j];
            dfs(i + 1, j, h, w, grid, area_id);
        }
    }

    // Check up
    if i > 0 {
        if grid[i][j] == grid[i - 1][j] && area_id[i - 1][j] == (i - 1) * w + j {
            area_id[i - 1][j] = area_id[i][j];
            dfs(i - 1, j, h, w, grid, area_id);
        }
    }

    // Check right
    if j + 1 < w {
        if grid[i][j] == grid[i][j + 1] && area_id[i][j + 1] == i * w + (j + 1) {
            area_id[i][j + 1] = area_id[i][j];
            dfs(i, j + 1, h, w, grid, area_id);
        }
    }

    // Check left
    if j > 0 {
        if grid[i][j] == grid[i][j - 1] && area_id[i][j - 1] == i * w + (j - 1) {
            area_id[i][j - 1] = area_id[i][j];
            dfs(i, j - 1, h, w, grid, area_id);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().map(|line| line.unwrap());

    loop {
        let dimensions: Vec<usize> = input.next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let h = dimensions[0];
        let w = dimensions[1];

        if h == 0 && w == 0 {
            break;
        }

        let mut grid: Vec<Vec<char>> = vec![vec![' '; MAX_SIZE]; MAX_SIZE];
        let mut area_id: Vec<Vec<usize>> = vec![vec![0; MAX_SIZE]; MAX_SIZE];
        let mut area_num = vec![0; MAX_SIZE * MAX_SIZE];
        let mut cnt = 0;

        for i in 0..h {
            let line: String = input.next().unwrap();
            for j in 0..w {
                grid[i][j] = line.chars().nth(j).unwrap();
                area_id[i][j] = i * w + j;
            }
        }

        for i in 0..h {
            for j in 0..w {
                dfs(i, j, h, w, &grid, &mut area_id);
            }
        }

        for i in 0..h {
            for j in 0..w {
                area_num[area_id[i][j]] = 1;
            }
        }

        for i in 0..h {
            for j in 0..w {
                if area_num[i * w + j] == 1 {
                    cnt += 1;
                }
            }
        }

        println!("{}", cnt);
    }
}
