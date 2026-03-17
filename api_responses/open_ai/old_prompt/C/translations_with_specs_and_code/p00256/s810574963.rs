use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut s = String::new();

    while reader.read_line(&mut s).unwrap() > 0 {
        if s.chars().next().unwrap() == '#' {
            break;
        }

        let parts: Vec<&str> = s.trim().split('.').collect();
        let mut d: Vec<i32> = parts.iter().map(|&x| x.parse().unwrap()).collect();
        let mut a = [0; 5];
        let mut dt: u64 = 0;

        if d.len() == 3 {
            let m = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            for i in 1..d[1] {
                dt += m[i as usize];
            }
            if d[1] > 2 && (d[0] % 4 == 0 && (d[0] % 100 != 0 || d[0] % 400 == 0)) {
                dt += 1;
            }
            dt += (d[2] - 1) as u64;
            d[0] -= 2001;
            dt += (365 * d[0] as u64) + (d[0] / 4) as u64 - (d[0] / 100) as u64 + (d[0] / 400) as u64 + 365 - 11;

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

            for a0 in 2000.. {
                let mut t = dt;
                dt -= 365;
                if a0 % 4 == 0 && (a0 % 100 != 0 || a0 % 400 == 0) {
                    dt -= 1;
                }
                if dt <= 0 {
                    a[0] = a0;
                    if dt == 0 {
                        dt = t;
                        a[0] -= 1;
                    }
                    break;
                }
            }

            let mut month = 1;
            let m = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            while month < 13 {
                if dt <= m[month] {
                    break;
                }
                dt -= m[month];
                if month == 2 && (a[0] % 4 == 0 && (a[0] % 100 != 0 || a[0] % 400 == 0)) {
                    dt -= 1;
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
                month += 1;
            }
            println!("{}.{}.{}", a[0], month, dt);
        }

        s.clear(); // Clear the string for the next input
    }
}
