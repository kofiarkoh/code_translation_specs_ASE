use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = String::new();
    while reader.read_line(&mut input).unwrap() > 0 {
        let mut dimensions = input.split_whitespace();
        let w: usize = dimensions.next().unwrap().parse().unwrap();
        let h: usize = dimensions.next().unwrap().parse().unwrap();

        if w == 0 && h == 0 {
            break;
        }

        let mut m = vec![vec![' '; w]; h];
        let mut c = vec![vec![0; w]; h];
        let mut x = vec![0; 200000];
        let mut y = vec![0; 200000];
        let mut t = 0;
        let mut r = 1;

        for i in 0..h {
            input.clear();
            reader.read_line(&mut input).unwrap();
            for (j, ch) in input.trim().chars().enumerate() {
                m[i][j] = ch;
                if ch == '&' {
                    m[i][j] = '.';
                    y[0] = i;
                    x[t] = j;
                    c[i][j] = 1;
                }
            }
        }

        while t < r {
            for i in 0..4 {
                let nx = x[t] + [0, 1, 0, -1][i];
                let ny = y[t] + [1, 0, -1, 0][i];

                if nx < 0 || nx >= w || ny < 0 || ny >= h {
                    continue;
                }

                if m[y[t]][x[t]] == m[ny][nx] {
                    if c[y[t]][x[t]] < c[ny][nx] || c[ny][nx] == 0 {
                        c[ny][nx] = c[y[t]][x[t]];
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                } else {
                    if c[y[t]][x[t]] + 1 < c[ny][nx] || c[ny][nx] == 0 {
                        c[ny][nx] = c[y[t]][x[t]] + 1;
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                }
            }
            t += 1;
        }

        let mut min = w;
        for i in 0..w {
            if c[0][i] / 2 < min {
                min = c[0][i] / 2;
            }
            if c[h - 1][i] / 2 < min {
                min = c[h - 1][i] / 2;
            }
        }
        for i in 0..h {
            if c[i][0] / 2 < min {
                min = c[i][0] / 2;
            }
            if c[i][w - 1] / 2 < min {
                min = c[i][w - 1] / 2;
            }
        }

        println!("{}", min);
        input.clear();
    }
}
