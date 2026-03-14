use std::io::{self, BufRead};

fn bomb(field: &mut [[i32; 8]; 8], x: usize, y: usize) {
    field[x][y] = 0;
    for i in 1..=3 {
        if x >= i && field[x - i][y] == 1 {
            field[x - i][y] = 0;
            bomb(field, x - i, y);
        }
        if x + i < 8 && field[x + i][y] == 1 {
            field[x + i][y] = 0;
            bomb(field, x + i, y);
        }
        if y >= i && field[x][y - i] == 1 {
            field[x][y - i] = 0;
            bomb(field, x, y - i);
        }
        if y + i < 8 && field[x][y + i] == 1 {
            field[x][y + i] = 0;
            bomb(field, x, y + i);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 1..=n {
        let mut field = [[0; 8]; 8];
        for j in 0..8 {
            let line = lines.next().unwrap().unwrap();
            for k in 0..8 {
                field[j][k] = line.chars().nth(k).unwrap().to_digit(10).unwrap() as i32;
            }
        }
        let x: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let y: usize = lines.next().unwrap().unwrap().parse().unwrap();
        
        // Adjusting for 0-based indexing
        let x = x - 1;
        let y = y - 1;

        bomb(&mut field, y, x);

        println!("Data {}:", i);
        for j in 0..8 {
            for k in 0..8 {
                print!("{}", field[j][k]);
            }
            println!();
        }
    }
}
