use std::io::{self, Read};

fn explode(grid: &mut Vec<Vec<u8>>, px: i32, py: i32) {
    // Specification assumes px,py are within 1..=8; otherwise C would be UB.
    // We choose the narrowest reasonable interpretation: only operate when in-range.
    if !(1..=8).contains(&px) || !(1..=8).contains(&py) {
        return;
    }

    let (x, y) = (px as usize, py as usize);
    if y >= grid.len() || x >= grid[y].len() {
        // Input line length is not validated in the spec; avoid panics on short lines.
        return;
    }

    // Exploding a cell immediately sets it to '0' even if it was not '1'.
    grid[y][x] = b'0';

    // Directions: left, up, right, down.
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for (dx, dy) in DIRS {
        for dist in 1..=3 {
            let nx = px + dx * dist;
            let ny = py + dy * dist;

            // Stop early in this direction if leaving interior.
            if !(1..=8).contains(&nx) || !(1..=8).contains(&ny) {
                break;
            }

            let (ux, uy) = (nx as usize, ny as usize);
            if uy >= grid.len() || ux >= grid[uy].len() {
                // If the stored row is too short, treat as non-'1' and continue.
                continue;
            }

            if grid[uy][ux] == b'1' {
                explode(grid, nx, ny);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = match lines.next() {
        Some(s) => s.trim().parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for dataset_idx in 1..=n {
        // Store 9 lines; each line is stored with its first character at column index 1.
        // We emulate this by prefixing a dummy byte at index 0.
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(9);
        for _ in 0..9 {
            let line = lines.next().unwrap_or("");
            let mut row = Vec::with_capacity(line.len() + 1);
            row.push(0); // dummy column 0
            row.extend_from_slice(line.as_bytes());
            grid.push(row);
        }

        let sx: i32 = lines.next().unwrap_or("0").trim().parse().unwrap();
        let sy: i32 = lines.next().unwrap_or("0").trim().parse().unwrap();

        // Rows are conceptually 1..=8; we stored 9 rows (indices 0..8).
        // Explosion operates on interior coordinates (x,y) with 1..=8.
        explode(&mut grid, sx, sy);

        out.push_str(&format!("Data {}:\n", dataset_idx));

        // Print stored grid rows 1 through 8, starting at column index 1.
        for y in 1..=8 {
            if y >= grid.len() {
                out.push('\n');
                continue;
            }
            let row = &grid[y];
            if row.len() <= 1 {
                out.push('\n');
                continue;
            }
            out.push_str(&String::from_utf8_lossy(&row[1..]));
            out.push('\n');
        }
    }

    print!("{out}");
}
