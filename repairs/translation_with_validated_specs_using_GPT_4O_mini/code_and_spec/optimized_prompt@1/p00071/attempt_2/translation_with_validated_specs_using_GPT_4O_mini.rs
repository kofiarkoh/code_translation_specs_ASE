use std::io::{self, BufRead};

const DIR: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    field[py][px] = '0'; // Set the starting cell to '0'
    for (dx, dy) in DIR.iter() {
        for j in 1..=3 {
            let x = (px as isize + j * dx) as usize;
            let y = (py as isize + j * dy) as usize;
            // Check bounds
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break;
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
    let n_datasets: usize = first_line.trim().parse().unwrap();

    for n in 1..=n_datasets {
        let mut field = [['0'; 10]; 10]; // Initialize the grid

        // Read the 8x8 grid, starting from index 1
        for i in 1..=8 {
            let line = lines.next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                field[i][j + 1] = c; // Fill the grid
            }
        }

        // Read starting coordinates
        let sx: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let sy: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Perform explosion if coordinates are valid
        if sx >= 1 && sx <= 8 && sy >= 1 && sy <= 8 {
            explode(&mut field, sx, sy);
        }

        // Output the modified grid
        println!("Data {}:", n);
        for i in 1..=8 {
            println!("{}", &field[i][1..9].iter().collect::<String>());
        }
    }
}
