use std::io::{self, Write};

const N: usize = 5;

fn main() {
    let mut f = [[0u32; N]; N];
    let mut input = String::new();

    for i in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s: Vec<char> = input.trim().chars().collect();

        if i % 2 == 1 {
            for j in 0..5 {
                let a = s[j].to_digit(10).unwrap();
                f[i / 2][j] |= a << 2;
                f[i / 2 + 1][j] |= a;
            }
        } else {
            for j in 0..4 {
                let a = s[j].to_digit(10).unwrap();
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
        while !((f[y][x] >> (p = (p + 1) % 4)) & 1) != 0) {}

        print!("{}", "URDL".chars().nth(p).unwrap());
        x += if p != 0 { 2 - p } else { 0 };
        y += if p == 3 { 0 } else { p - 1 };

        if y == 0 && x == 0 && f[0][0] == 6 && p == 3 {
            break;
        }
    }
    println!();
}
