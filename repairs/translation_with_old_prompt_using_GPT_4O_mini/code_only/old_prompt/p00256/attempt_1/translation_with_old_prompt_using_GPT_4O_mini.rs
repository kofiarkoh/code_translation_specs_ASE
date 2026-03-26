use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut s = String::new();
    let mut d = [0; 5];
    let mut a = [0; 5];
    let m12 = 365 * 13 + 4 - 11;
    let d400 = 365 * 400 + 100 - 4 + 1;
    let d100 = 365 * 100 + 25 - 1;
    let d4 = 365 * 4 + 1;
    let M = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    while stdin.lock().read_line(&mut buffer).unwrap() > 0 {
        s = buffer.trim().to_string();
        if s.chars().next().unwrap() == '#' {
            break;
        }

        let mut c = 0;
        let mut n = 0;
        let mut r = 0;

        for ch in s.chars() {
            if ch == '.' {
                c += 1;
                d[r] = n;
                r += 1;
                n = 0;
            } else {
                n = n * 10 + (ch as u8 - b'0') as i32;
            }
        }
        d[r] = n;

        let mut dt: u64 = 0; // -m12;
        if c == 2 {
            for i in 1..d[1] {
                dt += M[i];
            }
            if d[1] > 2 && d[0] % 4 == 0 && (d[0] % 100 != 0 || d[0] % 400 == 0) {
                dt += 1;
            }
            dt += d[2] as u64 - 1;
            dt += (365 * (d[0] - 2001) + (d[0] - 2001) / 4 - (d[0] - 2001) / 100 + (d[0] - 2001) / 400 + 366 - m12) as u64;

            a[4] = (dt % 20) as i32;
            dt /= 20;
            a[3] = (dt % 18) as i32;
            dt /= 18;
            a[2] = (dt % 20) as i32;
            dt /= 20;
            a[1] = (dt % 20) as i32;
            dt /= 20;
            a[0] = (dt % 13) as i32;

            println!("{}.{}.{}.{}.{}", a[0], a[1], a[2], a[3], a[4]);
        } else {
            dt = d[0] as u64;
            dt = dt * 20 + d[1] as u64;
            dt = dt * 20 + d[2] as u64;
            dt = dt * 18 + d[3] as u64;
            dt = dt * 20 + d[4] as u64;
            dt += 4738 + 1;

            a[0] = 2000;
            while dt > 365 {
                let t = dt;
                dt -= 365;
                if a[0] % 4 == 0 && (a[0] % 100 != 0 || a[0] % 400 == 0) {
                    dt -= 1;
                }
                a[0] += 1;
            }
            if dt == 0 {
                dt = t;
                a[0] -= 1;
            }

            let mut i = 1;
            while i < 13 {
                if dt <= M[i] {
                    break;
                }
                dt -= M[i];
                if i == 2 && a[0] % 4 == 0 && (a[0] % 100 != 0 || a[0] % 400 == 0) {
                    dt -= 1;
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
                i += 1;
            }
            println!("{}.{}.{}", a[0], i, dt);
        }
        buffer.clear();
    }
}
