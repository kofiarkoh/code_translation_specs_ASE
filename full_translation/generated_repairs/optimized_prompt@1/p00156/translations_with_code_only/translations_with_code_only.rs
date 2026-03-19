use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();

    let mut x = vec![0; 200000];
    let mut y = vec![0; 200000];

    let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while input.read_line(&mut line).unwrap() > 0 {
        let mut parts = line.split_whitespace();
        let w: usize = parts.next().unwrap().parse().unwrap();
        let h: usize = parts.next().unwrap().parse().unwrap();

        if w == 0 && h == 0 {
            break;
        }

        let mut c = vec![vec![0; 101]; 101];
        let mut m = vec![vec![' '; 101]; 101];
        let mut t = 0;

        for i in 0..h {
            line.clear();
            input.read_line(&mut line).unwrap();
            for j in 0..w {
                m[i][j] = line.chars().nth(j).unwrap();
                if m[i][j] == '&' {
                    m[i][j] = '.';
                    y[t] = i as i32;
                    x[t] = j as i32;
                    c[i][j] = 1;
                    t += 1;
                }
            }
        }

        let mut r = 1;
        while t > 0 {
            t -= 1;
            for (dx, dy) in &directions {
                let nx = x[t] + dx;
                let ny = y[t] + dy;

                if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                    continue;
                }

                if m[y[t] as usize][x[t] as usize] == m[ny as usize][nx as usize] {
                    if c[y[t] as usize][x[t] as usize] < c[ny as usize][nx as usize] || c[ny as usize][nx as usize] == 0 {
                        c[ny as usize][nx as usize] = c[y[t] as usize][x[t] as usize];
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                } else {
                    if c[y[t] as usize][x[t] as usize] + 1 < c[ny as usize][nx as usize] || c[ny as usize][nx as usize] == 0 {
                        c[ny as usize][nx as usize] = c[y[t] as usize][x[t] as usize] + 1;
                        x[r] = nx;
                        y[r] = ny;
                        r += 1;
                    }
                }
            }
        }

        let mut min = w as i32;
        for i in 0..w {
            if c[0][i] > 0 {
                min = min.min(c[0][i] / 2);
            }
            if c[h - 1][i] > 0 {
                min = min.min(c[h - 1][i] / 2);
            }
        }
        for i in 0..h {
            if c[i][0] > 0 {
                min = min.min(c[i][0] / 2);
            }
            if c[i][w - 1] > 0 {
                min = min.min(c[i][w - 1] / 2);
            }
        }
        println!("{}", min);
    }
}
