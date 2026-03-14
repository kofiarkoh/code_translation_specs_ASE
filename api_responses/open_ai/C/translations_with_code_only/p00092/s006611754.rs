use std::io::{self, BufRead};

const MAX_SIZE: usize = 1000;

fn is_ok(y: usize, target: usize, map_ruiseki: &[[usize; MAX_SIZE]; MAX_SIZE], map_size: usize) -> bool {
    let mut now_renzoku = 0;
    for i in 0..map_size {
        if map_ruiseki[y][i] >= target {
            now_renzoku += 1;
        } else {
            now_renzoku = 0;
        }
        if now_renzoku >= target {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut map = [[0u8; 1001]; MAX_SIZE];
    let mut map_ruiseki = [[0usize; MAX_SIZE]; MAX_SIZE];

    for line in reader.lines() {
        let line = line.unwrap();
        let map_size: usize = line.trim().parse().unwrap();
        if map_size == 0 {
            break;
        }

        for i in 0..map_size {
            let input_line = reader.lines().next().unwrap().unwrap();
            map[i].copy_from_slice(input_line.as_bytes());
        }

        for i in 0..map_size {
            let mut now = 0;
            for j in 0..map_size {
                if map[j][i] == b'*' {
                    now = 0;
                } else {
                    now += 1;
                }
                map_ruiseki[j][i] = now;
            }
        }

        let mut result = 0;
        for i in 0..map_size {
            let mut left = result;
            let mut right = map_size;
            while left <= right {
                let mid = (left + right) / 2;
                if is_ok(i, mid, &map_ruiseki, map_size) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            if left > result + 1 {
                result = left - 1;
            }
        }
        println!("{}", result);
    }
}
