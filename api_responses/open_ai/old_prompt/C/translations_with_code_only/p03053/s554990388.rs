use std::io::{self, BufRead};
use std::cmp::{min, max};

const INF: i32 = 1001001001;
const MAX_SIZE: usize = 1001;
const QUEUE_SIZE: usize = 1001001;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let h: usize = dimensions.next().unwrap().parse().unwrap();
    let w: usize = dimensions.next().unwrap().parse().unwrap();

    let mut b = [[INF; MAX_SIZE]; MAX_SIZE];
    let mut a = [[' '; MAX_SIZE]; MAX_SIZE];
    let mut que = vec![0; QUEUE_SIZE];
    let mut l = 0;
    let mut r = 0;
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let mut ans = 0;

    for i in 0..h {
        let line = lines.next().unwrap().unwrap();
        for j in 0..w {
            a[i][j] = line.chars().nth(j).unwrap();
            b[i][j] = INF;
            if a[i][j] == '#' {
                b[i][j] = 0;
                que[r] = i * 10000 + j;
                r = (r + 1) % QUEUE_SIZE;
            }
        }
    }

    while r != l {
        let x = que[l] / 10000;
        let y = que[l] % 10000;
        l = (l + 1) % QUEUE_SIZE;

        for i in 0..4 {
            let nx = x + dx[i];
            let ny = y + dy[i];
            if nx < h && ny < w && a[nx][ny] == '.' && b[nx][ny] > b[x][y] + 1 {
                b[nx][ny] = b[x][y] + 1;
                que[r] = nx * 10000 + ny;
                r = (r + 1) % QUEUE_SIZE;
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            ans = max(ans, b[i][j]);
        }
    }

    println!("{}", ans);
}
