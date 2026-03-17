use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();

    while handle.read_line(&mut input).unwrap() > 0 {
        let s = input.trim();
        if s.is_empty() || s.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = s.split('.').collect();
        let mut d = vec![0; 5];
        let mut r = 0;

        for part in parts.iter() {
            if let Ok(num) = part.parse::<i32>() {
                d[r] = num;
                r += 1;
            }
        }

        let mut dt: u64 = 0;
        let m12 = 365 * 13 + 4 - 11;
        let mut a = vec![0; 5];
        let m = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if r == 3 {
            for i in 1..d[1] {
                dt += m[i];
            }
            if d[1] > 2 && (d[0] % 4 == 0 && (d[0] % 100 != 0 || d[0] % 400 == 0)) {
                dt += 1;
            }
            dt += (d[2] - 1) as u64;
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

            for i in 1..13 {
                if dt <= m[i] {
                    break;
                }
                dt -= m[i];
                if i == 2 && a[0] % 4 == 0 && (a[0] % 100 != 0 || a[0] % 400 == 0) {
                    dt -= 1;
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
            }
            println!("{}.{}.{}", a[0], i, dt);
        }

        input.clear(); // Clear input for the next iteration
    }
}
