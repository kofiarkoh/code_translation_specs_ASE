use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..n {
        let mut s = [[0; 12]; 12];
        let mut a = [[0; 12]; 12];

        // Read the 10x10 grid
        for i in 0..10 {
            let line = lines.next().unwrap().unwrap();
            let values: Vec<i32> = line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for j in 0..10 {
                s[i][j] = values[j];
            }
        }

        // Process the grid
        for i in 0..9 {
            for j in 0..10 {
                if s[i][j] % 2 == 1 {
                    a[i + 1][j] = 1;
                    s[i][j] += 1;
                    s[i + 1][j] += 1;
                    if j > 0 {
                        s[i + 1][j - 1] += 1; // Check bounds for j-1
                    }
                    if j < 9 {
                        s[i + 1][j + 1] += 1; // Check bounds for j+1
                    }
                    s[i + 2][j] += 1;
                }
            }
        }

        // Print the result
        for i in 0..10 {
            for j in 0..10 {
                print!("{} ", a[i][j]);
            }
            println!();
        }
    }
}
