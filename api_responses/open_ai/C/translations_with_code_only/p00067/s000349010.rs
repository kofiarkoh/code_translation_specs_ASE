use std::io::{self, BufRead};

const SIZE: usize = 16;

struct Map {
    data: [[i32; SIZE]; SIZE],
}

impl Map {
    fn new() -> Self {
        Map {
            data: [[0; SIZE]; SIZE],
        }
    }

    fn delete_island(&mut self, x: usize, y: usize) {
        if self.data[x][y] == 0 {
            return;
        }
        self.data[x][y] = 0;
        if x > 0 {
            self.delete_island(x - 1, y);
        }
        if x < 11 {
            self.delete_island(x + 1, y);
        }
        if y > 0 {
            self.delete_island(x, y - 1);
        }
        if y < 11 {
            self.delete_island(x, y + 1);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut map = Map::new();

        // Read the first line into map[0][1]
        let mut chars = line.chars();
        for j in 1..12 {
            if let Some(c) = chars.next() {
                map.data[0][j] = (c as i32) - ('0' as i32);
            }
        }

        // Read the next 11 lines into map[1][1] to map[11][1]
        for i in 1..12 {
            let line = reader.lines().next().unwrap().unwrap();
            for j in 1..12 {
                map.data[i][j] = (line.chars().nth(j - 1).unwrap() as i32) - ('0' as i32);
            }
        }

        let mut count = 0;

        // Count islands
        for i in 0..12 {
            for j in 0..12 {
                if map.data[i][j + 1] == 1 {
                    count += 1;
                    map.delete_island(i, j + 1);
                }
            }
        }

        println!("{}", count);

        // Consume the newline character after each grid
        let _ = reader.lines().next();
    }
}
