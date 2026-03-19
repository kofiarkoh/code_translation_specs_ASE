use std::io::{self, Read};

const N_MAX: usize = 4;
const SIZE: usize = N_MAX * 2 + 1; // 9

// right[top][front]
const RIGHT_TABLE: [[i8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // top
    [0, 0, 3, 5, 2, 4, 0], // 1
    [0, 4, 0, 1, 6, 0, 3], // 2
    [0, 2, 6, 0, 0, 1, 5], // 3
    [0, 5, 1, 0, 0, 6, 2], // 4
    [0, 3, 0, 6, 1, 0, 4], // 5
    [0, 0, 4, 2, 5, 3, 0], // 6
];

#[inline]
fn right(top: i8, front: i8) -> i8 {
    RIGHT_TABLE[top as usize][front as usize]
}
#[inline]
fn bottom(top: i8, _front: i8) -> i8 {
    7 - top
}
#[inline]
fn back(_top: i8, front: i8) -> i8 {
    7 - front
}
#[inline]
fn left(top: i8, front: i8) -> i8 {
    7 - right(top, front)
}

fn init_maps(height_map: &mut [[i8; SIZE]; SIZE], top_map: &mut [[i8; SIZE + 1]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            height_map[y][x] = 0;
            top_map[y][x] = 0;
        }
        // top_map has one extra column; C code doesn't explicitly clear it, but it is never read.
        top_map[y][SIZE] = 0;
    }
}

fn put(
    mut top: i8,
    mut front: i8,
    height_map: &mut [[i8; SIZE]; SIZE],
    top_map: &mut [[i8; SIZE + 1]; SIZE],
) {
    let mut x: usize = N_MAX;
    let mut y: usize = N_MAX;

    loop {
        let mut maxv: i8 = 0;
        let mut newfront: i8 = 0;
        let mut newtop: i8 = 0;
        let mut newx: usize = x;
        let mut newy: usize = y;

        // The C code indexes y-1, y+1, x-1, x+1 without bounds checks.
        // With N_MAX=4 and starting at center, the intended behavior stays within bounds.
        // We mirror that assumption and use direct indexing.

        // front (move up: y-1)
        if height_map[y - 1][x] < height_map[y][x] && front > 3 && front > maxv {
            newfront = top;
            newtop = back(top, front);
            newx = x;
            newy = y - 1;
            maxv = front;
        }

        // back (move down: y+1)
        let b = back(top, front);
        if height_map[y + 1][x] < height_map[y][x] && b > 3 && b > maxv {
            newfront = bottom(top, front);
            newtop = front;
            newx = x;
            newy = y + 1;
            maxv = b;
        }

        // left (move left: x-1)
        let l = left(top, front);
        if height_map[y][x - 1] < height_map[y][x] && l > 3 && l > maxv {
            newfront = front;
            newtop = right(top, front);
            newx = x - 1;
            newy = y;
            maxv = l;
        }

        // right (move right: x+1)
        let r = right(top, front);
        if height_map[y][x + 1] < height_map[y][x] && r > 3 && r > maxv {
            newfront = front;
            newtop = left(top, front);
            newx = x + 1;
            newy = y;
            maxv = r;
        }

        if maxv == 0 {
            break;
        } else {
            front = newfront;
            top = newtop;
            x = newx;
            y = newy;
        }
    }

    // C uses `char` for height_map; emulate 8-bit wrapping on increment.
    height_map[y][x] = height_map[y][x].wrapping_add(1);
    top_map[y][x] = top;
}

fn count_and_format(top_map: &[[i8; SIZE + 1]; SIZE]) -> String {
    let mut count = [0i32; 7];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = top_map[y][x] as usize; // values are 0..6
            count[idx] += 1;
        }
    }

    // Match C formatting: "c1 c2 c3 c4 c5 c6\n"
    let mut out = String::new();
    for x in 1..6 {
        out.push_str(&format!("{} ", count[x]));
    }
    out.push_str(&format!("{}\n", count[6]));
    out
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut height_map = [[0i8; SIZE]; SIZE];
    let mut top_map = [[0i8; SIZE + 1]; SIZE];

    let mut output = String::new();

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
            let t: i8 = it.next().unwrap().parse::<i32>().unwrap() as i8;
            let f: i8 = it.next().unwrap().parse::<i32>().unwrap() as i8;
            put(t, f, &mut height_map, &mut top_map);
        }

        output.push_str(&count_and_format(&top_map));
    }

    print!("{output}");
}
