use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(line) = input.next() {
        let line = line.unwrap();
        let dimensions: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (w, h) = (dimensions[0], dimensions[1]);
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
            let line = input.next().unwrap().unwrap();
            for j in 0..w {
                m[i][j] = line.chars().nth(j).unwrap();
                if m[i][j] == '&' {
                    m[i][j] = '.';
                    y[0] = i;
                    x[t] = j;
                    c[i][j] = 1;
                }
            }
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while t < r {
            for (dx, dy) in &directions {
                let nx = x[t] as isize + dx;
                let ny = y[t] as isize + dy;

                if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

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
    }
}
