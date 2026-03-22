use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse like scanf would (whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Global arrays in C: int x[200000], y[200000];
    // Keep same capacity; indices r/t are within this bound for given constraints.
    let mut x = vec![0i32; 200_000];
    let mut y = vec![0i32; 200_000];

    let dx = [0i32, 1, 0, -1];
    let dy = [1i32, 0, -1, 0];

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

        // char m[101][101];
        let mut m = vec![vec!['\0'; 101]; 101];
        // int c[101][101] = {0};
        let mut c = vec![vec![0i32; 101]; 101];

        let mut t: usize = 0; // will be set when '&' found (as in C: x[t=0]=j)
        for i in 0..(h as usize) {
            for j in 0..(w as usize) {
                let tok = it.next().unwrap();
                let ch = tok.chars().next().unwrap(); // scanf(" %c") reads one non-ws char
                m[i][j] = ch;
                if m[i][j] == '&' {
                    m[i][j] = '.';
                    y[0] = i as i32;
                    t = 0;
                    x[t] = j as i32;
                    c[i][j] = 1;
                }
            }
        }

        // for(r=1; r-t; t++) { ... }  (queue-like traversal)
        let mut r: usize = 1;
        while r != t {
            let cx = x[t];
            let cy = y[t];
            for k in 0..4 {
                let nx = cx + dx[k];
                let ny = cy + dy[k];
                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }
                let nxu = nx as usize;
                let nyu = ny as usize;
                let cyu = cy as usize;
                let cxu = cx as usize;

                if m[cyu][cxu] == m[nyu][nxu] {
                    if c[cyu][cxu] < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = c[cyu][cxu];
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                } else {
                    let cand = c[cyu][cxu] + 1;
                    if cand < c[nyu][nxu] || c[nyu][nxu] == 0 {
                        c[nyu][nxu] = cand;
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                }
            }
            t += 1;
        }

        // min = w; then check borders with integer division by 2 (C int division).
        let mut minv: i32 = w;
        for i in 0..(w as usize) {
            let v1 = c[0][i] / 2;
            if v1 < minv {
                minv = v1;
            }
            let v2 = c[(h as usize) - 1][i] / 2;
            if v2 < minv {
                minv = v2;
            }
        }
        for i in 0..(h as usize) {
            let v1 = c[i][0] / 2;
            if v1 < minv {
                minv = v1;
            }
            let v2 = c[i][(w as usize) - 1] / 2;
            if v2 < minv {
                minv = v2;
            }
        }

        print!("{minv}\n");
    }
}
