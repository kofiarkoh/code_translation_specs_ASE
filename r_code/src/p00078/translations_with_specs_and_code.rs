use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut magic = vec![vec![0; 16]; 16]; // Fixed size 16x16 grid
        let hn = n / 2;
        let mut x = hn;
        let mut y = (hn + 1) % n; // Start one row above the middle

        let mut number = 1;

        while number <= n * n {
            if magic[y][x] != 0 {
                // If the position is already filled, move down one row and left one column
                y = (y + 1) % n;
                x = (x + n - 1) % n; // Move left
                continue;
            }
            magic[y][x] = number; // Place the number
            y = (y + 1) % n; // Move down
            x = (x + 1) % n; // Move right
            number += 1;
        }

        for i in 0..n {
            for j in 0..n {
                print!("{:4}", magic[i][j]); // Right-align within width of 4
            }
            println!(); // Newline after each row
        }
    }
}
