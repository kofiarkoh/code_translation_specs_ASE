use std::io::{self, BufRead};

const N_MAX: usize = 4;
const SIZE: usize = N_MAX * 2 + 1;

static RIGHT: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // top
    [0, 0, 3, 5, 2, 4, 0], // 1
    [0, 4, 0, 1, 6, 0, 3], // 2
    [0, 2, 6, 0, 0, 1, 5], // 3
    [0, 5, 1, 0, 0, 6, 2], // 4
    [0, 3, 0, 6, 1, 0, 4], // 5
    [0, 0, 4, 2, 5, 3, 0], // 6
];

fn right(top: u8, front: u8) -> u8 {
    RIGHT[top as usize][front as usize]
}

fn bottom(top: u8) -> u8 {
    7 - top
}

fn back(front: u8) -> u8 {
    7 - front
}

fn left(top: u8, front: u8) -> u8 {
    7 - right(top, front)
}

fn init(height_map: &mut [[u8; SIZE]; SIZE], top_map: &mut [[u8; SIZE + 1]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            height_map[y][x] = 0;
            top_map[y][x] = 0;
        }
    }
}

fn put(height_map: &mut [[u8; SIZE]; SIZE], top_map: &mut [[u8; SIZE + 1]; SIZE], mut top: u8, mut front: u8) {
    let mut x = N_MAX;
    let mut y = N_MAX;

    loop {
        let mut max = 0;
        let (mut newfront, mut newtop, mut newx, mut newy) = (0, 0, x, y);

        // Check front
        if height_map[y - 1][x] < height_map[y][x] && front > 3 && front > max {
            newfront = top;
            newtop = back(front);
            newx = x;
            newy = y - 1;
            max = front;
        }

        // Check back
        if height_map[y + 1][x] < height_map[y][x] && back(top) > 3 && back(top) > max {
            newfront = bottom(top);
            newtop = front;
            newx = x;
            newy = y + 1;
            max = back(top);
        }

        // Check left
        if height_map[y][x - 1] < height_map[y][x] && left(top, front) > 3 && left(top, front) > max {
            newfront = front;
            newtop = right(top, front);
            newx = x - 1;
            newy = y;
            max = left(top, front);
        }

        // Check right
        if height_map[y][x + 1] < height_map[y][x] && right(top, front) > 3 && right(top, front) > max {
            newfront = front;
            newtop = left(top, front);
            newx = x + 1;
            newy = y;
            max = right(top, front);
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

fn count_and_print(height_map: &[[u8; SIZE]; SIZE], top_map: &[[u8; SIZE + 1]; SIZE]) {
    let mut count = [0; 7];

    for y in 0..SIZE {
        for x in 0..SIZE {
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
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut height_map = [[0; SIZE]; SIZE];
        let mut top_map = [[0; SIZE + 1]; SIZE];

        init(&mut height_map, &mut top_map);

        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let parts: Vec<u8> = input.trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            let (t, f) = (parts[0], parts[1]);
            put(&mut height_map, &mut top_map, t, f);
        }

        count_and_print(&height_map, &top_map);
    }
}
