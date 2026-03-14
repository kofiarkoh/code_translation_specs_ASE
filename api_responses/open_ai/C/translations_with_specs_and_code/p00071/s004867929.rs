use std::io::{self, BufRead};

fn bomb(field: &mut [[u8; 8]; 8], x: usize, y: usize) {
    // Set the current position to '0'
    field[x][y] = 0;

    // Check and recursively bomb in all four directions
    // Up
    if x > 0 && field[x - 1][y] == 1 {
        bomb(field, x - 1, y);
    }
    // Down
    if x < 7 && field[x + 1][y] == 1 {
        bomb(field, x + 1, y);
    }
    // Left
    if y > 0 && field[x][y - 1] == 1 {
        bomb(field, x, y - 1);
    }
    // Right
    if y < 7 && field[x][y + 1] == 1 {
        bomb(field, x, y + 1);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 0..n {
        let mut field = [[0; 8]; 8];

        // Read the 8x8 grid
        for j in 0..8 {
            let line = lines.next().unwrap().unwrap();
            for (k, c) in line.chars().enumerate() {
                field[j][k] = c.to_digit(10).unwrap() as u8;
            }
        }

        // Read the coordinates and adjust for 0-based indexing
        let (mut x, mut y): (usize, usize) = {
            let coords: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            (coords[0] - 1, coords[1] - 1)
        };

        // Perform the explosion
        bomb(&mut field, x, y);

        // Print the result
        println!("Data {}:", i + 1);
        for j in 0..8 {
            for k in 0..8 {
                print!("{}", field[j][k]);
            }
            println!();
        }
    }
}
