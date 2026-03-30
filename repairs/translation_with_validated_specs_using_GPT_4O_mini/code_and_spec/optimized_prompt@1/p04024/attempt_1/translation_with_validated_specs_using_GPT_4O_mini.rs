use std::io::{self, BufRead};

const MODULO: i64 = 1_000_000_007;

fn mod_value(a: i64) -> i64 {
    let mut a = a % MODULO;
    if a < 0 {
        a += MODULO;
    }
    a
}

fn power(a: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let half = power(a, n / 2);
    if n % 2 == 0 {
        mod_value(half * half)
    } else {
        mod_value(a * half)
    }
}

fn inverse(a: i64) -> i64 {
    power(a, MODULO - 2)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let k: i64 = params.next().unwrap().parse().unwrap();

    let mut grid = Vec::with_capacity(h);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }

    if k == 0 {
        println!("1");
        return;
    }

    let mut f_yoko = false;
    let mut f_tate = false;

    for i in 0..h {
        if grid[i][0] == '#' && grid[i][w - 1] == '#' {
            f_yoko = true;
        }
    }

    for j in 0..w {
        if grid[0][j] == '#' && grid[h - 1][j] == '#' {
            f_tate = true;
        }
    }

    if f_yoko && f_tate {
        println!("1");
        return;
    }

    let (mut h, mut w) = (h, w);
    if f_tate {
        // Transpose the grid
        let mut transposed = vec![vec![' '; h]; w];
        for i in 0..h {
            for j in 0..w {
                transposed[j][i] = grid[i][j];
            }
        }
        grid = transposed;
        std::mem::swap(&mut h, &mut w);
    }

    let mut x = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for i in 0..h {
        let mut x_sub = 0;
        for j in 0..w {
            if grid[i][j] == '#' {
                x_sub += 1;
            }
        }

        let mut e = 0;
        for j in 1..w {
            if grid[i][j - 1] == '#' && grid[i][j] == '#' {
                e += 1;
            }
        }

        if x_sub == w {
            c += 1;
        } else {
            a += x_sub - e;
            if grid[i][0] == '#' && grid[i][w - 1] == '#' {
                a -= 2;
                b += 1;
            }
        }
        x += x_sub;
    }

    if !f_yoko && !f_tate {
        println!("{}", power(x, k - 1));
    } else if a == 0 && b == 0 {
        println!("{}", power(c, k - 1));
    } else {
        let result = mod_value(mod_value((a + b) * power(x, k - 1) + (x - (a + 2 * b + c)) * power(b + c, k - 1)) * inverse(x - (b + c)));
        println!("{}", result);
    }
}
