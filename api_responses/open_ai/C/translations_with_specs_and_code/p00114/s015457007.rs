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
    (a * b) / gcd(a, b)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 6 {
            continue; // Skip lines that do not have exactly 6 integers
        }

        let (a1, m1, a2, m2, a3, m3) = (nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);

        // Check for termination condition
        if a1 == 0 && m1 == 0 && a2 == 0 && m2 == 0 && a3 == 0 && m3 == 0 {
            break;
        }

        let mut c1 = 1;
        let mut r1 = a1;
        while r1 != 1 {
            r1 = (r1 * a1) % m1;
            c1 += 1;
        }

        let mut c2 = 1;
        let mut r2 = a2;
        while r2 != 1 {
            r2 = (r2 * a2) % m2;
            c2 += 1;
        }

        let mut c3 = 1;
        let mut r3 = a3;
        while r3 != 1 {
            r3 = (r3 * a3) % m3;
            c3 += 1;
        }

        let result = lcm(lcm(c1, c2), c3);
        println!("{}", result);
    }
}
