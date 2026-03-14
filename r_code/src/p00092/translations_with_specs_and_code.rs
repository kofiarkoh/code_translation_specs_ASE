use std::io::{self, BufRead};

fn is_ok(map_ruiseki: &[Vec<usize>], y: usize, target: usize, map_size: usize) -> bool {
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
    let mut lines = reader.lines();

    loop {
        let size_line = lines.next().unwrap().unwrap();
        let map_size: usize = size_line.trim().parse().unwrap();
        if map_size == 0 {
            break;
        }

        let mut map = vec![String::new(); map_size];
        for i in 0..map_size {
            map[i] = lines.next().unwrap().unwrap();
        }

        let mut map_ruiseki = vec![vec![0; map_size]; map_size];

        for i in 0..map_size {
            let mut now = 0;
            for j in 0..map_size {
                if map[j].chars().nth(i).unwrap() == '*' {
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
                if is_ok(&map_ruiseki, i, mid, map_size) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            if left - 1 > result {
                result = left - 1;
            }
        }
        println!("{}", result);
    }
}
