use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

fn dfs(y: usize, x: usize, group_id: usize, grid: &Vec<Vec<char>>, group: &mut Vec<Vec<usize>>) {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    if group[y][x] == group_id {
        return;
    }
    group[y][x] = group_id;

    for (dy, dx) in &directions {
        let ny = y as isize + dy;
        let nx = x as isize + dx;
        if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize {
            let ny = ny as usize;
            let nx = nx as usize;
            if grid[y][x] != grid[ny][nx] {
                dfs(ny, nx, group_id, grid, group);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().ok_or("Failed to read height and width")??;
    let mut dimensions = first_line.split_whitespace();
    let height: usize = dimensions.next().ok_or("Missing height")?.parse().map_err(|_| "Invalid height")?;
    let width: usize = dimensions.next().ok_or("Missing width")?.parse().map_err(|_| "Invalid width")?;

    if height > MAX_SIZE || width > MAX_SIZE {
        return Err("Height or width exceeds maximum size".to_string());
    }

    let mut grid = Vec::with_capacity(height);
    for _ in 0..height {
        let line = lines.next().ok_or("Failed to read grid row")??;
        if line.len() != width {
            return Err("Grid row does not match expected width".to_string());
        }
        grid.push(line.chars().collect());
    }

    let mut group = vec![vec![0; width]; height];
    let mut count = vec![[0; 2]; height * width];
    let mut group_id = 0;

    for i in 0..height {
        for j in 0..width {
            if group[i][j] == 0 {
                group_id += 1;
                dfs(i, j, group_id, &grid, &mut group);
            }
        }
    }

    for i in 0..height {
        for j in 0..width {
            count[group[i][j] - 1][(grid[i][j] == '.') as usize] += 1;
        }
    }

    let mut total_pairs: u64 = 0;
    for i in 0..group_id {
        total_pairs += count[i][0] as u64 * count[i][1] as u64;
    }

    println!("{}", total_pairs);
    Ok(())
}
