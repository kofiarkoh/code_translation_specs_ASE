use std::io::{self, BufRead};

const WIDTH: usize = 6;
const HEIGHT: usize = 8;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut abacus = [['*'; WIDTH]; HEIGHT];
    let mut first_output = true;

    for line in reader.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        // Initialize the abacus grid
        for i in 0..HEIGHT {
            for j in 0..WIDTH - 1 { // Leave the last column for null terminator
                abacus[i][j] = '*';
            }
        }

        // Set the middle row to '='
        for j in 0..WIDTH - 1 {
            abacus[2][j] = '=';
        }

        // Process each digit of the number
        for i in 0..5 {
            let a = n % 10;
            n /= 10;

            if a < 5 {
                abacus[1][4 - i] = ' ';
            } else {
                abacus[0][4 - i] = ' ';
                a -= 5;
            }

            abacus[3 + a as usize][4 - i] = ' ';
        }

        // Print a newline before the output if it's not the first output
        if !first_output {
            println!();
        }
        first_output = false;

        // Output the abacus representation
        for i in 0..HEIGHT {
            println!("{}", abacus[i].iter().collect::<String>());
        }
    }
}
