use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input until a zero is encountered
    while io::stdin().read_line(&mut input).is_ok() {
        let n: usize = match input.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(0) => break, // Stop reading on zero
            _ => continue, // Ignore invalid inputs
        };

        // Generate and print the magic square for the valid size n
        if n <= 16 {
            let magic_square = generate_magic_square(n);
            print_magic_square(&magic_square);
        }

        input.clear(); // Clear input for the next read
    }
}

// Function to generate a magic square of size n x n
fn generate_magic_square(n: usize) -> Vec<Vec<usize>> {
    let mut square = vec![vec![0; n]; n];
    let mut num = 1;
    let mut i = 0;
    let mut j = n / 2; // Start from the middle of the first row

    while num <= n * n {
        square[i][j] = num;
        num += 1;

        // Move to the next position
        let new_i = (i as isize - 1).rem_euclid(n as isize) as usize; // Up
        let new_j = (j as isize + 1).rem_euclid(n as isize) as usize; // Right

        if square[new_i][new_j] != 0 { // If the position is already filled
            i = (i + 1) % n; // Move down instead
        } else {
            i = new_i;
            j = new_j;
        }
    }

    square
}

// Function to print the magic square
fn print_magic_square(square: &[Vec<usize>]) {
    for row in square {
        for &num in row {
            // Print each number right-aligned within a width of 4 characters
            print!("{:>4}", num);
        }
        println!(); // New line after each row
    }
}
