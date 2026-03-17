use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut student = [[0; 501]; 501];
    let mut list = [0; 501];
    
    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap_or(0);
        
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let m: i32 = input.trim().parse().unwrap_or(0);
        
        if (n | m) == 0 {
            break;
        }
        
        for i in 0..=500 {
            list[i] = 0;
            for j in 0..=500 {
                student[i][j] = 0;
            }
        }
        
        for _ in 0..m {
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            let nums: Vec<i32> = input.split_whitespace()
                                       .filter_map(|s| s.parse().ok())
                                       .collect();
            if nums.len() < 2 {
                continue; // Skip if not enough numbers
            }
            let p1 = nums[0];
            let p2 = nums[1];
            student[p1 as usize][p2 as usize] = 1;
            student[p2 as usize][p1 as usize] = 1;
        }
        
        for i in 2..=n {
            if student[1][i as usize] == 1 {
                list[i as usize] = 1;
            }
        }
        
        let mut visited = vec![false; (n + 1) as usize];
        let mut count = 0;

        for i in 2..=n {
            if student[1][i as usize] == 1 && !visited[i as usize] {
                count += 1;
                visited[i as usize] = true;
                for j in 2..=n {
                    if student[i as usize][j as usize] == 1 {
                        visited[j as usize] = true;
                    }
                }
            }
        }
        
        println!("{}", count);
    }
}
