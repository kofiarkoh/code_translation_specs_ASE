use std::io::{self, Read};

const DIRS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[u8; 9]; 9], px: usize, py: usize) {
    // Coordinates are 1..=8 (inclusive) per spec/C code.
    field[py][px] = b'0';

    for (dx, dy) in DIRS {
        for step in 1..=3 {
            let x = px as i32 + step * dx;
            let y = py as i32 + step * dy;

            if x < 1 || x > 8 || y < 1 || y > 8 {
                break; // stop extending in this direction once out of bounds
            }

            let xu = x as usize;
            let yu = y as usize;
            if field[yu][xu] == b'1' {
                explode(field, xu, yu);
            }
        }
    }
}

fn main() {
    // Read entire stdin; no explicit error handling (matches spec: undefined on malformed input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    // First line: dataset count
    let n_line = it.next().unwrap_or("");
    let n_datasets: usize = n_line.trim().parse().unwrap();

    for dataset_idx in 1..=n_datasets {
        // field[y][x], with indices 1..=8 used; 0 row/col unused.
        let mut field = [[b'\0'; 9]; 9];

        // C reads 9 lines (i = 0..=8) into field[i][1], but only rows 1..=8 are later printed/used.
        // We preserve that input consumption behavior.
        for row in 0..=8 {
            let line = it.next().unwrap_or("");
            let bytes = line.as_bytes();
            for col in 1..=8 {
                field[row][col] = *bytes.get(col - 1).unwrap_or(&b'\0');
            }
        }

        // Starting coordinates: each read from its own line
        let sx_line = it.next().unwrap_or("");
        let sx: usize = sx_line.trim().parse().unwrap();
        let sy_line = it.next().unwrap_or("");
        let sy: usize = sy_line.trim().parse().unwrap();

        explode(&mut field, sx, sy);

        print!("Data {}:\n", dataset_idx);
        for row in 1..=8 {
            // Print exactly 8 characters per line (like puts(&(field[i][1])) in C).
            let line = String::from_utf8_lossy(&field[row][1..=8]);
            println!("{}", line);
        }
    }
}
