use std::io::{self, Write};

const N_MAX: usize = 4;

static RIGHT: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // top
    [0, 0, 3, 5, 2, 4, 0], // 1
    [0, 4, 0, 1, 6, 0, 3], // 2
    [0, 2, 6, 0, 0, 1, 5], // 3
    [0, 5, 1, 0, 0, 6, 2], // 4
    [0, 3, 0, 6, 1, 0, 4], // 5
    [0, 0, 4, 2, 5, 3, 0], // 6
];

fn right(top: usize, front: usize) -> u8 {
    RIGHT[top][front]
}

fn bottom(top: usize, front: usize) -> usize {
    7 - top
}

fn back(top: usize, front: usize) -> usize {
    7 - front
}

fn left(top: usize, front: usize) -> usize {
    7 - right(top, front) as usize
}

fn init(height_map: &mut [[u8; N_MAX * 2 + 1]; N_MAX * 2 + 1], top_map: &mut [[u8; N_MAX * 2 + 2]; N_MAX * 2 + 1]) {
    for y in 0..N_MAX * 2 + 1 {
        for x in 0..N_MAX * 2 + 1 {
            height_map[y][x] = 0;
            top_map[y][x] = 0;
        }
    }
}

fn put(height_map: &mut [[u8; N_MAX * 2 + 1]; N_MAX * 2 + 1], top_map: &mut [[u8; N_MAX * 2 + 2]; N_MAX * 2 + 1], mut top: u8, mut front: u8) {
    let mut x = N_MAX;
    let mut y = N_MAX;

    loop {
        let mut max = 0;
        let (mut newfront, mut newtop, mut newx, mut newy) = (0, 0, x, y);

        // front
        if height_map[y - 1][x] < height_map[y][x] && front > 3 && front > max {
            newfront = top;
            newtop = back(top as usize, front as usize) as u8;
            newx = x;
            newy = y - 1;
            max = front;
        }

        // back
        if height_map[y + 1][x] < height_map[y][x] && back(top as usize, front as usize) > 3 && back(top as usize, front as usize) > max {
            newfront = bottom(top as usize, front as usize) as u8;
            newtop = front;
            newx = x;
            newy = y + 1;
            max = back(top as usize, front as usize);
        }

        // left
        if height_map[y][x - 1] < height_map[y][x] && left(top as usize, front as usize) > 3 && left(top as usize, front as usize) > max {
            newfront = front;
            newtop = right(top as usize, front as usize);
            newx = x - 1;
            newy = y;
            max = left(top as usize, front as usize);
        }

        // right
        if height_map[y][x + 1] < height_map[y][x] && right(top as usize, front as usize) > 3 && right(top as usize, front as usize) > max {
            newfront = front;
            newtop = left(top as usize, front as usize);
            newx = x + 1;
            newy = y;
            max = right(top as usize, front as usize);
        }

        if max == 0 {
            break;
        } else {
            front = newfront;
            top = newtop;
            x = newx;
            y = newy;
        }
    }

    height_map[y][x] += 1;
    top_map[y][x] = top;
}

fn count_and_print(height_map: &[[u8; N_MAX * 2 + 1]; N_MAX * 2 + 1], top_map: &[[u8; N_MAX * 2 + 2]; N_MAX * 2 + 1]) {
    let mut count = [0; 7];

    for y in 0..N_MAX * 2 + 1 {
        for x in 0..N_MAX * 2 + 1 {
            count[top_map[y][x] as usize] += 1;
        }
    }

    for x in 1..6 {
        print!("{} ", count[x]);
    }
    println!("{}", count[6]);
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut height_map = [[0; N_MAX * 2 + 1]; N_MAX * 2 + 1];
        let mut top_map = [[0; N_MAX * 2 + 2]; N_MAX * 2 + 1];

        init(&mut height_map, &mut top_map);

        for _ in 0..n {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let t: u8 = parts.next().unwrap().parse().unwrap();
            let f: u8 = parts.next().unwrap().parse().unwrap();
            put(&mut height_map, &mut top_map, t, f);
        }

        count_and_print(&height_map, &top_map);
    }
}
