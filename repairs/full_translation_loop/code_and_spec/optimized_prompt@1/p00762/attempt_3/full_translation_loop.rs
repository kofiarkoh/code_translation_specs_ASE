use std::io::{self, BufRead};

const N_MAX: usize = 4;

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

    fn put(&mut self, mut top: u8, mut front: u8) {
        let mut x = N_MAX as isize;
        let mut y = N_MAX as isize;

        loop {
            let mut max = 0;
            let (mut new_front, mut new_top, mut new_x, mut new_y) = (front, top, x, y);

            // Check front
            if self.height_map[(y - 1) as usize][x as usize] < self.height_map[y as usize][x as usize]
                && front > 3 && front > max
            {
                new_front = top;
                new_top = back(top, front);
                new_x = x;
                new_y = y - 1;
                max = front;
            } 
            // Check back
            else if self.height_map[(y + 1) as usize][x as usize] < self.height_map[y as usize][x as usize]
                && back(top, front) > 3 && back(top, front) > max
            {
                new_front = bottom(top, front);
                new_top = front;
                new_x = x;
                new_y = y + 1;
                max = back(top, front);
            } 
            // Check left
            else if self.height_map[y as usize][(x - 1) as usize] < self.height_map[y as usize][x as usize]
                && left(top, front) > 3 && left(top, front) > max
            {
                new_front = front;
                new_top = right(top, front);
                new_x = x - 1;
                new_y = y;
                max = left(top, front);
            } 
            // Check right
            else if self.height_map[y as usize][(x + 1) as usize] < self.height_map[y as usize][x as usize]
                && right(top, front) > 3 && right(top, front) > max
            {
                new_front = front;
                new_top = left(top, front);
                new_x = x + 1;
                new_y = y;
                max = right(top, front);
            } 
            else {
                break;
            }

            front = new_front;
            top = new_top;
            x = new_x;
            y = new_y;
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

fn right(top: u8, front: u8) -> u8 {
    let right_map: [[u8; 7]; 7] = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 5, 2, 4, 0],
        [0, 4, 0, 1, 6, 0, 3],
        [0, 2, 6, 0, 0, 1, 5],
        [0, 5, 1, 0, 0, 6, 2],
        [0, 3, 0, 6, 1, 0, 4],
        [0, 0, 4, 2, 5, 3, 0],
    ];
    right_map[top as usize][front as usize]
}

fn bottom(top: u8, _front: u8) -> u8 {
    7 - top
}

fn back(_top: u8, front: u8) -> u8 {
    7 - front
}

fn left(top: u8, front: u8) -> u8 {
    7 - right(top, front)
}

fn main() {
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
            let parts: Vec<u8> = input.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (t, f) = (parts[0], parts[1]);
            grid.put(t, f);
        }

        grid.count_and_print();
    }
}
