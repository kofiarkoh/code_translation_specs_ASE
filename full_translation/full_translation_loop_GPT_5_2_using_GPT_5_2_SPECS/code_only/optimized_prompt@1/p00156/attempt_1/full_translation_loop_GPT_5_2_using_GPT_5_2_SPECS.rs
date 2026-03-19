use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse like scanf would (whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Global arrays in C: int x[200000], y[200000];
    // Keep them across test cases; C reuses them without clearing.
    let mut x = vec![0i32; 200_000];
    let mut y = vec![0i32; 200_000];

    let dx = [0i32, 1, 0, -1];
    let dy = [1i32, 0, -1, 0];

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();
        let h: i32 = it.next().unwrap().parse().unwrap();

        if w == 0 && h == 0 {
            break;
        }

        // C: int c[101][101] = {0};
        let mut c = vec![vec![0i32; 101]; 101];
        // C: char m[101][101];
        let mut m = vec![vec!['\0'; 101]; 101];

        // C uses `t` uninitialized until it finds '&' and sets `x[t=0]`.
        // We mirror the intended behavior by initializing t=0, and only using it
        // after '&' is found (as the C code effectively does).
        let mut t: i32 = 0;

        for i in 0..h {
            for j in 0..w {
                let tok = it.next().unwrap();
                let ch = tok.chars().next().unwrap();
                m[i as usize][j as usize] = ch;
                if ch == '&' {
                    m[i as usize][j as usize] = '.';
                    y[0] = i;
                    t = 0;
                    x[t as usize] = j;
                    c[i as usize][j as usize] = 1;
                }
            }
        }

        // C: for(r=1; r-t; t++) { ... }  (condition is r - t != 0)
        let mut r: i32 = 1;
        while r - t != 0 {
            for dir in 0..4 {
                let nx = x[t as usize] + dx[dir];
                let ny = y[t as usize] + dy[dir];

                if nx < 0 || w <= nx || ny < 0 || h <= ny {
                    continue;
                }

                let cx = x[t as usize] as usize;
                let cy = y[t as usize] as usize;
                let nxu = nx as usize;
                let nyu = ny as usize;

                if m[cy][cx] == m[nyu][nxu] {
                    if c[cy][cx] < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = c[cy][cx];
                        x[r as usize] = nx;
                        y[r as usize] = ny;
                        r += 1;
                    }
                } else {
                    let cand = c[cy][cx] + 1;
                    if cand < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = cand;
                        x[r as usize] = nx;
                        y[r as usize] = ny;
                        r += 1;
                    }
                }
            }
            t += 1;
        }

        // C: min = w; then checks borders with integer division by 2.
        let mut minv: i32 = w;

        for i in 0..w {
            let top = c[0][i as usize] / 2;
            if top < minv {
                minv = top;
            }
            let bottom = c[(h - 1) as usize][i as usize] / 2;
            if bottom < minv {
                minv = bottom;
            }
        }
        for i in 0..h {
            let left = c[i as usize][0] / 2;
            if left < minv {
                minv = left;
            }
            let right = c[i as usize][(w - 1) as usize] / 2;
            if right < minv {
                minv = right;
            }
        }

        out.push_str(&format!("{minv}\n"));
    }

    print!("{out}");
}
