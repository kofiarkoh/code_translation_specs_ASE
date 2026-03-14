use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut f = [[0; 5]; 5];

    // Read exactly 9 input strings
    for i in 0..9 {
        let mut s = String::new();
        stdin.lock().read_line(&mut s).expect("Failed to read line");
        let s = s.trim(); // Remove any trailing newline or spaces

        // Ensure the input is valid
        assert!(s.len() <= 5 && s.chars().all(|c| c == '0' || c == '1'), "Invalid input");

        if i % 2 == 1 {
            for j in 0..5 {
                let a = s.chars().nth(j).unwrap() as u8 - b'0';
                f[i / 2][j] |= a << 2;
                f[i / 2 + 1][j] |= a;
            }
        } else {
            for j in 0..4 {
                let a = s.chars().nth(j).unwrap() as u8 - b'0';
                f[i / 2][j] |= a << 1;
                f[i / 2][j + 1] |= a << 3;
            }
        }
    }

    print!("R");
    let mut y = 0;
    let mut x = 1;
    let mut p = 1;

    loop {
        p = (p + 2) % 4;
        while !((f[y][x] >> (p = (p + 1) % 4)) & 1 != 0) {}

        print!("{}", "URDL".chars().nth(p).unwrap());
        x += if p != 0 { 2 - p } else { 0 };
        y += if p == 3 { 0 } else { p - 1 };

        if y == 0 && x == 0 && f[0][0] == 6 && p == 3 {
            break;
        }
    }
    println!(); // Ensure the output ends with a newline
}
