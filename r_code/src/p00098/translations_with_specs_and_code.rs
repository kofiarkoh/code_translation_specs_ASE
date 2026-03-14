use std::io::{self, BufRead};

fn get_int(p: &mut std::str::Chars) -> i32 {
    let mut n = 0;
    while let Some(c) = p.next() {
        if c.is_digit(10) {
            n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        } else {
            break;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the size of the matrix
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut a = vec![vec![0; 102]; 102];
    let mut s = vec![vec![0; 103]; 102]; // s[r][c+1] requires an extra column

    // Read the matrix
    for r in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut p = line.chars();
        for c in 0..n {
            let first_char = p.next().unwrap();
            if first_char == '-' {
                a[r][c] = -get_int(&mut p);
            } else {
                a[r][c] = get_int(&mut std::iter::once(first_char).chain(p));
            }
        }
    }

    // Calculate cumulative sums
    for r in 0..n {
        for c in 0..n {
            s[r][c + 1] = s[r][c] + a[r][c];
        }
    }

    let mut ans = s[0][1]; // Initialize with the first value
    for c in 0..n {
        for k in (c + 1)..=n {
            let mut t = 0;
            for r in 0..n {
                if t < 0 {
                    t = s[r][k] - s[r][c];
                } else {
                    t += s[r][k] - s[r][c];
                }
                if t > ans {
                    ans = t;
                }
            }
        }
    }

    println!("{}", ans);
}
