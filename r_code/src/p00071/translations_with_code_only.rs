use std::io::{self, Read};

fn bomb(field: &mut [[i32; 8]; 8], x: i32, y: i32) {
    // Mimic the C function semantics: x and y are signed ints.
    // The caller ensures (by the same checks as in C) that indexing is safe.
    field[x as usize][y as usize] = 0;
    // Upwards: i = 1..=3 while x-i >= 0
    for i in 1..=3 {
        if x - i < 0 {
            break;
        }
        let xi = (x - i) as usize;
        let yi = y as usize;
        if field[xi][yi] == 1 {
            field[xi][yi] = 0;
            bomb(field, x - i, y);
        }
    }
    // Downwards: i = 1..=3 while x+i < 8
    for i in 1..=3 {
        if x + i >= 8 {
            break;
        }
        let xi = (x + i) as usize;
        let yi = y as usize;
        if field[xi][yi] == 1 {
            field[xi][yi] = 0;
            bomb(field, x + i, y);
        }
    }
    // Left: i = 1..=3 while y-i >= 0
    for i in 1..=3 {
        if y - i < 0 {
            break;
        }
        let xi = x as usize;
        let yi = (y - i) as usize;
        if field[xi][yi] == 1 {
            field[xi][yi] = 0;
            bomb(field, x, y - i);
        }
    }
    // Right: i = 1..=3 while y+i < 8
    for i in 1..=3 {
        if y + i >= 8 {
            break;
        }
        let xi = x as usize;
        let yi = (y + i) as usize;
        if field[xi][yi] == 1 {
            field[xi][yi] = 0;
            bomb(field, x, y + i);
        }
    }
}

fn main() {
    // Read whole stdin and split by whitespace, similar to repeated scanf calls.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    // Parse number of datasets
    let n: i32 = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => return,
    };

    for case in 1..=n {
        // field[row][col], matching C's field[j][k] with j=0..7 rows
        let mut field: [[i32; 8]; 8] = [[0; 8]; 8];

        // Read 8 lines, each as a token (scanf("%s",line))
        for row in 0..8 {
            let line = it.next().unwrap();
            // The C code uses line[k] - '0' for each character.
            // We replicate by converting each char to its digit value.
            // Assumes input contains at least 8 characters per line as in the C code.
            let mut chars = line.chars();
            for col in 0..8 {
                let c = chars.next().unwrap();
                // Convert ASCII digit to integer (like c - '0').
                // Using to_digit ensures correctness for digit chars.
                field[row][col] = c.to_digit(10).unwrap() as i32;
            }
        }

        // Read x and y, then decrement to convert from 1-based to 0-based like C's x--; y--;
        let x_in: i32 = it.next().unwrap().parse().unwrap();
        let y_in: i32 = it.next().unwrap().parse().unwrap();
        let mut x = x_in - 1;
        let mut y = y_in - 1;

        // In the C code the call is bomb(field, y, x) (note the swap).
        // Preserve that exact behavior.
        bomb(&mut field, y, x);

        // Print output exactly as the C program does.
        println!("Data {}:", case);
        for row in 0..8 {
            for col in 0..8 {
                print!("{}", field[row][col]);
            }
            println!();
        }
    }
}
