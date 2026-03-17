use std::io::{self, BufRead};

const MAX_SIZE: usize = 512;

static mut GROUP: [[i32; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];
static mut COUNT: [[i32; 2]; MAX_SIZE * MAX_SIZE] = [[0; 2]; MAX_SIZE * MAX_SIZE];

fn dfs(y: usize, x: usize, mikisinn: i32, h: usize, w: usize, s: &Vec<Vec<char>>) {
    static D: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    
    unsafe {
        if GROUP[y][x] == mikisinn {
            return;
        }
        GROUP[y][x] = mikisinn;

        for &(dy, dx) in &D {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;
            if ny < h && nx < w && s[y][x] != s[ny][nx] {
                dfs(ny, nx, mikisinn, h, w, s);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    let mut s: Vec<Vec<char>> = Vec::with_capacity(h);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        s.push(line.chars().collect());
    }

    let mut asumisu = 0;

    for i in 0..h {
        for j in 0..w {
            unsafe {
                if GROUP[i][j] == 0 {
                    asumisu += 1;
                    dfs(i, j, asumisu, h, w, &s);
                }
            }
        }
    }

    let mut mizuhasu: u64 = 0;

    for i in 1..=asumisu {
        unsafe {
            mizuhasu += (COUNT[i as usize][0] as u64) * (COUNT[i as usize][1] as u64);
        }
    }

    println!("{}", mizuhasu);
}
