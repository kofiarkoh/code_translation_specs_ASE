use std::io::{self, BufRead};

const GRID_SIZE: usize = 10;

fn main() {
    let mut map = [[0; GRID_SIZE]; GRID_SIZE];
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(input) = line {
            let parts: Vec<&str> = input.split(',').collect();
            if parts.len() == 3 {
                if let (Ok(x), Ok(y), Ok(z)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                    if x < GRID_SIZE && y < GRID_SIZE && z >= 1 && z <= 3 {
                        drop_ink(&mut map, x, y, z);
                    }
                }
            }
        }
    }

    println!("{}\n{}", untaint(&map), most_taint(&map));
}

fn drop_ink(map: &mut [[usize; GRID_SIZE]; GRID_SIZE], x: usize, y: usize, z: usize) {
    match z {
        3 => taint_3(map, x, y),
        2 => {
            taint_2(map, x, y);
            taint_3(map, x, y);
        }
        1 => {
            taint_1(map, x, y);
            taint_2(map, x, y);
            taint_3(map, x, y);
        }
        _ => {}
    }
}

fn taint_1(map: &mut [[usize; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    if x > 0 {
        map[x - 1][y] += 1;
    }
    if x + 1 < GRID_SIZE {
        map[x + 1][y] += 1;
    }
    if y > 0 {
        map[x][y - 1] += 1;
    }
    if y + 1 < GRID_SIZE {
        map[x][y + 1] += 1;
    }
}

fn taint_2(map: &mut [[usize; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    if x > 0 {
        if y > 0 {
            map[x - 1][y - 1] += 1;
        }
        if y + 1 < GRID_SIZE {
            map[x - 1][y + 1] += 1;
        }
    }
    if x + 1 < GRID_SIZE {
        if y > 0 {
            map[x + 1][y - 1] += 1;
        }
        if y + 1 < GRID_SIZE {
            map[x + 1][y + 1] += 1;
        }
    }
}

fn taint_3(map: &mut [[usize; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    if x >= 2 {
        map[x - 2][y] += 1;
    }
    if x + 2 < GRID_SIZE {
        map[x + 2][y] += 1;
    }
    if y >= 2 {
        map[x][y - 2] += 1;
    }
    if y + 2 < GRID_SIZE {
        map[x][y + 2] += 1;
    }
}

fn untaint(map: &[[usize; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut count = 0;
    for row in map.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                count += 1;
            }
        }
    }
    count
}

fn most_taint(map: &[[usize; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut max_taint = 0;
    for row in map.iter() {
        for &cell in row.iter() {
            if cell > max_taint {
                max_taint = cell;
            }
        }
    }
    max_taint
}
