use std::io::{self, BufRead};

fn check_box(num: &[[i32; 9]; 9], flag: &mut [[bool; 9]; 9], y: usize, x: usize) {
    let mut cnt = [0; 10];

    for i in y..y + 3 {
        for j in x..x + 3 {
            cnt[num[i][j] as usize] += 1;
        }
    }
    for i in y..y + 3 {
        for j in x..x + 3 {
            if cnt[num[i][j] as usize] != 1 {
                flag[i][j] = true;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut num = [[0; 9]; 9];
        let mut flag = [[false; 9]; 9];

        for i in 0..9 {
            let line = lines.next().unwrap().unwrap();
            let entries: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for j in 0..9 {
                num[i][j] = entries[j];
            }

            let mut cnt = [0; 10];
            for j in 0..9 {
                cnt[num[i][j] as usize] += 1;
            }
            for j in 0..9 {
                if cnt[num[i][j] as usize] != 1 {
                    flag[i][j] = true;
                }
            }
        }

        for j in 0..9 {
            let mut cnt = [0; 10];
            for i in 0..9 {
                cnt[num[i][j] as usize] += 1;
            }
            for i in 0..9 {
                if cnt[num[i][j] as usize] != 1 {
                    flag[i][j] = true;
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                check_box(&num, &mut flag, i * 3, j * 3);
            }
        }

        for i in 0..9 {
            for j in 0..9 {
                print!("{}{}", if flag[i][j] { '*' } else { ' ' }, num[i][j]);
            }
            println!();
        }
        println!(); // Blank line between grids
    }
}
