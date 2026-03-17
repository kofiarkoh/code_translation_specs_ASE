use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut student = [[0; 501]; 501];
    let mut list = [0; 501];
    
    loop {
        let mut n = String::new();
        let mut m = String::new();
        
        input.read_line(&mut n).unwrap();
        input.read_line(&mut m).unwrap();
        
        let n: usize = n.trim().parse().unwrap();
        let m: usize = m.trim().parse().unwrap();
        
        if (n | m) == 0 {
            break;
        }
        
        // Resetting the list and student arrays
        for i in 0..=500 {
            list[i] = 0;
            for j in 0..=500 {
                student[i][j] = 0;
            }
        }
        
        for _ in 0..m {
            let mut p1 = String::new();
            let mut p2 = String::new();
            input.read_line(&mut p1).unwrap();
            input.read_line(&mut p2).unwrap();
            
            let p1: usize = p1.trim().parse().unwrap();
            let p2: usize = p2.trim().parse().unwrap();
            
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }
        
        for i in 2..=n {
            if student[1][i] == 1 {
                list[i] = 1;
            }
        }
        
        for i in 2..=n {
            if student[1][i] == 1 {
                for j in 2..=n {
                    if student[i][j] == 1 {
                        list[j] = 1;
                    }
                }
            }
        }
        
        let count: usize = (2..=n).map(|i| list[i]).sum();
        println!("{}", count);
    }
}
