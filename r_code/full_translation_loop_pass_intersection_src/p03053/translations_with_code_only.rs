use std::io::{self, Read};

const INF: i32 = 1001001001;
const QSIZE: usize = 1001001;

fn main() {
    // Read entire stdin and tokenize like scanf("%d%d") / scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    // a: grid of bytes, b: distances
    let mut a: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
    let mut b: Vec<Vec<i32>> = vec![vec![INF; w]; h];

    // Ring queue storing x*10000 + y, matching the C encoding.
    let mut que: Vec<i32> = vec![0; QSIZE];
    let mut l: usize = 0;
    let mut r: usize = 0;

    for i in 0..h {
        let s = it.next().unwrap_or("");
        let bytes = s.as_bytes();
        // C code assumes the string length is at least w; we mirror by reading what we got.
        for j in 0..w {
            let ch = bytes[j];
            a[i][j] = ch;
            b[i][j] = INF;
            if ch == b'#' {
                b[i][j] = 0;
                que[r] = (i as i32) * 10000 + (j as i32);
                r = (r + 1) % QSIZE;
            }
        }
    }

    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];

    while r != l {
        let v = que[l];
        let x = (v / 10000) as i32;
        let y = (v % 10000) as i32;
        l = (l + 1) % QSIZE;

        for k in 0..4 {
            let nx = x + dx[k];
            let ny = y + dy[k];
            if 0 <= nx
                && (nx as usize) < h
                && 0 <= ny
                && (ny as usize) < w
                && a[nx as usize][ny as usize] == b'.'
                && b[nx as usize][ny as usize] > b[x as usize][y as usize] + 1
            {
                b[nx as usize][ny as usize] = b[x as usize][y as usize] + 1;
                que[r] = nx * 10000 + ny;
                r = (r + 1) % QSIZE;
            }
        }
    }

    let mut ans: i32 = 0;
    for i in 0..h {
        for j in 0..w {
            if b[i][j] > ans {
                ans = b[i][j];
            }
        }
    }

    print!("{ans}\n");
}
