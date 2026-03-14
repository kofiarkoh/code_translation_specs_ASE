use std::io::{self, Write};

const WIDTH: usize = 6;
const HEIGHT: usize = 8;

fn main() {
    let mut abacus = [[' '; WIDTH]; HEIGHT];
    let mut f = false;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if n == 0 {
            break;
        }

        // Initialize abacus with '*'
        for i in 0..HEIGHT {
            for j in 0..WIDTH - 1 { // Only fill first 5 columns
                abacus[i][j] = '*';
            }
        }

        // Set the second row to '='
        for i in 0..WIDTH - 1 { // Only fill first 5 columns
            abacus[2][i] = '=';
        }

        let mut temp_n = n;
        for i in 0..5 {
            let a = temp_n % 10;
            temp_n /= 10;

            if a < 5 {
                abacus[1][4 - i] = ' ';
            } else {
                abacus[0][4 - i] = ' ';
                // Decrement a by 5
                let a = a - 5;
                abacus[3 + a][4 - i] = ' ';
            } else {
                abacus[3 + a][4 - i] = ' ';
            }
        }

        if f {
            println!(); // Print a newline if it's not the first output
        }

        for i in 0..HEIGHT {
            println!("{}", abacus[i].iter().collect::<String>());
        }

        f = true; // Set flag to true after first output
    }
}
