use std::io::{self, BufRead};

const H: usize = 101;
const W: usize = 101;

fn mag_tile(y: usize, x: usize, data: &mut [[usize; W]; H], tiles: &[[char; W]; H]) -> bool {
    if data[y][x] == 1 {
        println!("LOOP");
        return true;
    }
    if tiles[y][x] == '.' {
        println!("{} {}", x, y);
        return false;
    }

    // Mark the current tile as visited
    data[y][x] += 1;

    // Navigate based on the tile value
    match tiles[y][x] {
        '>' => {
            if x + 1 < W {
                return mag_tile(y, x + 1, data, tiles);
            }
        }
        '<' => {
            if x > 0 {
                return mag_tile(y, x - 1, data, tiles);
            }
        }
        '^' => {
            if y > 0 {
                return mag_tile(y - 1, x, data, tiles);
            }
        }
        'v' => {
            if y + 1 < H {
                return mag_tile(y + 1, x, data, tiles);
            }
        }
        _ => {}
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace();
        let h: usize = dimensions.next().unwrap().parse().unwrap();
        let w: usize = dimensions.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        let mut data = [[0; W]; H];
        let mut tiles = [[' '; W]; H];

        for i in 0..h {
            let line = reader.lines().next().unwrap().unwrap();
            for (j, c) in line.chars().enumerate() {
                tiles[i][j] = c;
                data[i][j] = 0;
            }
        }

        let _ = mag_tile(0, 0, &mut data, &tiles);
    }
}
