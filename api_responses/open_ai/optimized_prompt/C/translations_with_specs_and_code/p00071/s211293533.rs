use std::io::{self, BufRead};

const DIR: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    field[py][px] = '0'; // Mark the current cell as exploded
    for (dx, dy) in DIR.iter() {
        for j in 1..=3 {
            let x = (px as isize + j * dx) as usize;
            let y = (py as isize + j * dy) as usize;
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break; // Out of bounds
            }
            if field[y][x] == '1' {
                explode(field, x, y); // Recursive explosion
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let n_cases: usize = first_line.trim().parse().unwrap();

    for n in 1..=n_cases {
        let mut field = [[' '; 10]; 10];

        for i in 1..=8 {
            let line = lines.next().unwrap().unwrap();
            field[i][1..=8].copy_from_slice(&line.chars().collect::<Vec<_>>()[..8]);
        }

        let sx: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let sy: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Check bounds before exploding
        if sx >= 1 && sx <= 8 && sy >= 1 && sy <= 8 {
            explode(&mut field, sx, sy);
        }

        println!("Data {}:", n);
        for i in 1..=8 {
            println!("{}", field[i][1..=8].iter().collect::<String>());
        }
    }
}
