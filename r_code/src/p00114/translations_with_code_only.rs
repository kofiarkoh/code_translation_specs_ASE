use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while a != 0 {
        let t = a;
        a = b % a;
        b = t;
    }
    b
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok());

        let a1 = nums.next().unwrap_or(0);
        let m1 = nums.next().unwrap_or(0);
        let a2 = nums.next().unwrap_or(0);
        let m2 = nums.next().unwrap_or(0);
        let a3 = nums.next().unwrap_or(0);
        let m3 = nums.next().unwrap_or(0);

        if a1 == 0 && m1 == 0 && a2 == 0 && m2 == 0 && a3 == 0 && m3 == 0 {
            break;
        }

        let mut c1 = 1;
        let mut r1 = a1;
        while r1 != 1 {
            r1 = r1 * a1 % m1;
            c1 += 1;
        }

        let mut c2 = 1;
        let mut r2 = a2;
        while r2 != 1 {
            r2 = r2 * a2 % m2;
            c2 += 1;
        }

        let mut c3 = 1;
        let mut r3 = a3;
        while r3 != 1 {
            r3 = r3 * a3 % m3;
            c3 += 1;
        }

        println!("{}", lcm(lcm(c1, c2), c3));
    }
}
