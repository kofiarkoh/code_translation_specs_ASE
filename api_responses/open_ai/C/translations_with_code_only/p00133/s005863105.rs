/*
  AOJ 0133
  Title: Rotation of a Pattern
  @kankichi573
*/

use std::io::{self, BufRead};

const SIZE: usize = 8;
const PATTERN_LENGTH: usize = 9;

fn rot(pat1: &[[u8; PATTERN_LENGTH]; SIZE], pat2: &mut [[u8; PATTERN_LENGTH]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            pat2[i][j] = pat1[SIZE - 1 - j][i];
        }
    }
}

fn display(pat2: &[[u8; PATTERN_LENGTH]; SIZE]) {
    for i in 0..SIZE {
        // Print the row as a string, converting from u8 to char
        println!("{}", std::str::from_utf8(&pat2[i][..PATTERN_LENGTH - 1]).unwrap());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut pat1: [[u8; PATTERN_LENGTH]; SIZE] = [[0; PATTERN_LENGTH]; SIZE];
    let mut pat2: [[u8; PATTERN_LENGTH]; SIZE] = [[0; PATTERN_LENGTH]; SIZE];

    // Read input patterns
    for i in 0..SIZE {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        // Copy input into pat1, ensuring null termination
        let bytes = input.as_bytes();
        pat1[i][..bytes.len()].copy_from_slice(bytes);
        pat1[i][bytes.len()] = 0; // Null-terminate the string
    }

    for i in 1..=3 {
        rot(&pat1, &mut pat2);
        println!("{}", i * 90);
        display(&pat2);
        // Copy pat2 back to pat1
        pat1.copy_from_slice(&pat2);
    }
}
