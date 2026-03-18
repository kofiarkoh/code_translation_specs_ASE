use std::io::{self, Read};

const N_MAX: usize = 4;

// right[top][front]
const RIGHT_TABLE: [[i8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // top (unused)
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

struct State {
    height_map: [[i8; N_MAX * 2 + 1]; N_MAX * 2 + 1],
    top_map: [[i8; N_MAX * 2 + 2]; N_MAX * 2 + 1], // matches C's slightly wider second dim
}

impl State {
    fn new() -> Self {
        Self {
            height_map: [[0; N_MAX * 2 + 1]; N_MAX * 2 + 1],
            top_map: [[0; N_MAX * 2 + 2]; N_MAX * 2 + 1],
        }
    }

    fn init(&mut self) {
        for y in 0..(N_MAX * 2 + 1) {
            for x in 0..(N_MAX * 2 + 1) {
                self.height_map[y][x] = 0;
                self.top_map[y][x] = 0;
            }
        }
    }

    fn put(&mut self, mut top: i8, mut front: i8) {
        let mut x: usize = N_MAX;
        let mut y: usize = N_MAX;

        loop {
            let mut maxv: i8 = 0;
            let mut newfront: i8 = 0;
            let mut newtop: i8 = 0;
            let mut newx: usize = x;
            let mut newy: usize = y;

            // The C code indexes y-1, y+1, x-1, x+1 without bounds checks.
            // With N_MAX=4 and starting at center, the intended behavior stays in-bounds.
            // We preserve that assumption and use direct indexing.

            // front (move up)
            if self.height_map[y - 1][x] < self.height_map[y][x] && front > 3 && front > maxv {
                newfront = top;
                newtop = back(top, front);
                newx = x;
                newy = y - 1;
                maxv = front;
            }

            // back (move down)
            let b = back(top, front);
            if self.height_map[y + 1][x] < self.height_map[y][x] && b > 3 && b > maxv {
                newfront = bottom(top, front);
                newtop = front;
                newx = x;
                newy = y + 1;
                maxv = b;
            }

            // left (move left)
            let l = left(top, front);
            if self.height_map[y][x - 1] < self.height_map[y][x] && l > 3 && l > maxv {
                newfront = front;
                newtop = right(top, front);
                newx = x - 1;
                newy = y;
                maxv = l;
            }

            // right (move right)
            let r = right(top, front);
            if self.height_map[y][x + 1] < self.height_map[y][x] && r > 3 && r > maxv {
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

        // C uses `char` for height_map; emulate wrapping on overflow.
        self.height_map[y][x] = self.height_map[y][x].wrapping_add(1);
        self.top_map[y][x] = top;
    }

    fn count_and_print(&self, out: &mut String) {
        let mut count = [0i32; 7];
        for y in 0..(N_MAX * 2 + 1) {
            for x in 0..(N_MAX * 2 + 1) {
                let idx = self.top_map[y][x] as usize;
                count[idx] += 1;
            }
        }

        for x in 1..6 {
            out.push_str(&format!("{} ", count[x]));
        }
        out.push_str(&format!("{}\n", count[6]));
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut st = State::new();
    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        st.init();

        for _ in 0..n {
            let t: i8 = it.next().unwrap().parse::<i32>().unwrap() as i8;
            let f: i8 = it.next().unwrap().parse::<i32>().unwrap() as i8;
            st.put(t, f);
        }

        st.count_and_print(&mut out);
    }

    print!("{out}");
}
