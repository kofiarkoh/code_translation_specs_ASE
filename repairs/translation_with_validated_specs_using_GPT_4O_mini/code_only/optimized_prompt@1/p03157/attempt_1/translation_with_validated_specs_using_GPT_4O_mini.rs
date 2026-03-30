use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

static mut H: usize = 0;
static mut W: usize = 0;
static mut S: [[char; MAX_SIZE]; MAX_SIZE] = [['\0'; MAX_SIZE]; MAX_SIZE];

static mut GROUP: [[usize; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];
static mut COUNT: [[usize; 2]; MAX_SIZE * MAX_SIZE] = [[0; 2]; MAX_SIZE * MAX_SIZE];

fn dfs(y: usize, x: usize, mikisinn: usize) {
    static D: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    unsafe {
        if GROUP[y][x] == mikisinn {
            return;
        }
        GROUP[y][x] = mikisinn;

        for &(dy, dx) in &D {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;

            if ny < H && nx < W && S[y][x] != S[ny][nx] {
                dfs(ny, nx, mikisinn);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    unsafe {
        let first_line = lines.next().unwrap().unwrap();
        let mut dimensions = first_line.split_whitespace();
        H = dimensions.next().unwrap().parse().unwrap();
        W = dimensions.next().unwrap().parse().unwrap();

        for i in 0..H {
            let line = lines.next().unwrap().unwrap();
            S[i].copy_from_slice(&line.chars().collect::<Vec<_>>()[..W]);
        }

        let mut asumisu = 0;
        for i in 0..H {
            for j in 0..W {
                if GROUP[i][j] == 0 {
                    asumisu += 1;
                    dfs(i, j, asumisu);
                }
            }
        }

        let mut mizuhasu: u64 = 0;
        for i in 1..=asumisu {
            for j in 0..H {
                for k in 0..W {
                    if GROUP[j][k] == i {
                        COUNT[i][(S[j][k] == '.') as usize] += 1;
                    }
                }
            }
            mizuhasu += (COUNT[i][0] as u64) * (COUNT[i][1] as u64);
        }

        println!("{}", mizuhasu);
    }
}
