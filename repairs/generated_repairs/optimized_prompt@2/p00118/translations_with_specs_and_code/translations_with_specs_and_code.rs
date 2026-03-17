use std::io::{self, BufRead};

const MAX_SIZE: usize = 128;

fn dfs(i: usize, j: usize, h: usize, w: usize, grid: &Vec<Vec<char>>, area_id: &mut Vec<Vec<usize>>, current_area: usize) {
    if i >= h || j >= w || area_id[i][j] != i * w + j || grid[i][j] != grid[area_id[i][j] / w][area_id[i][j] % w] {
        return;
    }

    area_id[i][j] = current_area;

    // Explore adjacent cells
    if i + 1 < h {
        dfs(i + 1, j, h, w, grid, area_id, current_area);
    }
    if i > 0 {
        dfs(i - 1, j, h, w, grid, area_id, current_area);
    }
    if j + 1 < w {
        dfs(i, j + 1, h, w, grid, area_id, current_area);
    }
    if j > 0 {
        dfs(i, j - 1, h, w, grid, area_id, current_area);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut area_id = vec![vec![0; MAX_SIZE]; MAX_SIZE];
    let _area_num = vec![0; MAX_SIZE * MAX_SIZE]; // Changed to unused variable

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace();
        let h: usize = dimensions.next().unwrap().parse().unwrap();
        let w: usize = dimensions.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        // Initialize area_id
        for i in 0..h {
            for j in 0..w {
                area_id[i][j] = i * w + j;
            }
        }

        // Read the grid
        let mut grid = Vec::new();
        for _ in 0..h {
            let mut row = String::new();
            reader.read_line(&mut row).unwrap(); // Changed to unwrap the result
            grid.push(row.trim().chars().collect());
        }

        // Count distinct areas
        let mut cnt = 0;

        for i in 0..h {
            for j in 0..w {
                if area_id[i][j] == i * w + j {
                    dfs(i, j, h, w, &grid, &mut area_id, area_id[i][j]);
                    cnt += 1;
                }
            }
        }

        // Output the count of distinct areas
        println!("{}", cnt);
    }
}
