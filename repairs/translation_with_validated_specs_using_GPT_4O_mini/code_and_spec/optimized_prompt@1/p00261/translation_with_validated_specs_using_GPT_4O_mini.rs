use std::io::{self, BufRead};

const A: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;
const W: usize = 4;
const B: usize = 5;
const NA: usize = 6;

const MAP: [[usize; 2]; 6] = [
    [X, Y], // From A
    [NA, Z], // From X
    [X, NA], // From Y
    [W, B], // From Z
    [B, Y], // From W
    [Y, X], // From B
];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let path = line.unwrap();
        if path.starts_with('#') {
            break;
        }

        let mut city = A;
        let mut valid = true;

        for ch in path.chars() {
            if let Some(index) = ch.to_digit(10) {
                let idx = index as usize;
                if idx < 2 && MAP[city][idx] == NA {
                    valid = false;
                    break;
                }
                city = MAP[city][idx];
            } else {
                valid = false;
                break;
            }
        }

        if valid && city == B {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
