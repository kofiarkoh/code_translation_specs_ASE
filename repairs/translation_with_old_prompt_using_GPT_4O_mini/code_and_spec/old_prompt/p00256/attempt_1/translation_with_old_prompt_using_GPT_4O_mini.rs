use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut s = String::new();

    while reader.read_line(&mut s).unwrap() > 0 {
        // Check for the '#' character at the start
        if s.chars().next().unwrap() == '#' {
            break;
        }

        // Trim the input string to remove any trailing newline
        let s = s.trim();
        let mut d = [0; 5];
        let mut n = 0;
        let mut r = 0;
        let mut c = 0;

        // Parse the input string
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

        let mut dt: u64 = 0;
        let m12 = 365 * 13 + 4 - 11;
        let m = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if c == 2 {
            for i in 1..d[1] {
                dt += m[i as usize] as u64;
            }
            if d[1] > 2 && (d[0] % 4 == 0 && (d[0] % 100 != 0 || d[0] % 400 == 0)) {
                dt += 1;
            }
            dt += (d[2] - 1) as u64;
            dt += (365 * (d[0] - 2001) as u64) + ((d[0] - 2001) / 4) as u64 - ((d[0] - 2001) / 100) as u64 + ((d[0] - 2001) / 400) as u64 + 366 - m12;

            let mut a = [0; 5];
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

            let mut a0 = 2000;
            while dt > 365 {
                let t = dt;
                dt -= 365;
                if a0 % 4 == 0 && (a0 % 100 != 0 || a0 % 400 == 0) {
                    dt -= 1;
                }
                a0 += 1;
            }

            if dt == 0 {
                dt = t;
                a0 -= 1;
            }

            let mut month = 1;
            for i in 1..13 {
                if dt <= m[i] {
                    month = i;
                    break;
                }
                dt -= m[i];
                if i == 2 && a0 % 4 == 0 && (a0 % 100 != 0 || a0 % 400 == 0) {
                    dt -= 1;
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
            }
            println!("{}.{}.{}", a0, month, dt);
        }

        // Clear the string for the next input
        s.clear();
    }
}
