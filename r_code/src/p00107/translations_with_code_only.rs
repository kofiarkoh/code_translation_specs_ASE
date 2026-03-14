use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    loop {
        let mut t = [0; 3];
        let mut line = String::new();
        
        // Read three integers
        if input.read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if nums.len() < 3 {
            continue; // Not enough numbers
        }
        
        t[0] = nums[0];
        t[1] = nums[1];
        t[2] = nums[2];

        if t[0] == 0 || t[1] == 0 || t[2] == 0 {
            break;
        }

        line.clear();
        input.read_line(&mut line).unwrap();
        let n: i32 = line.trim().parse().unwrap();

        if t[0] >= t[2] {
            t[0] = t[2];
            t[2] = -1;
        }
        if t[1] >= t[2] && t[2] != -1 {
            t[1] = t[2];
        }

        let hakari = t[0] * t[0] + t[1] * t[1];

        for _ in 0..n {
            line.clear();
            input.read_line(&mut line).unwrap();
            let mut data: i32 = line.trim().parse().unwrap();
            data *= 2;
            data *= data;

            if hakari < data {
                println!("OK");
            } else {
                println!("NA");
            }
        }
    }
}
