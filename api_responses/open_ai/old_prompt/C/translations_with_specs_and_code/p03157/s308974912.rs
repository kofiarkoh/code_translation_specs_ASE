use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

fn dfs(y: usize, x: usize, mikisinn: usize, group: &mut [[usize; MAX_SIZE]; MAX_SIZE], S: &Vec<Vec<char>>, H: usize, W: usize) {
    let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    if group[y][x] == mikisinn {
        return;
    }
    group[y][x] = mikisinn;

    for (dy, dx) in directions.iter() {
        let ny = y as isize + dy;
        let nx = x as isize + dx;
        if ny >= 0 && ny < H as isize && nx >= 0 && nx < W as isize {
            let ny = ny as usize;
            let nx = nx as usize;
            if S[y][x] != S[ny][nx] {
                dfs(ny, nx, mikisinn, group, S, H, W);
            }
        }
    }
}

fn main() -> Result<(), i32> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().ok_or(1)??;
    let mut dimensions = first_line.split_whitespace();
    let H: usize = dimensions.next().ok_or(1)?.parse().map_err(|_| 1)?;
    let W: usize = dimensions.next().ok_or(1)?.parse().map_err(|_| 1)?;

    if H < 1 || H > MAX_SIZE || W < 1 || W > MAX_SIZE {
        return Err(1);
    }

    let mut S: Vec<Vec<char>> = Vec::with_capacity(H);
    for _ in 0..H {
        let line = lines.next().ok_or(1)??;
        if line.len() != W {
            return Err(1);
        }
        S.push(line.chars().collect());
    }

    let mut group = [[0; MAX_SIZE]; MAX_SIZE];
    let mut count = [[0; 2]; MAX_SIZE * MAX_SIZE];
    let mut asumisu = 0;
    let mut mizuhasu: u64 = 0;

    for i in 0..H {
        for j in 0..W {
            if group[i][j] == 0 {
                dfs(i, j, asumisu + 1, &mut group, &S, H, W);
                asumisu += 1;
            }
        }
    }

    for i in 0..H {
        for j in 0..W {
            count[group[i][j]][(S[i][j] == '.') as usize] += 1;
        }
    }

    for i in 1..=asumisu {
        mizuhasu += (count[i][0] as u64) * (count[i][1] as u64);
    }

    println!("{}", mizuhasu);
    Ok(())
}
