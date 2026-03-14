use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).unwrap() > 0 {
        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if n == 0 {
            break;
        }

        let mut magic = [[0; 16]; 16];
        let hn = n / 2;
        let mut x = hn;
        let mut y = hn + 1;
        let mut number = 1;

        while number <= n * n {
            if magic[y as usize][x as usize] != 0 {
                y = (y + 1 + n) % n;
                x = (x - 1 + n) % n;
                continue;
            }
            magic[y as usize][x as usize] = number;
            y = (y + 1) % n;
            x = (x + 1) % n;
            number += 1;
        }

        for i in 0..n {
            for j in 0..n {
                print!("{:4}", magic[i as usize][j as usize]);
            }
            println!();
        }

        input.clear(); // Clear input for the next iteration
    }
}
