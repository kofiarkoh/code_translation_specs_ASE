use std::io::{self, BufRead};

const DIR: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    field[py][px] = '0'; // Mark the current cell as exploded
    for (dx, dy) in DIR.iter() {
        for j in 1..=3 {
            let x = (px as isize + j * dx) as usize;
            let y = (py as isize + j * dy) as usize;
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break; // Out of bounds check
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
    let n: usize = first_line.trim().parse().unwrap();

    for data_index in 1..=n {
        let mut field = [['0'; 10]; 10]; // Initialize the field with '0'
        
        for i in 1..=8 {
            let line = lines.next().unwrap().unwrap();
            field[i][1..9].copy_from_slice(&line.chars().collect::<Vec<_>>()[0..8]);
        }

        let sx: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let sy: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        explode(&mut field, sx, sy);

        println!("Data {}:", data_index);
        for i in 1..=8 {
            // Collect the entire row as a string, including the first and last columns
            let output_row: String = field[i].iter().collect();
            println!("{}", output_row);
        }
    }
}
``` 

### Changes Made:
1. **Output Row Construction**: The output row is now constructed by collecting the entire row, ensuring that the first and last columns are included in the output. This addresses the issue where the output was missing the first and last columns of the grid.
2. **Indexing**: The indexing remains consistent with the original logic, ensuring that the explosion logic and output format adhere to the specifications.
