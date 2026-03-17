const N_MAX: usize = 4;

//                 front  1  2  3  4  5  6
const RIGHT: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],   // top
    [0, 0, 3, 5, 2, 4, 0],   // 1
    [0, 4, 0, 1, 6, 0, 3],   // 2
    [0, 2, 6, 0, 0, 1, 5],   // 3
    [0, 5, 1, 0, 0, 6, 2],   // 4
    [0, 3, 0, 6, 1, 0, 4],   // 5
    [0, 0, 4, 2, 5, 3, 0],   // 6
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

struct Grid {
    height_map: [[u8; N_MAX * 2 + 1]; N_MAX * 2 + 1],
    top_map: [[u8; N_MAX * 2 + 2]; N_MAX * 2 + 1],
}

impl Grid {
    fn new() -> Self {
        Self {
            height_map: [[0; N_MAX * 2 + 1]; N_MAX * 2 + 1],
            top_map: [[0; N_MAX * 2 + 2]; N_MAX * 2 + 1],
        }
    }

    fn put(&mut self, top: u8, front: u8) {
        let mut x = N_MAX as isize;
        let mut y = N_MAX as isize;
        let mut newfront;
        let mut newtop;
        let mut newx;
        let mut newy;
        let mut max;

        loop {
            max = 0;

            // front
            if self.height_map[(y - 1) as usize][x as usize] < self.height_map[y as usize][x as usize]
                && front > 3 && front > max {
                newfront = top;
                newtop = back(front);
                newx = x;
                newy = y - 1;
                max = front;
            }

            // back
            if self.height_map[(y + 1) as usize][x as usize] < self.height_map[y as usize][x as usize]
                && back(top) > 3 && back(top) > max {
                newfront = bottom(top);
                newtop = front;
                newx = x;
                newy = y + 1;
                max = back(top);
            }

            // left
            if self.height_map[y as usize][(x - 1) as usize] < self.height_map[y as usize][x as usize]
                && left(top, front) > 3 && left(top, front) > max {
                newfront = front;
                newtop = right(top, front);
                newx = x - 1;
                newy = y;
                max = left(top, front);
            }

            // right
            if self.height_map[y as usize][(x + 1) as usize] < self.height_map[y as usize][x as usize]
                && right(top, front) > 3 && right(top, front) > max {
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

        self.height_map[y as usize][x as usize] += 1;
        self.top_map[y as usize][x as usize] = top;
    }

    fn count_and_print(&self) {
        let mut count = [0; 7];

        for y in 0..N_MAX * 2 + 1 {
            for x in 0..N_MAX * 2 + 1 {
                count[self.top_map[y][x] as usize] += 1;
            }
        }

        for x in 1..6 {
            print!("{} ", count[x]);
        }
        println!("{}", count[6]);
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut grid = Grid::new();

        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let t: u8 = parts.next().unwrap().parse().unwrap();
            let f: u8 = parts.next().unwrap().parse().unwrap();
            grid.put(t, f);
        }

        grid.count_and_print();
    }
}
