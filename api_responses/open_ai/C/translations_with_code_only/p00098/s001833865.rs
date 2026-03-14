// AOJ 0098: Maximum Sum Sequence II 
// 2017.8.15 bal4u@uu
// 2017.11.1 retry

use std::io::{self, BufRead};

const N: usize = 102;

fn get_int(p: &mut &str) -> i32 {
    let mut n = 0;
    while let Some(c) = p.chars().next() {
        if c >= '0' {
            n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
            *p = &p[1..]; // Move the pointer forward
        } else {
            break;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut buf = String::new();
    
    handle.read_line(&mut buf).unwrap();
    let mut p = &buf[..];
    let n = get_int(&mut p);
    
    let mut a = [[0; N]; N];
    let mut s = [[0; N]; N];

    for r in 0..n {
        buf.clear();
        handle.read_line(&mut buf).unwrap();
        p = &buf[..];
        for c in 0..n {
            if p.chars().next() == Some('-') {
                p = &p[1..]; // Skip the '-'
                a[r][c] = -get_int(&mut p);
            } else {
                a[r][c] = get_int(&mut p);
            }
        }
    }

    for r in 0..n {
        for c in 0..n {
            s[r][c + 1] = s[r][c] + a[r][c];
        }
    }

    let mut ans = s[0][1];
    for c in 0..n {
        for k in c + 1..=n {
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
