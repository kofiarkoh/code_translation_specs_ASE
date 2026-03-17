use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut go = vec![0; 100_000]; // Equivalent to int go[100000];
    
    for line in stdin.lock().lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }
        
        // Read n integers into go
        for i in 0..n {
            go[i as usize] = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
        }
        
        // Process the array as per the C code logic
        for i in (1..n).step_by(2) {
            if go[i as usize - 1] != go[i as usize] {
                let mut j = i - 1;
                while j >= 0 && go[j as usize] != go[i as usize] {
                    go[j as usize] = go[i as usize];
                    if j == 0 { break; } // Prevent underflow
                    j -= 1;
                }
            }
        }
        
        // Count the number of zeros in go
        let cnt = (0..n).filter(|&i| go[i as usize] == 0).count();
        println!("{}", cnt);
    }
}
