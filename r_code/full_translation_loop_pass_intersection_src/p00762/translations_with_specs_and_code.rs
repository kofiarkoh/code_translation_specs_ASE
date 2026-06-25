use std::io::{self, Read};

const N_MAX: usize = 4;
const SIZE: usize = N_MAX * 2 + 1; // 9

// RIGHT(top, front) lookup table from the C program.
// Indices 0..=6, with 0 row/col unused (kept as 0).
const RIGHT_TABLE: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // top = 0 (unused)
    [0, 0, 3, 5, 2, 4, 0], // top = 1
    [0, 4, 0, 1, 6, 0, 3], // top = 2
    [0, 2, 6, 0, 0, 1, 5], // top = 3
    [0, 5, 1, 0, 0, 6, 2], // top = 4
    [0, 3, 0, 6, 1, 0, 4], // top = 5
    [0, 0, 4, 2, 5, 3, 0], // top = 6
];

#[inline]
fn right(top: u8, front: u8) -> u8 {
    RIGHT_TABLE[top as usize][front as usize]
}
#[inline]
fn bottom(top: u8) -> u8 {
    7 - top
}
#[inline]
fn back(front: u8) -> u8 {
    7 - front
}
#[inline]
fn left(top: u8, front: u8) -> u8 {
    7 - right(top, front)
}

fn init_maps(height_map: &mut [[u8; SIZE]; SIZE], top_map: &mut [[u8; SIZE]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            height_map[y][x] = 0;
            top_map[y][x] = 0;
        }
    }
}

fn put_cube(height_map: &mut [[u8; SIZE]; SIZE], top_map: &mut [[u8; SIZE]; SIZE], mut top: u8, mut front: u8) {
    // Per spec, movement is assumed to stay within bounds; the original C code does no bounds checks.
    let mut x: usize = N_MAX;
    let mut y: usize = N_MAX;

    loop {
        let mut max_val: u8 = 0;
        let mut newfront: u8 = 0;
        let mut newtop: u8 = 0;
        let mut newx: usize = x;
        let mut newy: usize = y;

        // North (y-1): associated value = front
        if height_map[y - 1][x] < height_map[y][x] && front > 3 && front > max_val {
            newfront = top;
            newtop = back(front);
            newx = x;
            newy = y - 1;
            max_val = front;
        }

        // South (y+1): associated value = back(front)
        let b = back(front);
        if height_map[y + 1][x] < height_map[y][x] && b > 3 && b > max_val {
            newfront = bottom(top);
            newtop = front;
            newx = x;
            newy = y + 1;
            max_val = b;
        }

        // West (x-1): associated value = left(top, front)
        let l = left(top, front);
        if height_map[y][x - 1] < height_map[y][x] && l > 3 && l > max_val {
            newfront = front;
            newtop = right(top, front);
            newx = x - 1;
            newy = y;
            max_val = l;
        }

        // East (x+1): associated value = right(top, front)
        let r = right(top, front);
        if height_map[y][x + 1] < height_map[y][x] && r > 3 && r > max_val {
            newfront = front;
            newtop = left(top, front);
            newx = x + 1;
            newy = y;
            max_val = r;
        }

        if max_val == 0 {
            break;
        } else {
            front = newfront;
            top = newtop;
            x = newx;
            y = newy;
        }
    }

    height_map[y][x] = height_map[y][x].wrapping_add(1);
    top_map[y][x] = top;
}

fn count_and_format(top_map: &[[u8; SIZE]; SIZE]) -> String {
    let mut count = [0u32; 7]; // indices 0..6
    for y in 0..SIZE {
        for x in 0..SIZE {
            count[top_map[y][x] as usize] += 1;
        }
    }
    format!(
        "{} {} {} {} {} {}\n",
        count[1], count[2], count[3], count[4], count[5], count[6]
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    let mut height_map = [[0u8; SIZE]; SIZE];
    let mut top_map = [[0u8; SIZE]; SIZE];

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        init_maps(&mut height_map, &mut top_map);

        for _ in 0..n {
            let t: u8 = it.next().unwrap().parse().unwrap();
            let f: u8 = it.next().unwrap().parse().unwrap();
            put_cube(&mut height_map, &mut top_map, t, f);
        }

        out.push_str(&count_and_format(&top_map));
    }

    print!("{out}");
}
