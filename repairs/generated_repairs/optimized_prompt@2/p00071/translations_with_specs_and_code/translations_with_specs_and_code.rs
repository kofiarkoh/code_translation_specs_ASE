use std::io::{self, BufRead};

const DIR: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    // Mark the current cell as exploded
    field[py][px] = '0';
    
    // Propagate the explosion in all four directions
    for (dx, dy) in DIR.iter() {
        for j in 1..=3 {
            let x = (px as isize + j * dx) as usize;
            let y = (py as isize + j * dy) as usize;

            // Check bounds
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break;
            }
            if field[y][x] == '1' {
                explode(field, x, y);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let first_line = lines.next().unwrap().unwrap();
    let n_cases: usize = first_line.trim().parse().unwrap();

    for n in 1..=n_cases {
        let mut field = [['0'; 10]; 10]; // Initialize the field with '0'

        // Read the grid, skipping the first and last row
        for i in 1..=8 {
            let line = lines.next().unwrap().unwrap();
            field[i][1..9].copy_from_slice(&line.chars().collect::<Vec<_>>()[..8]);
        }

        // Read starting coordinates
        let sx: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let sy: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Check bounds for starting coordinates
        if sx >= 1 && sx <= 8 && sy >= 1 && sy <= 8 {
            explode(&mut field, sx, sy);
        }

        // Output the modified grid
        println!("Data {}:", n);
        for i in 1..=8 {
            println!("{}", field[i][1..9].iter().collect::<String>());
        }
    }
}
