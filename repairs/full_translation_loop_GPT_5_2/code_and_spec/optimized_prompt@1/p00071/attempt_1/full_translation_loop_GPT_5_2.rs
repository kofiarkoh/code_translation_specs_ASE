use std::io::{self, Read};

const DIR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[u8; 10]; 10], px: i32, py: i32) {
    // Spec: if starting coordinates are out of bounds, the explosion will not occur.
    // The C code assumes valid (and effectively uses 1..=8). We enforce bounds here.
    if px < 1 || px > 8 || py < 1 || py > 8 {
        return;
    }

    field[py as usize][px as usize] = b'0';

    for (dx, dy) in DIR {
        for j in 1..=3 {
            let x = px + j * dx;
            let y = py + j * dy;
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
    // Read entire stdin; the original C uses gets/sscanf line-by-line, but the effective
    // parsing is whitespace-separated integers/strings. We preserve output formatting.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_cases: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for case_idx in 1..=n_cases {
        let mut field = [[b'\0'; 10]; 10];

        // C reads 9 lines into field[i][1] for i=0..=8, then prints i=1..=8.
        // Each line is expected to contain 8 characters (columns 1..=8).
        for i in 0..=8 {
            let line = it.next().unwrap_or("");
            let bytes = line.as_bytes();
            for col in 0..8 {
                field[i][1 + col] = *bytes.get(col).unwrap_or(&b'\0');
            }
        }

        let sx: i32 = it.next().unwrap().parse().unwrap();
        let sy: i32 = it.next().unwrap().parse().unwrap();

        explode(&mut field, sx, sy);

        out.push_str(&format!("Data {}:\n", case_idx));
        for i in 1..=8 {
            // Print columns 1..=8 as a line.
            let line = String::from_utf8_lossy(&field[i][1..=8]);
            out.push_str(&line);
            out.push('\n');
        }
    }

    print!("{out}");
}
