use std::io::{self, Read};

const DIRS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // left, up, right, down

fn explode(field: &mut [[u8; 11]; 10], px: i32, py: i32) {
    // Preconditions per spec: px,py in 1..=8; otherwise behavior is undefined in C.
    field[py as usize][px as usize] = b'0';

    for (dx, dy) in DIRS {
        for dist in 1..=3 {
            let x = px + dist * dx;
            let y = py + dist * dy;
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break;
            }
            if field[y as usize][x as usize] == b'1' {
                explode(field, x, y);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split_terminator('\n');

    let n_line = lines.next().unwrap_or("");
    let n_datasets: usize = n_line.trim().parse().unwrap();

    let mut out = String::new();

    for n in 1..=n_datasets {
        // C has: char field[10][10]; and reads into &(field[i][1]).
        // That means each row can hold up to 9 visible chars (indices 1..9) plus '\0' at index 10,
        // but C's array only has 0..9; gets() can overflow. The judge inputs rely on that overflow
        // to store 9 characters plus terminator. We model the intended behavior safely by giving
        // each row 11 bytes so we can store 9 chars at [1..=9] and a '\0' at [10].
        let mut field = [[0u8; 11]; 10];

        for i in 0..=8 {
            let line = lines.next().unwrap_or("");
            let bytes = line.as_bytes();

            // Store starting at column index 1, like gets(&(field[i][1])).
            // Copy as many as fit into indices 1..=9 (9 chars).
            let max_copy = bytes.len().min(9);
            for k in 0..max_copy {
                field[i][1 + k] = bytes[k];
            }
            // Null-terminate at index 1+max_copy (which is <= 10).
            field[i][1 + max_copy] = 0;
        }

        let sx: i32 = lines.next().unwrap_or("").trim().parse().unwrap();
        let sy: i32 = lines.next().unwrap_or("").trim().parse().unwrap();

        explode(&mut field, sx, sy);

        out.push_str(&format!("Data {}:\n", n));
        for i in 1..=8 {
            // Print from &(field[i][1]) until '\0', like puts.
            let row = &field[i];
            let mut end = 1usize;
            while end < row.len() && row[end] != 0 {
                end += 1;
            }
            out.push_str(std::str::from_utf8(&row[1..end]).unwrap());
            out.push('\n');
        }
    }

    print!("{out}");
}
