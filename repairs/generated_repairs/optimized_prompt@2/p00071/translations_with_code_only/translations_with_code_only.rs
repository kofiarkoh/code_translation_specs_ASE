use std::io::{self, BufRead};

const DIR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn explode(field: &mut [[char; 10]; 10], px: usize, py: usize) {
    field[py][px] = '0';
    for i in 0..4 {
        for j in 1..=3 {
            let x = px as i32 + j * DIR[i].0;
            let y = py as i32 + j * DIR[i].1;
            if x < 1 || x > 8 || y < 1 || y > 8 {
                break;
            }
            if field[y as usize][x as usize] == '1' {
                explode(field, x as usize, y as usize);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut field: [[char; 10]; 10] = [['0'; 10]; 10]; // Initialize with '0'
    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.trim().parse().unwrap();

    for n in 1..=n {
        for i in 1..=8 { // Change to start from 1 to 8
            let line = lines.next().unwrap().unwrap();
            field[i][1..9].copy_from_slice(&line.chars().collect::<Vec<_>>()[..8]);
        }
        let sx: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let sy: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        explode(&mut field, sx, sy);

        println!("Data {}:", n);
        for i in 1..=8 {
            println!("{}", field[i][1..9].iter().collect::<String>());
        }
        // Reset the field for the next test case
        field = [['0'; 10]; 10];
    }
}
