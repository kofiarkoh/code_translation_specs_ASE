use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();

    while handle.read_line(&mut input).unwrap() > 0 {
        let input = input.trim();
        if input.is_empty() || input.starts_with('#') {
            break;
        }

        let parts: Vec<&str> = input.split('.').collect();
        let count = parts.len();
        let mut d: Vec<i32> = parts.iter().map(|&s| s.parse().unwrap()).collect();
        let mut dt: u64 = 0;
        let m12 = 365 * 13 + 4 - 11;

        if count == 3 {
            // Processing "YYYY.MM.DD" format
            let year = d[0];
            let month = d[1];
            let day = d[2];

            for i in 1..month {
                dt += MONTH_DAYS[i as usize] as u64;
            }
            if month > 2 && is_leap_year(year) {
                dt += 1;
            }
            dt += (day - 1) as u64;
            dt += (year - 2001) as u64 * 365 + (year - 2001) as u64 / 4 - (year - 2001) as u64 / 100 + (year - 2001) as u64 / 400 + 366 - m12;

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
            // Processing custom encoded format "DD.DD.DD.DD.DD"
            dt = d[0] as u64;
            dt = dt * 20 + d[1] as u64;
            dt = dt * 20 + d[2] as u64;
            dt = dt * 18 + d[3] as u64;
            dt = dt * 20 + d[4] as u64;
            dt += 4738 + 1;

            let mut year = 2000;
            while dt > 365 {
                let t = dt;
                dt -= 365;
                if is_leap_year(year) {
                    dt -= 1;
                }
                year += 1;
            }
            if dt == 0 {
                dt = t;
                year -= 1;
            }

            let mut month = 1;
            while dt > MONTH_DAYS[month as usize] as u64 {
                dt -= MONTH_DAYS[month as usize] as u64;
                if month == 2 && is_leap_year(year) {
                    dt -= 1;
                    if dt == 0 {
                        dt = 29;
                        break;
                    }
                }
                month += 1;
            }
            println!("{}.{}.{}", year, month, dt);
        }
        input.clear();
    }
}

const MONTH_DAYS: [i32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
