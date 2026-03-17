use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

fn dfs(y: usize, x: usize, group_id: usize, grid: &Vec<Vec<char>>, group: &mut Vec<Vec<usize>>) {
    static DIRECTION: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    if group[y][x] == group_id {
        return;
    }
    
    group[y][x] = group_id;
    
    for &(dy, dx) in &DIRECTION {
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

    if height == 0 || width == 0 || height > MAX_SIZE || width > MAX_SIZE {
        return Err("Height and width must be positive and within limits".to_string());
    }

    let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
    let mut group: Vec<Vec<usize>> = vec![vec![0; width]; height];
    let mut count = vec![(0, 0); height * width];

    for _ in 0..height {
        let line = lines.next().ok_or("Failed to read grid row")??;
        if line.len() != width {
            return Err("Grid row does not match specified width".to_string());
        }
        grid.push(line.chars().collect());
    }

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
            let idx = group[i][j] - 1; // Adjust for 0-based index
            if grid[i][j] == '.' {
                count[idx].0 += 1;
            } else {
                count[idx].1 += 1;
            }
        }
    }

    let mut total_pairs: u64 = 0;

    for (dot_count, other_count) in count.iter() {
        total_pairs += (*dot_count as u64) * (*other_count as u64);
    }

    println!("{}", total_pairs);
    Ok(())
}
