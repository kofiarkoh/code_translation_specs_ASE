// translated from the provided C code according to the specification.
// Safe, idiomatic Rust; preserves semantics including coordinate ordering:
// first integer read is column (1-based), second is row (1-based).
use std::io::{self, Read};

fn bomb(field: &mut [[u8; 8]; 8], x: usize, y: usize) {
    // x = row index, y = column index (both 0-based)
    field[x][y] = 0;
    let xi = x as i32;
    let yi = y as i32;

    // Up (decreasing row)
    for d in 1..=3 {
        let nx = xi - d;
        if nx < 0 {
            break;
        }
        let ux = nx as usize;
        if field[ux][y] == 1 {
            field[ux][y] = 0;
            bomb(field, ux, y);
        }
    }

    // Down (increasing row)
    for d in 1..=3 {
        let nx = xi + d;
        if nx >= 8 {
            break;
        }
        let ux = nx as usize;
        if field[ux][y] == 1 {
            field[ux][y] = 0;
            bomb(field, ux, y);
        }
    }

    // Left (decreasing column)
    for d in 1..=3 {
        let ny = yi - d;
        if ny < 0 {
            break;
        }
        let uy = ny as usize;
        if field[x][uy] == 1 {
            field[x][uy] = 0;
            bomb(field, x, uy);
        }
    }

    // Right (increasing column)
    for d in 1..=3 {
        let ny = yi + d;
        if ny >= 8 {
            break;
        }
        let uy = ny as usize;
        if field[x][uy] == 1 {
            field[x][uy] = 0;
            bomb(field, x, uy);
        }
    }
}

fn main() {
    // Read entire stdin and split by whitespace like scanf("%s"/"%d") in C.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut it = input.split_whitespace();

    // Helper to get next token or panic (spec assumes valid input).
    let mut next_token = || -> &str {
        it.next().expect("unexpected end of input")
    };

    let n: usize = next_token().parse().expect("failed to parse n");

    for case in 1..=n {
        let mut field: [[u8; 8]; 8] = [[0; 8]; 8];

        for row in 0..8 {
            let line = next_token();
            let bytes = line.as_bytes();
            assert!(bytes.len() == 8, "grid line must be 8 chars");
            for col in 0..8 {
                // Expect '0' or '1'
                let b = bytes[col];
                // Convert ascii digit to numeric 0/1
                field[row][col] = (b - b'0') as u8;
            }
        }

        // Read column (x) then row (y), both 1-based in input.
        let input_x: i32 = next_token().parse().expect("failed to parse x");
        let input_y: i32 = next_token().parse().expect("failed to parse y");

        // Convert to 0-based and note ordering: first token is column.
        let col = (input_x - 1) as usize;
        let row = (input_y - 1) as usize;

        // Call bomb with (row, column) as in the spec / C code.
        bomb(&mut field, row, col);

        // Print output exactly as specified.
        println!("Data {}:", case);
        for r in 0..8 {
            for c in 0..8 {
                print!("{}", field[r][c]);
            }
            println!();
        }
    }
}
