use std::io::{self, BufRead};

const H: usize = 101;
const W: usize = 101;

fn mag_tile(y: usize, x: usize, data: &mut [[i32; W]; H], tiles: &[[char; W]; H]) {
    if data[y][x] == 1 {
        println!("LOOP");
        return;
    }
    if tiles[y][x] == '.' {
        println!("{} {}", x, y);
        return;
    }
    
    let (new_y, new_x) = match tiles[y][x] {
        '>' => (y, x + 1),
        '<' => (y, x.wrapping_sub(1)), // wrapping_sub to emulate C's behavior
        '^' => (y.wrapping_sub(1), x), // wrapping_sub to prevent underflow
        'v' => (y + 1, x),
        _ => return, // Handle unexpected characters gracefully
    };

    data[y][x] += 1; // Increment the data at the current position
    mag_tile(new_y, new_x, data, tiles);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut data = [[0; W]; H];
    let mut tiles = [['.'; W]; H];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let h: i32 = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        for i in 0..h {
            let line = reader.lines().next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                tiles[i as usize][j] = c;
                data[i as usize][j] = 0;
            }
        }
        mag_tile(0, 0, &mut data, &tiles);
    }
}
